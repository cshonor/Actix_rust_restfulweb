use sqlx::{PgPool, Row};
use crate::models::{User, CreateUser, UpdateUser};
use anyhow::Result;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user_data: CreateUser) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            user_data.name,
            user_data.email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let uuid = id.parse::<uuid::Uuid>()?;
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update_user(&self, id: &str, user_data: UpdateUser) -> Result<Option<User>> {
        let uuid = id.parse::<uuid::Uuid>()?;
        
        // 检查用户是否存在
        if !self.user_exists(id).await? {
            return Ok(None);
        }

        // 构建动态更新查询
        let mut set_clauses = Vec::new();
        let mut param_count = 1;

        if let Some(name) = &user_data.name {
            set_clauses.push(format!("name = ${}", param_count));
            param_count += 1;
        }

        if let Some(email) = &user_data.email {
            set_clauses.push(format!("email = ${}", param_count));
            param_count += 1;
        }

        if set_clauses.is_empty() {
            return self.get_user_by_id(id).await;
        }

        let query = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING *",
            set_clauses.join(", "),
            param_count
        );

        // 执行更新
        let mut query_builder = sqlx::query(&query);
        
        if let Some(name) = &user_data.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(email) = &user_data.email {
            query_builder = query_builder.bind(email);
        }
        
        let result = query_builder.bind(uuid).fetch_optional(&self.pool).await?;

        if let Some(row) = result {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_user(&self, id: &str) -> Result<bool> {
        let uuid = id.parse::<uuid::Uuid>()?;
        let result = sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn user_exists(&self, id: &str) -> Result<bool> {
        let uuid = id.parse::<uuid::Uuid>()?;
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1) as exists",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}

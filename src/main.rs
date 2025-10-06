use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod config;
mod models;
mod database;

use config::Config;
use database::Database;
use models::{UserResponse, CreateUser, UpdateUser};

// 应用状态
struct AppState {
    db: Database,
}

// 处理器函数
async fn get_users(data: web::Data<AppState>) -> Result<HttpResponse> {
    match data.db.get_all_users().await {
        Ok(users) => {
            let users_response: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
            Ok(HttpResponse::Ok().json(users_response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

async fn get_user(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match data.db.get_user_by_id(&id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

async fn create_user(
    user_data: web::Json<CreateUser>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    match data.db.create_user(user_data.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(UserResponse::from(user))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

async fn update_user(
    path: web::Path<String>,
    user_data: web::Json<UpdateUser>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match data.db.update_user(&id, user_data.into_inner()).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

async fn delete_user(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match data.db.delete_user(&id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// 健康检查端点
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "Server is running"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 设置日志
    env_logger::init();
    
    println!("🚀 Starting Actix Web RESTful Server with Database...");
    
    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");
    
    // 创建数据库连接池
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    // 运行数据库迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    
    let app_state = web::Data::new(AppState {
        db: Database::new(pool),
    });

    let server_url = format!("http://{}:{}", config.host, config.port);
    println!("📡 Server will be available at: {}", server_url);
    println!("🔗 Available endpoints:");
    println!("  GET    /health           - Health check");
    println!("  GET    /api/users        - Get all users");
    println!("  GET    /api/users/{{id}}   - Get user by ID");
    println!("  POST   /api/users        - Create new user");
    println!("  PUT    /api/users/{{id}}   - Update user");
    println!("  DELETE /api/users/{{id}}   - Delete user");
    println!("🗄️  Database: PostgreSQL with connection pooling");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .route("/users", web::get().to(get_users))
                    .route("/users", web::post().to(create_user))
                    .route("/users/{id}", web::get().to(get_user))
                    .route("/users/{id}", web::put().to(update_user))
                    .route("/users/{id}", web::delete().to(delete_user)),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

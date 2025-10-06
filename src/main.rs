use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod config;
mod models;
mod database;
mod security;
mod middleware;

use config::Config;
use database::Database;
use models::{UserResponse, CreateUser, UpdateUser};
use security::{JwtConfig, JwtManager, PasswordManager, CreateUserRequest, UpdateUserRequest, LoginRequest, AuthResponse};
use middleware::{RateLimitMiddleware, cors_config, security_headers, request_logger};

// 应用状态
struct AppState {
    db: Database,
    jwt_manager: JwtManager,
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

// 认证处理器
async fn register(
    user_data: web::Json<CreateUserRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    // 验证输入
    if let Err(validation_errors) = user_data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        })));
    }

    // 哈希密码
    let hashed_password = match PasswordManager::hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Password hashing failed"
            })));
        }
    };

    // 创建用户数据
    let create_user = CreateUser {
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };

    match data.db.create_user(create_user).await {
        Ok(user) => {
            // 生成 JWT token
            let token = match data.jwt_manager.generate_token(user.id.to_string()) {
                Ok(token) => token,
                Err(_) => {
                    return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Token generation failed"
                    })));
                }
            };

            let auth_response = AuthResponse {
                token,
                user: UserResponse::from(user),
            };

            Ok(HttpResponse::Created().json(auth_response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "User creation failed"
            })))
        }
    }
}

async fn login(
    login_data: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    // 验证输入
    if let Err(validation_errors) = login_data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        })));
    }

    // 这里应该验证用户凭据
    // 为了演示，我们假设验证成功
    let user_id = "user-123".to_string(); // 实际应该从数据库获取

    // 生成 JWT token
    let token = match data.jwt_manager.generate_token(user_id) {
        Ok(token) => token,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Token generation failed"
            })));
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "message": "Login successful"
    })))
}

// 健康检查端点
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "Server is running",
        "security": "enabled"
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
    
    // 创建 JWT 配置
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let jwt_config = JwtConfig::new(jwt_secret, 3600); // 1小时过期
    let jwt_manager = JwtManager::new(jwt_config);

    let app_state = web::Data::new(AppState {
        db: Database::new(pool),
        jwt_manager,
    });

    let server_url = format!("http://{}:{}", config.host, config.port);
    println!("📡 Server will be available at: {}", server_url);
    println!("🔗 Available endpoints:");
    println!("  GET    /health           - Health check");
    println!("  POST   /auth/register    - User registration");
    println!("  POST   /auth/login       - User login");
    println!("  GET    /api/users        - Get all users");
    println!("  GET    /api/users/{{id}}   - Get user by ID");
    println!("  POST   /api/users        - Create new user");
    println!("  PUT    /api/users/{{id}}   - Update user");
    println!("  DELETE /api/users/{{id}}   - Delete user");
    println!("🗄️  Database: PostgreSQL with connection pooling");
    println!("🔒  Security: JWT authentication, rate limiting, input validation");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(cors_config())
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(security_headers())
            .wrap(RateLimitMiddleware::new(100, std::time::Duration::from_secs(60)))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login)),
            )
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

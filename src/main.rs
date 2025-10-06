use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

// 应用状态
struct AppState {
    users: Mutex<HashMap<String, User>>,
}

// 处理器函数
async fn get_users(data: web::Data<AppState>) -> Result<HttpResponse> {
    let users = data.users.lock().unwrap();
    let users_list: Vec<User> = users.values().cloned().collect();
    Ok(HttpResponse::Ok().json(users_list))
}

async fn get_user(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let users = data.users.lock().unwrap();
    match users.get(&path.into_inner()) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}

async fn create_user(
    user_data: web::Json<CreateUser>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = Uuid::new_v4().to_string();
    let user = User {
        id: id.clone(),
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    let mut users = data.users.lock().unwrap();
    users.insert(id.clone(), user.clone());
    
    Ok(HttpResponse::Created().json(user))
}

async fn update_user(
    path: web::Path<String>,
    user_data: web::Json<UpdateUser>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    match users.get_mut(&id) {
        Some(user) => {
            if let Some(name) = &user_data.name {
                user.name = name.clone();
            }
            if let Some(email) = &user_data.email {
                user.email = email.clone();
            }
            Ok(HttpResponse::Ok().json(user.clone()))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}

async fn delete_user(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    match users.remove(&id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
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
    println!("🚀 Starting Actix Web RESTful Server...");
    
    let app_state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    println!("📡 Server will be available at: http://localhost:8081");
    println!("🔗 Available endpoints:");
    println!("  GET    /health           - Health check");
    println!("  GET    /api/users        - Get all users");
    println!("  GET    /api/users/{{id}}   - Get user by ID");
    println!("  POST   /api/users        - Create new user");
    println!("  PUT    /api/users/{{id}}   - Update user");
    println!("  DELETE /api/users/{{id}}   - Delete user");

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
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

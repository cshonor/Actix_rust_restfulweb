use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use validator::Validate;

// JWT 配置
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64, // 秒
}

impl JwtConfig {
    pub fn new(secret: String, expiration: u64) -> Self {
        Self { secret, expiration }
    }
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
    pub iss: String, // 签发者
}

impl Claims {
    pub fn new(user_id: String, expiration: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        Self {
            sub: user_id,
            exp: now + expiration as usize,
            iat: now,
            iss: "actix-rust-api".to_string(),
        }
    }
}

// 密码哈希
pub struct PasswordManager;

impl PasswordManager {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

// JWT 管理器
pub struct JwtManager {
    config: JwtConfig,
}

impl JwtManager {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, self.config.expiration);
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &EncodingKey::from_secret(self.config.secret.as_ref()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

// 输入验证
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 100, message = "姓名长度必须在2-100个字符之间"))]
    pub name: String,
    
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 255, message = "邮箱长度不能超过255个字符"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 128, message = "密码长度必须在8-128个字符之间"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 100, message = "姓名长度必须在2-100个字符之间"))]
    pub name: Option<String>,
    
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 255, message = "邮箱长度不能超过255个字符"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 128, message = "密码长度必须在8-128个字符之间"))]
    pub password: String,
}

// 安全响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

// 错误类型
#[derive(Debug, Serialize)]
pub struct SecurityError {
    pub error: String,
    pub message: String,
}

impl SecurityError {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}

// 安全中间件
use actix_web::{HttpRequest, HttpResponse, Result, web};
use actix_web::dev::ServiceRequest;
use actix_web::Error;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: actix_web::dev::Next<actix_web::dev::Payload>,
) -> Result<actix_web::dev::ServiceResponse<actix_web::dev::Payload>, Error> {
    // 从请求头中提取 JWT token
    let auth_header = req.headers().get("Authorization");
    
    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {
            if header_str.starts_with("Bearer ") {
                let token = &header_str[7..];
                // 这里可以验证 token
                // 如果验证失败，返回 401 错误
            }
        }
    }
    
    next.call(req).await
}

// 安全工具函数
pub fn sanitize_input(input: &str) -> String {
    // 移除潜在的恶意字符
    input
        .chars()
        .filter(|c| !matches!(c, '<' | '>' | '"' | '\'' | '&' | ';' | '(' | ')' | '[' | ']' | '{' | '}'))
        .collect()
}

pub fn validate_email(email: &str) -> bool {
    validator::validate_email(email)
}

pub fn validate_password_strength(password: &str) -> bool {
    // 密码强度验证：至少8位，包含大小写字母、数字和特殊字符
    password.len() >= 8
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_digit())
        && password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
}

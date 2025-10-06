use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    time::Duration,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::IpAddr;

// 速率限制中间件
pub struct RateLimitMiddleware {
    max_requests: u32,
    window_duration: Duration,
    store: Arc<RwLock<HashMap<IpAddr, (u32, std::time::Instant)>>>,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitService {
            service: Rc::new(service),
            max_requests: self.max_requests,
            window_duration: self.window_duration,
            store: self.store.clone(),
        }))
    }
}

pub struct RateLimitService<S> {
    service: Rc<S>,
    max_requests: u32,
    window_duration: Duration,
    store: Arc<RwLock<HashMap<IpAddr, (u32, std::time::Instant)>>>,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let max_requests = self.max_requests;
        let window_duration = self.window_duration;
        let store = self.store.clone();

        Box::pin(async move {
            // 获取客户端IP
            let client_ip = req
                .connection_info()
                .remote_addr()
                .and_then(|addr| addr.parse::<IpAddr>().ok())
                .unwrap_or_else(|| "127.0.0.1".parse().unwrap());

            let now = std::time::Instant::now();
            let mut store = store.write().await;

            // 检查速率限制
            if let Some((count, window_start)) = store.get(&client_ip) {
                if now.duration_since(*window_start) < window_duration {
                    if *count >= max_requests {
                        return Ok(ServiceResponse::new(
                            req.into_parts().0,
                            HttpResponse::TooManyRequests()
                                .json(serde_json::json!({
                                    "error": "Rate limit exceeded",
                                    "message": "Too many requests, please try again later"
                                })),
                        ));
                    }
                } else {
                    // 窗口过期，重置计数
                    store.insert(client_ip, (1, now));
                }
            } else {
                // 新客户端
                store.insert(client_ip, (1, now));
            }

            // 清理过期的条目
            store.retain(|_, (_, window_start)| {
                now.duration_since(*window_start) < window_duration
            });

            // 调用下一个服务
            service.call(req).await
        })
    }
}

// CORS 配置
pub fn cors_config() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin("http://localhost:3000") // 前端地址
        .allowed_origin("http://localhost:8080") // 管理界面
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            "Content-Type",
            "Authorization",
            "X-Requested-With",
            "Accept",
            "Origin",
        ])
        .max_age(3600)
}

// 安全头中间件
pub fn security_headers() -> impl Fn(&HttpRequest, &mut actix_web::dev::ServiceResponse) -> Result<(), Error> {
    move |_req: &HttpRequest, res: &mut actix_web::dev::ServiceResponse| {
        let headers = res.headers_mut();
        
        // 防止点击劫持
        headers.insert(
            "X-Frame-Options",
            actix_web::http::HeaderValue::from_static("DENY"),
        );
        
        // 防止MIME类型嗅探
        headers.insert(
            "X-Content-Type-Options",
            actix_web::http::HeaderValue::from_static("nosniff"),
        );
        
        // XSS保护
        headers.insert(
            "X-XSS-Protection",
            actix_web::http::HeaderValue::from_static("1; mode=block"),
        );
        
        // 严格传输安全
        headers.insert(
            "Strict-Transport-Security",
            actix_web::http::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        );
        
        // 内容安全策略
        headers.insert(
            "Content-Security-Policy",
            actix_web::http::HeaderValue::from_static("default-src 'self'"),
        );
        
        // 引用者策略
        headers.insert(
            "Referrer-Policy",
            actix_web::http::HeaderValue::from_static("strict-origin-when-cross-origin"),
        );
        
        // 权限策略
        headers.insert(
            "Permissions-Policy",
            actix_web::http::HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
        );

        Ok(())
    }
}

// 请求日志中间件
pub async fn request_logger(req: &HttpRequest) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let method = req.method();
    let path = req.path();
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");
    
    let client_ip = req
        .connection_info()
        .remote_addr()
        .unwrap_or("Unknown");

    log::info!(
        "Request: {} {} from {} (User-Agent: {})",
        method,
        path,
        client_ip,
        user_agent
    );

    Ok(())
}

// 输入验证中间件
pub fn validate_json<T: serde::de::DeserializeOwned + validator::Validate>(
) -> impl Fn(actix_web::web::Json<T>) -> Result<actix_web::web::Json<T>, actix_web::Error> {
    move |json: actix_web::web::Json<T>| -> Result<actix_web::web::Json<T>, actix_web::Error> {
        match json.validate() {
            Ok(_) => Ok(json),
            Err(errors) => {
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(_, errors)| errors)
                    .map(|e| e.message.clone().unwrap_or_else(|| "Invalid input".to_string()))
                    .collect();
                
                Err(actix_web::error::ErrorBadRequest(serde_json::json!({
                    "error": "Validation failed",
                    "details": error_messages
                })))
            }
        }
    }
}

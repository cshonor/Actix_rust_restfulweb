use actix_web::{web, App, HttpServer, Responder, HttpRequest};

 #[tokio::main]
 async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(greet))
    .route("/{name}", web::get().to(greet))
    .route("/health", web::get().to(health_check))
   })
   .bind("127.0.0.1:8080")
   .unwrap()
   .run()
   .await;
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}


async fn health_check(req: HttpRequest) -> impl Responder {
HttpResponse::Ok()
}
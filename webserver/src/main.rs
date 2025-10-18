use actix_web::{web, App, HttpServer, Responder, HttpRequest};

 #[tokio::main]
 async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(greet))
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
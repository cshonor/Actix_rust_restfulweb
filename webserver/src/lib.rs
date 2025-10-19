use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}


pub async fn health_check(req: HttpRequest) -> impl Responder {
HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
       
    create_app().await})
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}

pub  async fn create_app() -> App {
    App::new()
.route("/", web::get().to(greet))
.route("/{name}", web::get().to(greet))
.route("/health", web::get().to(health_check))
}
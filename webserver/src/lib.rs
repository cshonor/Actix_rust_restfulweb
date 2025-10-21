use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}


pub async fn health_check(req: HttpRequest) -> impl Responder {
HttpResponse::Ok()
}

pub  fn run() -> Result<Server, std::io::Error> {
   let server = HttpServer::new(|| {
       
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health", web::get().to(health_check))})
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}
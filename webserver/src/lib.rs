use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

pub async fn subscribe(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
pub async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
   let server = HttpServer::new(|| {
       
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health", web::get().to(health_check))})
        .route("/subscribe", web::post().to(subscribe))
    .listen(listener)?
    .run();
    Ok(server)
}
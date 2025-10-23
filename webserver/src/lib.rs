use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

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
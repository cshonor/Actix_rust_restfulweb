use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;
use webserver::configuration::get_configuration;
pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let settings = get_configuration().expect("Failed to load configuration");
    let address = format!("127.0.0.1:{}", settings.application_port);
    
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
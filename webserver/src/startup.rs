use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use actix_web::middleware;  
use std::net::TcpListener;
use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscribe};
use sqlx::PgPool;

pub  fn run(listener: TcpListener, db_pool:PgPool) -> Result<Server, std::io::Error> {

        let settings = get_configuration().expect("Failed to load configuration");
        let address = format!("127.0.0.1:{}", settings.application_port);
        let db_pool = web::Data::new(db_pool);
        let server = HttpServer::new(|| {  
         App::new()
         .wrap(middleware::Logger::default())
         .route("/", web::get().to(greet))  
         .route("/{name}", web::get().to(greet))
         .route("/health", web::get().to(health_check))
         .route("/subscribe", web::post().to(subscribe)).app_data(db_pool.clone())})
        
     .listen(listener)?
     .run();
     Ok(server)
 }
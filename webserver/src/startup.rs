use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscribe};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub  fn run(listener: TcpListener, db_pool:PgPool) -> Result<Server, std::io::Error> {
        let settings = get_configuration().expect("Failed to load configuration");
        let db_pool = web::Data::new(db_pool);
        let server = HttpServer::new(move || {  
         App::new()
         .wrap(TracingLogger::default())
         .route("/", web::get().to(greet))  
         .route("/{name}", web::get().to(greet))
         .route("/health", web::get().to(health_check))
         .route("/subscribe", web::post().to(subscribe)).app_data(db_pool.clone())})
        
     .listen(listener)?
     .run();
     Ok(server)
 }
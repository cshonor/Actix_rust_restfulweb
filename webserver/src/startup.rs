use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use actix_web::middleware;  
use std::net::TcpListener;
use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscribe};
use sqlx::PgPool;
use tracing_subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Registry, EnvFilter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_actix_web::TracingLogger;


pub  fn run(listener: TcpListener, db_pool:PgPool) -> Result<Server, std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("webserver".into(), std::io::stdout);
    let subscriber = Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
        let settings = get_configuration().expect("Failed to load configuration");
        let address = format!("127.0.0.1:{}", settings.application_port);
        let db_pool = web::Data::new(db_pool);
        let server = HttpServer::new(|| {  
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
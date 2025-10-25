use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use actix_web::middleware;  
use std::net::TcpListener;
use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscribe};
use sqlx::PgPool;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Registry, EnvFilter};

fn get_subscriber(name: String, env_filter: String) -> Subscriber {
   
    let env_filter = EnvFilter::new(env_filter);
   let registry = Registry::default().with(env_filter).with(formatting_layer);
    let formatting_layer = fmt::layer().with_target(true).with_line_number(true).with_file(true);
    Subscriber::builder().with_subscriber(env_filter).with_ansi(true).with_timer(ChronoTimer::new()).with_file(true).with_line_number(true).with_target(true).finish()
}   
pub  fn run(listener: TcpListener, db_pool:PgPool) -> Result<Server, std::io::Error> {
let subscriber = get_subscriber("webserver".into(), "info".into(), std::io::stdout);
let _ = subscriber.init();
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
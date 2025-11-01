use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscribe};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::domain::email_client::EmailClient;

pub  fn run(listener: TcpListener, db_pool:PgPool, email_client: EmailClient) -> Result<Server, std::io::Error> {
        let _settings = get_configuration().expect("Failed to load configuration");
        //web::Data::new 用于在 actix-web 中注册共享的应用状态，让所有请求处理器都能访问同一个数据实例。
        //db_pool 和 email_client 是两个不同的数据实例，但是它们都存储在 web::Data 中，
        //这样就可以让所有请求处理器都能访问同一个数据实例。
        let db_pool = web::Data::new(db_pool);
        let email_client = web::Data::new(email_client);
        let server = HttpServer::new(move || {  
         App::new()
         .wrap(TracingLogger::default())
         .route("/", web::get().to(greet))  
         .route("/{name}", web::get().to(greet))
         .route("/health", web::get().to(health_check))
         .route("/subscribe", web::post().to(subscribe))
         //app_data 用于在 actix-web 中注册共享的应用状态，让所有请求处理器都能访问同一个数据实例。
         //clone() 仅克隆 Arc，数据本身不会被复制
         //处理器中自动注入（subscribe.rs） web::Data<PgPool>  web::Data<EmailClient>  // actix-web 自动注入
         .app_data(db_pool.clone())
         .app_data(email_client.clone())})
        
     .listen(listener)?
     .run();
     Ok(server)
 }
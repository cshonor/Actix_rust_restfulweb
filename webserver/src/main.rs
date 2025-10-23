use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use crate::startup::run;
use crate::configuration::get_configuration;
use sqlx::{PgConnection, Connection};
use std::net::TcpListener;

 #[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings=get_configuration().expect("Failed to get configuration");
    let connection_pool=PgPoolOptions::new().connect_lazy_with(settings.database.connection_string());
    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
    run(listener, connection_pool).await?;
    Ok(())
}


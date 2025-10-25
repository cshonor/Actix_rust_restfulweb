use crate::startup::run;
use crate::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
use tracing_subscriber::{EnvFilter};
use crate::routes::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("webserver".into(), EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));
    init_subscriber(subscriber);

    let settings=get_configuration().expect("Failed to get configuration");
    let db_pool=PgPool::connect(&settings.database.connection_string()).await.expect("Failed to connect to Postgres");
    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
    run(listener, db_pool)?.await?;
    Ok(())
}


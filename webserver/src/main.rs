use webserver::startup::run;
use webserver::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
use tracing_subscriber::{EnvFilter};
use webserver::routes::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("webserver".into(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let settings=get_configuration().expect("Failed to get configuration");
    let db_pool=PgPool::connect_lazy_with(settings.database.with_db().build().expect("Failed to create connection pool"));
    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
    run(listener, db_pool)?.await?;
    Ok(())
}


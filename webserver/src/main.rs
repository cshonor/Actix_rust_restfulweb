use webserver::startup::run;
use webserver::configuration::get_configuration;
use sqlx::{PgConnection};
use std::net::TcpListener;
use tracing_subscriber::{EnvFilter};
use webserver::routes::telemetry::{get_subscriber, init_subscriber};
use crate::domain::email_client::EmailClient;
use reqwest::Client;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("webserver".into(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let settings=get_configuration().expect("Failed to get configuration");

    let db_pool=PgConnectOptions::connect_lazy_with(settings.database.with_db());

    let email_client_settings = settings.email_client.sender().expect("Invalid sender email");

    let email_client = EmailClient::new(email_client_settings, Client::new(), settings.email_client.base_url, settings.email_client.authorization_token);

    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
   
    run(listener, db_pool, email_client)?.await?;
    Ok(())
}



use webserver::startup::run;
use webserver::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Registry, EnvFilter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

 #[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化 tracing 日志系统
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("webserver".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    subscriber.init();

    let settings=get_configuration().expect("Failed to get configuration");
    let db_pool=PgPool::connect(&settings.database.connection_string()).await.expect("Failed to connect to Postgres");
    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
    run(listener, db_pool)?.await?;
    Ok(())
}



use crate::startup::run;
use crate::configuration::get_configuration;
use sqlx::{PgPool};
use std::net::TcpListener;

 #[tokio::main]
async fn main() -> std::io::Result<()> {

    let settings=get_configuration().expect("Failed to get configuration");
    let db_pool=PgPool::connect(&settings.database.connection_string()).await.expect("Failed to connect to Postgres");
    let listener=TcpListener::bind(format!("{}:{}", settings.application.host, settings.application.port)).expect("Failed to bind port");
    run(listener, db_pool).await?;
    Ok(())
}


use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use webserver::startup::run;
 #[tokio::main]
 async fn main() -> std::io::Result<()> {

   run().await?;
   
}


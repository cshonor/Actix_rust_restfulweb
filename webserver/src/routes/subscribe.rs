use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(_req: HttpRequest, form: web::Form<Subscriber>,db_pool: web::Data<PgPool>) -> impl Responder {
    
    let request_id = Uuid::new_v4();
    let span = tracing::info_span!("subscribe",
        request_id = %request_id,
        email = %form.email,
        name = %form.name
    );

    let _enter = span.enter();
    tracing::info!(" request_id: {} request body: {:?}", request_id, form);


    let db_span = tracing::info_span!("subscribe_db");
    match sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , request_id, form.email, form.name, chrono::Utc::now())
    .execute(db_pool.get_ref()).instrument(db_span).await
    {
        Ok(_)=>{
            tracing::info!(request_id = %request_id, "Subscription successful"); 
            HttpResponse::Ok().finish()
        },
        Err(e)=>{
            tracing::error!(request_id = %request_id, "Failed to subscribe: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(req: HttpRequest, form: web::Form<Subscriber>,db_pool: web::Data<PgPool>) -> impl Responder {
    log::info!("request body: {:?}", form); 
    let result = sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , Uuid::new_v4(), form.email, form.name, chrono::Utc::now())
    .execute(db_pool.get_ref()).await?;
    HttpResponse::Ok().finish()
}
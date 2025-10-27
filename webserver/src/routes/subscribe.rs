use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form, db_pool),
    fields(email = %form.email,name = %form.name))]
pub async fn subscribe(_req: HttpRequest, form: web::Form<Subscriber>,db_pool: web::Data<PgPool>) -> impl Responder {
    
    match insert_subscriber(&db_pool, &form).await
    {
        Ok(_)=>{
            HttpResponse::Ok().finish()
        },
        Err(_)=>{
            HttpResponse::InternalServerError().finish()
        }
    }
}
#[tracing::instrument(name = "Inserting a new subscriber", 
skip(form, db_pool),
fields(email = %form.email,name = %form.name))]

pub async fn insert_subscriber(db_pool: &PgPool, form:&Subscriber) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , Uuid::new_v4(), form.email, form.name, chrono::Utc::now())
    .execute(db_pool).await
    .map_err(|e| {
        tracing::error!("Failed to insert subscriber: {}", e);
        e
    })?;
    Ok(())
}
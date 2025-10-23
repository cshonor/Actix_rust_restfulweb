use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgConnection;
use uuid::Uuid;
use chrono::Utc;
#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(req: HttpRequest, form: web::Form<Subscriber>,connection: web::Data<PgConnection>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , Uuid::new_v4(), form.email, form.name, chrono::Utc::now())
    .execute(connection.get_ref())
    .await?;
    if result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
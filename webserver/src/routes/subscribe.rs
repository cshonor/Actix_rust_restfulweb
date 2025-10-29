use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;  

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
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().body("Invalid name").finish();
    }
    if !is_valid_email(&form.email) {
        return HttpResponse::BadRequest().body("Invalid email").finish();
    }
    match insert_subscriber(&db_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

pub fn is_valid_name(name: &str) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();
    let is_too_long = name.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
pub fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    email.contains('@') && email.ends_with(".com")
}
use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;  
use crate::domain::NewSubscriber;
use crate::domain::SubscriberName;

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
    let new_subscriber = NewSubscriber {
        //form.0：form是一个web::Form<Subscriber>类型的实例，它是一个元组结构体。
        //正因为它是元组结构体，且内部只有一个字段（索引为0），所以必须通过.0来访问其内部真正的T实例（这里就是Subscriber）
        email: form.0.email,
        name: SubscriberName::parse(form.0.name).expect("Failed to parse subscriber name"),
    };
    insert_subscriber(&db_pool, &new_subscriber).await.map_err(|_| HttpResponse::InternalServerError().finish())?;
    HttpResponse::Ok().finish()
}
#[tracing::instrument(name = "Inserting a new subscriber", 
skip(form, db_pool),
fields(email = %form.email,name = %form.name))]

pub async fn insert_subscriber(db_pool: &PgPool, form:&NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , Uuid::new_v4(), form.email, form.name.as_ref(), chrono::Utc::now())
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
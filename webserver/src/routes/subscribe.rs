use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;  
use crate::domain::NewSubscriber;
use crate::domain::{SubscriberName, SubscriberEmail};

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

pub fn parse_subscriber(form: web::Form<Subscriber>) -> Result<NewSubscriber, String> {
    let name= match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return Err("Invalid name".to_string()),
    };
    let email= match SubscriberEmail::parse(form.0.email) {
        Ok(email) => email,
        Err(_) => return Err("Invalid email".to_string()),
    };
    Ok(NewSubscriber {email, name})
}   
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form, db_pool),
    fields(email = %form.email,name = %form.name))]
pub async fn subscribe(_req: HttpRequest, form: web::Form<Subscriber>,db_pool: web::Data<PgPool>) -> impl Responder {
    let new_subscriber =  match parse_subscriber(form) {
        Ok(new_subscriber) => new_subscriber,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };
    match insert_subscriber(&db_pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[tracing::instrument(name = "Inserting a new subscriber", 
skip(form, db_pool))]

pub async fn insert_subscriber(db_pool: &PgPool, form:&NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO subscriptions (id,email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    , Uuid::new_v4(), form.email.as_ref(), form.name.as_ref(), chrono::Utc::now())
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
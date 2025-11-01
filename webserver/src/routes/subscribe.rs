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
#[tracing::instrument(name = "Converting a subscriber to a new subscriber", skip(subscriber))]
impl TryFrom<Subscriber> for NewSubscriber {
    type Error = String;
    fn try_from(subscriber: Subscriber) -> Result<Self, Self::Error> {
        let name= match SubscriberName::parse(subscriber.name) {
            Ok(name) => name,
            Err(_) => return Err("Invalid name".to_string()),
        };
        let email= match SubscriberEmail::parse(subscriber.email) {
            Ok(email) => email,
            Err(_) => return Err("Invalid email".to_string()),
        };
        Ok(NewSubscriber {email, name})
    }
}

#[tracing::instrument(name = "Parsing a new subscriber", skip(form))]
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
    let new_subscriber =  match form.0.try_into() {
        Ok(subscriber) => subscriber,
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


use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
#[derive(Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(req: HttpRequest, form: web::Form<Subscriber>) -> impl Responder {
    
    HttpResponse::Ok().finish()
}
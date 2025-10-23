use actix_web::{web, HttpRequest, HttpResponse, Responder};
pub async fn health_check(req: HttpRequest) -> impl Responder {
    println!("health_check");
    HttpResponse::Ok().finish()
}
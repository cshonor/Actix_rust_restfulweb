use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

pub async fn health_check(req: HttpRequest) -> impl Responder {
    let request_id = Uuid::new_v4();
    tracing::info!(" request_id: {} request body: {:?}", request_id, req);
    HttpResponse::Ok().finish()
}
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
pub async fn greet(req: HttpRequest) -> impl Responder {
    let request_id = Uuid::new_v4();
    let name = req.match_info().get("name").unwrap_or("World");
    let response = format!("Hello, {}!", name);
    tracing::info!(" request_id: {} request body: {:?}", request_id, response);
    HttpResponse::Ok().body(response)
}
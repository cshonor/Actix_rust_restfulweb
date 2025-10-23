use actix_web::{web, HttpRequest, HttpResponse, Responder};
pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}
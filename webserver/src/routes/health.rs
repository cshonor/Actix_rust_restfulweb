pub async fn health_check(req: HttpRequest) -> impl Responder {
    
    HttpResponse::Ok().finish()
}
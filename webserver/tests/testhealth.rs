use webserver::create_app;
// tests/testhealth.rs

use actix_web::{test, web, App};

#[actix_web::test]
async fn test_health_check() {
    let app = create_app();  // ✅ 可以访问！
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
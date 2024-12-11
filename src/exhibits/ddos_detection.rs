use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().body("DDoS Test")
}
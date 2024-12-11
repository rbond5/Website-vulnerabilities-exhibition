use actix_web::{get, HttpResponse, Responder};

pub async fn firewall_handle() -> impl Responder {
    HttpResponse::Ok().body("This is a placeholder") // The firewall exhibit is backend. Eventually, I want to display firewall code on this page.
}
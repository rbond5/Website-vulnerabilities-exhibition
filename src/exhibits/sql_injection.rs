use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::MySql;
use serde;

async fn sqli_form() -> impl Responder {
    let login_page = "login_page.html"; // this will become a path to the login page
    HttpResponse::Ok().content_type("text/html").body(login_page)
}


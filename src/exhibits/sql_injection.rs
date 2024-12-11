use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::MySql;
use serde;

async fn sqli_form() -> impl Responder {
    let login_page = "login_page.html"; // this will become a path to the login page
    HttpResponse::Ok().content_type("text/html").body(login_page)
}

#[derive(serde::Deserialize)]
struct login_form {
    username: String,
    password: String,
}

async fn sqli_check(form: web::Form<login_form>) -> impl Responder {
    let injection_phrase = " 'OR 1=1"; // Until a database is implemented, this will spit the flag to the user in the same way
    if form.username.trim() == injection_phrase || form.password.trim() == injection_phrase {
        HttpResponse::Ok().body("You have succesfully performed a basic SQL injection! Flag[rules_are_just_words]") //This will spit onto login page when implemented
    }
    else {
        HttpResponse::Ok().body("Try again!") //This will spit onto login page when implemented
    }
}

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::MySql;
use serde;
use std::fs;

pub async fn sqli_handle() -> impl Responder {
    let login_page = fs::read_to_string("login_page.html").expect("Something went wrong reading the file"); // This will become a path to the login page
    HttpResponse::Ok().content_type("text/html").body(login_page)
}

//We store the login information in a struct for easier use
#[derive(serde::Deserialize)]
pub struct login_form {
    username: String,
    password: String,
}

pub async fn sqli_check(form: web::Form<login_form>) -> impl Responder {
    let injection_phrase = " 'OR 1=1"; // Until a database is implemented, this will spit the flag to the user in the same way
    if form.username.trim() == injection_phrase || form.password.trim() == injection_phrase {
        HttpResponse::Ok().body("You have succesfully performed a basic SQL injection! Flag[rules_are_just_words]") //This will spit onto login page when implemented
    }
    else {
        HttpResponse::Ok().body("Try again!") //This will spit onto login page when implemented
    }
}

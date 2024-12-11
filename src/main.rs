use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

mod db;
mod middleware;
mod exhibits;

fn main() {
    println!("Hello, world!");
} 

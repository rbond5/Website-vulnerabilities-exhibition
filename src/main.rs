//#![feature(collections)]

use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

mod db;
mod middleware;
mod exhibits;
mod utilities;

fn main() {
    println!("Hello, world!");
} 

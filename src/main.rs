use actix_web::{App, HttpServer, HttpResponse, web, middleware::Logger};
use dotenv::dotenv;
//use std::env;
use utilities::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    env,
};

mod db;
mod tools;
mod exhibits;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Loading environment variables (Database, IP, Port)
    env_logger::init(); // Ensure logs from Logger middleware are displayed to monitor application

    // Loading host IP and Port from .env file. If it fails, set to localhost IP and standard backup http port
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    // Log message to confirm server is running
    println!("Starting Actix Web server at http://{}/", bind_address);

    // Placeholder webserver for development
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // Home page placeholder - eventually this will link to exhibits.
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World") }))
            // Placeholder routes for the three exhibits
            .route("/login", web::get().to(|| async { HttpResponse::Ok().body("Login Placeholder") }))
            .route("/ddos", web::get().to(|| async { HttpResponse::Ok().body("DDoS Placeholder") }))
            .route("/firewall", web::get().to(|| async { HttpResponse::Ok().body("Firewall Placeholder") }))
    })
    .bind(bind_address)?
    .run()
    .await
}

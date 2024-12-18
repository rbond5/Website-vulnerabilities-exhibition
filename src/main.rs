use actix_web::{App, HttpServer, HttpResponse, web, middleware::Logger};
use actix_web::http::header;
use actix_web::dev::{Service, Transform};
use actix_web::HttpMessage;
use dotenv::dotenv;
use futures_util::future::{ok, Ready};
use env_logger;
use std::{
    env,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use std::task::{Context, Poll};

// Local modules (make sure they exist as `src/exhibits/mod.rs` and `src/tools/mod.rs` or files)
mod tools;
mod exhibits;
mod middleware;

// Using local structs/functions from your modules
use crate::tools::firewall::Firewall;
use crate::tools::rate_limit::RateLimiter;
use crate::middleware::MyMiddleware;

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

    //Create a new firewall + rate limiter instance
    let firewall = Arc::new(Firewall::new_firewall_ruleset());
    let rate_limiter = Arc::new(RateLimiter::new_rate_limiter(firewall.clone()));

    // Placeholder webserver for development
    HttpServer::new(move || {
        App::new()
            .app_data(firewall.clone())
            .app_data(rate_limiter.clone())
            .wrap(MyMiddleware::new(firewall.clone(), rate_limiter.clone()))
            //.wrap(Logger::default())

            // Home page placeholder - eventually this will load homepage
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World") }))
            // Routes for the three exhibits
            .service(web::resource("/SQLi").route(web::get().to(exhibits::sql_injection::sqli_handle)).route(web::post().to(exhibits::sql_injection::sqli_check)))
            .service(web::resource("/DOS").route(web::get().to(exhibits::dos_detection::dos_handle)))
            .service(web::resource("/Firewall").route(web::get().to(exhibits::firewall::firewall_handle)))
    })
    .bind(bind_address)?
    .run()
    .await
}

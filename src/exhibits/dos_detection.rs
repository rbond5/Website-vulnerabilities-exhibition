use actix_web::{get, HttpRequest, HttpResponse, Responder};
use crate::tools::rate_limit;

pub async fn dos_handle(req: HttpRequest) -> impl Responder {
    // We store the attacker IP as the one that made the request because attacks will be coming from the user
    let attacker_ip = req.connection_info().realip_remote_addr().unwrap_or("Unknown").to_string();

    //dos_detected is a funtion that will check if a dos attempt is coming from attacker_ip
    if dos_detected(&attacker_ip) {
        HttpResponse::Ok().body("Denial of Service attempt detected. Flag[You've_got_mail]")
    }
    else{
        HttpResponse::Ok().body("Try again!")
    }
    
}
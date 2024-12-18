use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
//use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

//Struct to store firewall rules. for now, just IP and ports
pub struct Firewall {
    //Arc<> Multithread ownership of variables
    //RwLock<> is used because there will be a need for multiple threads to read the data at once
    //HashSet<String> Stores IP addresses as strings for fast lookup
    blocked_IP: Arc<RwLock<HashSet<String>>>,  
    blocked_ports: Arc<RwLock<HashSet<u16>>>, //port binary is 16 bit
}


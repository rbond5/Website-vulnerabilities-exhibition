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
    blocked_ip: Arc<RwLock<HashSet<String>>>,  
    blocked_ports: Arc<RwLock<HashSet<u16>>>, //port binary is 16 bit
}

impl Firewall {

    //Constructor creates a new struct with empty hashmaps
    pub fn new_firewall_ruleset() -> Self {
        Firewall {
            blocked_ip: Arc::new(RwLock::new(HashSet::new())),
            blocked_ports: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    //Add a given IP String to ruleset
    pub fn block_new_ip(&self, address: &SocketAddr) {
        let ip_string = address.ip().to_string();
        {
            let mut ip_table = self.blocked_ip.write().unwrap();
            ip_table.insert(ip_string);
        }
    }

    //Remove a given IP String from ruleset
    pub fn unblock_ip(&self, address: &SocketAddr) {
        let ip_string = address.ip().to_string();
        {
            let mut ip_table = self.blocked_ip.write().unwrap();
            ip_table.remove(&ip_string);
        }
    }

    //Add a given port number to ruleset
    pub fn block_new_port(&self, address: &SocketAddr) {
        let ports = address.port();
        {
            let mut port_table = self.blocked_ports.write().unwrap();
            port_table.insert(ports);
        }
    }

    //Remove a given port number from ruleset
    pub fn unblock_port(&self, address: &SocketAddr) {
        let ports = address.port();
        {
            let mut port_table = self.blocked_ports.write().unwrap();
            port_table.remove(&ports);
        }
    }

    //Check a given address against values in ruleset
    //Return false if found, true otherwise
    pub fn check_if_allowed(&self, address: &SocketAddr) -> bool {
        let ip_string = address.ip().to_string();
        {
            let ip_table = self.blocked_ip.read().unwrap();
            if ip_table.contains(&ip_string) {
                return false;
            }
        }
        let ports = address.port();
        {
            let port_table = self.blocked_ports.read().unwrap();
            if port_table.contains(&ports) {
                return false;
            }
        }

        return true;
    }
}
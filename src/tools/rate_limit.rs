use std::collections::HashMap;
use std::net::IpAddr;
use std::ptr::null;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use crate::tools::firewall::Firewall;


struct ip_stats {
    //How many requests an ip has made in the last second/minute
    second_count: u32,
    minute_count: u32,
    //When counter was last reset
    last_second_reset: Instant,
    last_minute_reset: Instant,
}

pub struct rate_limiter {
    firewall: Arc<Firewall>,
    data: Arc<RwLock<HashMap<IpAddr, ip_stats>>>,
}

impl rate_limiter {
    //Constructor builds new rate_limiter with firewall rules being passed in
    pub fn new_rate_limiter(firewall: Arc<Firewall>) -> Self {
        Self {
            firewall,
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn record_request(&self, ip: IpAddr) {
        let now = Instant::now();
        let mut rates_table = self.data.write().unwrap();

        let stat_tracker = rates_table.entry(ip).or_insert(ip_stats {
            second_count: 0,
            minute_count: 0,
            last_second_reset: now,
            last_minute_reset: now,
        });

        if now.duration_since(stat_tracker.last_second_reset) >= Duration::from_secs(1) {
            stat_tracker.second_count = 0;
            stat_tracker.last_second_reset = now;
        }

        if now.duration_since(stat_tracker.last_minute_reset) >= Duration::from_secs(60) {
            stat_tracker.minute_count = 0;
            stat_tracker.last_minute_reset = now;
        }

        stat_tracker.second_count += 1;
        stat_tracker.minute_count += 1;

        //World record for human clicks per second is 17.4, so we make our per second limit 20 (To make room for the freaks)
        //1000 requests per minute is plenty as an upper limit request/minute for this application
        if stat_tracker.second_count >= 20 || stat_tracker.minute_count >= 1000 {
            self.firewall.block_new_ip(ip);
        }
    }
}







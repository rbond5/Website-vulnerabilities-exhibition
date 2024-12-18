use actix_web::dev::{Transform, Service, ServiceRequest, ServiceResponse};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::{App, HttpServer, HttpResponse, web, middleware::Logger, Error};
use futures_util::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Arc;

use crate::tools::firewall::Firewall;
use crate::tools::rate_limit::RateLimiter;

struct MyMiddleware {
    firewall: Arc<Firewall>,
    rate_limiter: Arc<RateLimiter>,
}

impl MyMiddleware {
    pub fn new(firewall: Arc<Firewall>, rate_limiter: Arc<RateLimiter>) -> Self {
        MyMiddleware { firewall, rate_limiter }
    }
}

// Transform now specifies that the final response body is BoxBody
impl<S> Transform<S, ServiceRequest> for MyMiddleware
where
    S: Service<ServiceRequest, Error = Error> + 'static,
    S::Response: MessageBody + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = MyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MyMiddlewareService {
            service: Arc::new(service),
            firewall: Arc::clone(&self.firewall),
            rate_limiter: Arc::clone(&self.rate_limiter),
        })
    }
} 

struct MyMiddlewareService<S> {
    service: Arc<S>,
    firewall: Arc<Firewall>,
    rate_limiter: Arc<RateLimiter>,
}

// Note how we specify Response = ServiceResponse<BoxBody> here
impl<S, B> Service<ServiceRequest> for MyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<BoxBody>, Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let firewall = Arc::clone(&self.firewall);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let service = Arc::clone(&self.service);
        let remote_address = req.connection_info().realip_remote_addr().map(|s| s.to_owned());

        let fut = async move {
            if let Some(ip_string) = remote_address {
                if let Ok(ip) = ip_string.parse::<std::net::IpAddr>() {
                    rate_limiter.record_request(ip);
                    let address = std::net::SocketAddr::new(ip, 0);
                    if firewall.check_if_allowed(&address) == false {
                        let response = HttpResponse::Forbidden()
                            .body("Your IP has been blocked by this application's firewall")
                            .map_into_boxed_body();

                        return Ok(req.into_response(response));
                    }
                }
            }

            // Call underlying service and map body into BoxBody
            let response = self.service.call(req).await?; // response: ServiceResponse<B>
            let response = response.map_into_boxed_body(); // Now response: ServiceResponse<BoxBody>

            Ok(response)
        };

        Box::pin(fut)
    }
}

use futures::future::ok;
use http::FutureHandled;
use http::ServerResponse;
use hyper;
use hyper::Request;
use hyper::Response;
use hyper::server::NewService;
use hyper::server::Service;
use std::io;

/// `/health` service returning OK status if microservice is running (obviously)
#[derive(Debug, Copy, Clone)]
pub struct Health;

impl NewService for Health {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Health;
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Health)
    }
}

impl Service for Health {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureHandled;
    fn call(&self, _req: Request) -> Self::Future {
        box ok(ServerResponse::Data(HealthStatus::ok()).into())
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
struct HealthStatus {
    status: &'static str,
}

impl HealthStatus {
    fn ok() -> Self {
        HealthStatus { status: "OK" }
    }
}
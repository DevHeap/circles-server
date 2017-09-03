use hyper::{Request, Response, StatusCode};
use hyper;
use auth::Token;
use futures::Future;
use futures::future;
use tokio_service::Service;

pub struct Dispatcher {
    token: Token
}

impl Dispatcher {
    pub fn new(token: Token) -> Self {
        Dispatcher {
            token
        }
    }
}

impl Service for Dispatcher {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let uri = req.uri();
        let method = req.method();
        let user = self.token.user_id();
        let message = format!("[Dispatcher] User {}: accepted {} request for {}", user, method, uri);
        
        // @TODO logging
        println!("{}", message);

        let resp = Response::new()
            .with_status(StatusCode::Ok)
            .with_body(message);

        box future::ok(resp)
    }
}
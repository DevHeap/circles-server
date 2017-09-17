use hyper::{Request, Response};
use hyper;
use middleware::header::UserID;
use futures::Future;
use futures::future;
use tokio_service::Service;
use std::collections::BTreeMap;
use std::borrow::Cow;
use middleware::ErrorKind;
use middleware::error::ErrorResponse;

pub type FutureRoute = Box<Future<Item=Response, Error=hyper::Error>>;
pub type HandlerService = Service<
    Request = Request,
    Response = Response,
    Error = hyper::Error,
    Future = FutureRoute
>;

pub struct Router {
    routes: BTreeMap<String, Box<HandlerService>>
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: BTreeMap::new()
        }
    }

    pub fn add_route<P>(&mut self, path: P, handler: Box<HandlerService>) 
        where P: Into<Cow<'static, str>>
    {
        let path = path.into();
        self.routes.insert(path.into_owned(), handler);
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureRoute;

    fn call(&self, req: Self::Request) -> Self::Future {
        let handler = {
            let path = req.path();
            let method = req.method();
            let user = req.headers().get::<UserID>();
            let message = format!("User {:?}: accepted {} request for {}", user, method, path);
        
            // @TODO logging
            debug!("{}", message);

            let handler = match self.routes.get(path) {
                Some(handler) => handler,
                None => return box future::ok(
                    ErrorResponse::from(
                        ErrorKind::PathNotFound(path.to_owned())
                    ).into()
                )
            };

            handler
        };

        handler.call(req)
    }
}
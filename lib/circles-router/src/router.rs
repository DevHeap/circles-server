use hyper::{Request, Response};
use hyper::server::NewService;
use hyper::server::Service;
use hyper;
use futures::Future;
use futures::future;
use std::collections::BTreeMap;
use std::borrow::Cow;
use std::rc::Rc;
use std::io;

use ::error::ErrorKind;

use hyper_common::header::UserID;
use hyper_common::ErrorResponse;

/// Route handler return type
pub type FutureRoute = Box<Future<Item=Response, Error=hyper::Error>>;

/// Route handler Sevice Trait type
pub type HandlerService = Service<
    Request = Request,
    Response = Response,
    Error = hyper::Error,
    Future = FutureRoute
>;

type Routes = BTreeMap<String, Rc<HandlerService>>;

/// Router dispatches requests to specific handlers based on the request path
/// 
/// Invalid or non-registered routes will return a 404 error.
///
/// # Examples
///
/// ```
/// #![feature(box_syntax)]
/// extern crate circles_router;
/// extern crate service_fn;
/// extern crate hyper;
/// extern crate futures;
///
/// use circles_router::{RouterBuilder, FutureRoute};
/// use service_fn::service_fn;
/// use hyper::Request;
/// use hyper::Response;
/// use hyper::server::Http;
/// use futures::future::ok;
///
/// fn main() {
///     let addr = "127.0.0.1:8080".parse().unwrap();
///     let router = RouterBuilder::new()
///         .add_route("/",  box service_fn(|req: Request| -> FutureRoute {
///             println!("Accepted request to /");
///             box ok(Response::new())
///         }))
///        .build();
///     
///     Http::new().bind(&addr, router)
///         .expect("Failed to start server");
///       //.run().unwrap()
/// } 
/// ```
pub struct RouterBuilder {
    routes: Routes
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            routes: BTreeMap::new()
        }
    }

    pub fn add_route<P>(mut self, path: P, handler: Box<HandlerService>) -> Self
        where P: Into<Cow<'static, str>>
    {
        let path = path.into();
        self.routes.insert(path.into_owned(), Rc::new(handler));
        self
    }

    pub fn build(self) -> Router {
        Router {
            routes: Rc::new(self.routes)
        }
    }
}

/// hyper::Service factory, constructed by 
/// RouterBuilder::build() 
pub struct Router {
    routes: Rc<Routes>
}

impl NewService for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = RouterService;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(RouterService {
            routes: self.routes.clone()
        })
    }
}

/// hyper::server::Service constructed with 
/// Router::new_service()
pub struct RouterService {
    routes: Rc<Routes>
}

impl Service for RouterService {
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
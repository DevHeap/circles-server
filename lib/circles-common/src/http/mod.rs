#[macro_use]
pub mod service;
pub mod error;
pub mod header;
pub mod error_response;

pub use self::error_response::ErrorResponse;

use futures::Future;
use hyper;
use hyper::server::{Request, Response, Service, NewService};

/// Route handler return type
pub type FutureHandled = Box<Future<Item = Response, Error = hyper::Error>>;

/// Handler Factory Trait type
pub type HandlerFactory = NewService<
    Request = Request,
    Response = Response,
    Error = hyper::Error,
    Instance = Box<HandlerService>,
>;

/// Handler Service Trait type
pub type HandlerService = Service<
    Request = Request,
    Response = Response,
    Error = hyper::Error,
    Future = FutureHandled,
>;

#[macro_export]
macro_rules! router {
    ($($name:tt: $method:path, $path:expr => $handler:expr,)*) => {{       
        use hyper;
        use hyper::Request;
        use hyper::Response;
        use hyper::Method;
        use hyper::server::Service;
        use hyper::server::NewService;
        use ::circles_common::http::FutureHandled;
        use ::circles_common::http::HandlerService;
        use ::circles_common::http::ErrorResponse;
        use ::circles_common::http::error::ErrorKind;
        use futures::future::ok;
        use std::rc::Rc;
        use std::io;

        struct Router {
            $($name: Rc<HandlerService>,)*
        }

        impl NewService for Router {
            type Request = Request;
            type Response = Response;
            type Error = hyper::Error;
            type Instance = Box<HandlerService>;
            fn new_service(&self) -> io::Result<Self::Instance> {
                Ok(box RouterServive {
                    $($name: self.$name.clone(),)*
                })
            }
        }

        struct RouterServive {
            $($name: Rc<HandlerService>,)*
        }

        impl Service for RouterServive {
            type Request = Request;
            type Response = Response;
            type Error = hyper::Error;
            type Future = FutureHandled;
            fn call(&self, req: Request) -> Self::Future {
                $(
                    if req.method() == &$method
                    && req.path() == $path {
                        return self.$name.call(req)
                    }
                )*
                
                box ok(ErrorResponse::from(
                    ErrorKind::PathNotFound(
                        req.method().clone(),
                        req.path().to_owned()
                    )).into()
                )
            }
        }

        Router {
            $($name: $handler,)*
        }
    }}
}

/*

use hyper;
use hyper::Request;
use hyper::Response;
use hyper::Method;
use hyper::server::Service;
use http::FutureRoute;
use std::rc::Rc;

struct DummyService;
impl Service for DummyService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureRoute;

    fn call(&self, req: Request) -> Self::Future {
        unimplemented!()
    }
}

fn test() {
    let router = router!(
        post_position: Method::Post,"/position" => Rc::new(DummyService),
    );
}

*/
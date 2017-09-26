//! Static Router construction macro

#[macro_export]
macro_rules! router {
    ($($name:tt: $method:path, $path:expr => $handler:expr,)*) => {{       
        use hyper;
        use hyper::Request;
        use hyper::Response;
        use hyper::Method;
        use hyper::server::Service;
        use hyper::server::NewService;
        use http::FutureHandled;
        use http::HandlerService;
        use http::ServerResponse;
        use http::error::ErrorKind;
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
                
                box ok(ServerResponse::from(
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

use hyper;
use hyper::server::Service;
use hyper::{Request, Response};
use http::FutureHandled;

// Until i write tests for reuter
// @TODO write tests
#[allow(dead_code)]
fn does_it_even_compile_test() {
    struct DummyService;
    impl Service for DummyService {
        type Request = Request;
        type Response = Response;
        type Error = hyper::Error;
        type Future = FutureHandled;
        fn call(&self, _req: Request) -> FutureHandled {
            unimplemented!()
        }
    }

    let _router =
        router!(
        get_dummy: Method::Get, "/dummy" => Rc::new(DummyService),
        put_dymmy: Method::Put, "/dummy" => Rc::new(DummyService),
    );
}

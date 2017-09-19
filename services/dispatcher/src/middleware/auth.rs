use hyper::{Request, Response};
use hyper::header::{Authorization, Bearer};
use hyper;
use tokio_service::Service;
use futures::future;
use futures::Future;
use std::rc::Rc;

use firebase::Token;
use firebase::AsyncTokenVerifier;

use circles_router::router::Router;
use hyper::server::NewService;
use hyper_common::header::UserID;
use hyper_common::ErrorResponse;

use middleware::error::ErrorKind as MwErrorKind;
use middleware::error::Result as MwResult;

pub struct Authenticator {
    auth: Rc<AsyncTokenVerifier>,
    router: Rc<Router>
}   

impl Authenticator {
    pub fn new(auth: Rc<AsyncTokenVerifier>, router: Rc<Router>) -> Self {
        Authenticator {
            auth,
            router
        }
    }

    fn extract_token(req: &Request) -> Result<String, ErrorResponse> {
        let headers = req.headers();
        let bearer: &Authorization<Bearer> = headers.get()
            .ok_or(ErrorResponse::from(MwErrorKind::AuthHeaderMissing))?;

        // @TODO can we avoid cloning here?
        Ok(bearer.token.clone())
    }
}

impl Service for Authenticator {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, mut req: Request) -> Self::Future {
        let uri = req.uri().clone();

        trace!("accepted {} request for {}", req.method(), req.uri());

        // Extract IDToken from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token,
            Err(error) => return box future::ok(error.into())
        };

        let router = self.router.new_service()
        // Can never happen. Really.
            .unwrap(); 

        // Either pass the request to the Dispatcher or return error response to a client
        let future_response = self.auth.authenticate(token).map_err(|e| e.into())
                                       .then(move |auth_result: MwResult<Token>| {
            match auth_result {
                Ok(token) => {
                    debug!("authorized request from user {}", token.user_id());
                    
                    // Set UserID header
                    let uid = token.user_id().to_owned();
                    req.headers_mut().set(UserID(uid));

                    
                    // Pass the request to dispatcher
                    router.call(req)
                },
                Err(err) => {
                    debug!("attempted unathorized access to {}", uri);
                    box future::ok(ErrorResponse::from(err).into())
                }
            }
        });

        box future_response
    }
}


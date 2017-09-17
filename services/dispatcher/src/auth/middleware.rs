use hyper::{Request, Response};
use hyper::header::{Authorization, Bearer};
use hyper;
use tokio_service::Service;
use futures::future;
use futures::Future;
use std::rc::Rc;

use auth;
use auth::Authenticator;
use auth::error::ErrorKind;

use dispatcher::Dispatcher;

pub struct AuthMiddleware {
    auth: Rc<Authenticator>
}   

impl AuthMiddleware {
    pub fn new(auth: Rc<Authenticator>) -> Self {
        AuthMiddleware {
            auth
        }
    }

    fn extract_token(req: &Request) -> Result<String, ErrorResponse> {
        let headers = req.headers();
        let bearer: &Authorization<Bearer> = headers.get()
            .ok_or(ErrorResponse::from(ErrorKind::AuthHeaderMissing))?;

        // @TODO can we avoid cloning here?
        Ok(bearer.token.clone())
    }
}

impl Service for AuthMiddleware {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let uri = req.uri().clone();

        info!("Accepted {} request for {}", req.method(), req.uri());

        // Extract IDToken from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token,
            Err(error) => return box future::ok(error.into())
        };

        // Either pass the request to the Dispatcher or return error response to a client
        let future_response = self.auth.authenticate(token).then(move |auth_result| {
            match auth_result {
                Ok(token) => {
                    info!("Authorized request from user {}", token.user_id());
                    let dispatcher = Dispatcher::new(token);
                    dispatcher.call(req)
                },
                Err(err) => {
                    info!("Attempted unathorized access to {}", uri);
                    box future::ok(ErrorResponse::from(err).into())
                }
            }
        });

        box future_response
    }
}

/// Internal request handling errors should correspond to valid HTTP codes 
/// and be returned to client as error messages
#[derive(Debug)]
struct ErrorResponse {
    status: hyper::StatusCode,
    message: String,
}

use std;
use std::fmt;

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}): {}", self.status, self.message)
    }
}

impl std::error::Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.message
    }
}

/// Convert ErrorResponse into hyper::Response that can be send to a client
impl Into<Response> for ErrorResponse {
    fn into(self) -> Response {
        let mut response = Response::default();
        response.set_status(self.status);
        response.set_body(self.to_json());
        response
    } 
}

use std::borrow::Borrow;

/// This function converts ErrorKind and &ErrorKind to an ErrorResponse
impl<A> From<A> for ErrorResponse
    where A: Borrow<auth::ErrorKind>
{
    fn from(err: A) -> Self {
        use auth::error::ErrorKind::*;
        use hyper::StatusCode;

        let err = err.borrow();

        let with_status = |status| {
            ErrorResponse {
                status: status,
                message: format!("{}", err)
            }
        };

        match *err {
            FailedToRetrieveKeyring(..) | Io(..) | Hyper(..) | OpenSSL(..) | OpenSSLStack(..) | Reqwest(..) 
              => with_status(StatusCode::InternalServerError),
            _ => with_status(StatusCode::Unauthorized),
        }
    }
}

impl From<auth::Error> for ErrorResponse {
    fn from(err: auth::Error) -> Self {
        ErrorResponse::from(err.kind())
    }
}

// @TODO use serde here
impl ErrorResponse {
    fn to_json(&self) -> String {
        format!("{{\"error\":{{\"status\":{},\"message\":{:?}}}}}", 
        self.status.as_u16(),
        self.message)
    } 
}


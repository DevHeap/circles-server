use hyper;
use hyper::Response;

error_chain! {
    links {
        Firebase(::firebase::Error, ::firebase::ErrorKind);
    }

    errors {
        AuthHeaderMissing {
            description("missing Authorization header")
            display("missing Authorization header")
        }

        PathNotFound(path: String) {
            description("path not found")
            display("path {} does not exist", path)
        }
    }
}

/// Internal request handling errors should correspond to valid HTTP codes 
/// and be returned to client as error messages
#[derive(Debug)]
pub struct ErrorResponse {
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
    where A: Borrow<ErrorKind>
{
    fn from(err: A) -> Self {
        use firebase::ErrorKind::*;
        use hyper::StatusCode;

        let err = err.borrow();

        let with_status = |status| {
            ErrorResponse {
                status: status,
                message: format!("{}", err)
            }
        };

        match *err {
            ErrorKind::Firebase(ref err) => match *err {
                FailedToRetrieveKeyring(..) | Io(..) | Hyper(..) | OpenSSL(..) | OpenSSLStack(..) | Reqwest(..) 
                  => with_status(StatusCode::InternalServerError),
                _ => with_status(StatusCode::Unauthorized),
            },
            ErrorKind::AuthHeaderMissing => with_status(StatusCode::Unauthorized),
            ErrorKind::PathNotFound(..)  => with_status(StatusCode::NotFound),
            _ => unimplemented!()
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
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

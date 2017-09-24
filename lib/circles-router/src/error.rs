
#![allow(unused_doc_comment)]

use hyper::StatusCode;
use hyper::Method;
use hyper_common::ErrorResponse;

error_chain! {
    errors {
        AuthHeaderMissing {
            description("missing Authorization header")
            display("missing Authorization header")
        }

        PathNotFound(method: Method, path: String) {
            description("path not found")
            display("path {} for method {} does not exist", path, method)
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        use ErrorKind::*;
        match *e.kind() {
            AuthHeaderMissing => ErrorResponse::with_status(&e, StatusCode::Unauthorized),
            PathNotFound(..)  => ErrorResponse::with_status(&e, StatusCode::NotFound),
            Msg(..)           => ErrorResponse::with_status(&e, StatusCode::InternalServerError),
        }
    }
}

impl From<ErrorKind> for ErrorResponse {
    fn from(ek: ErrorKind) -> Self {
        ErrorResponse::from(Error::from(ek))
    }
}
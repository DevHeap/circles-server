
#![allow(unused_doc_comment)]

use hyper::StatusCode;
use hyper::Method;
use http::ErrorResponse;

error_chain!{
    links {
        Firebase(::firebase::Error, ::firebase::ErrorKind);
        Database(::db::Error, ::db::ErrorKind);
    }

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
        match *e.kind() {
            ErrorKind::Firebase(ref e)   => ErrorResponse::from(e),
            ErrorKind::Database(ref e)   => ErrorResponse::with_status(&e, StatusCode::InternalServerError),
            ErrorKind::AuthHeaderMissing => ErrorResponse::with_status(&e, StatusCode::Unauthorized),
            ErrorKind::PathNotFound(..)  => ErrorResponse::with_status(&e, StatusCode::NotFound),
            ErrorKind::Msg(..)           => ErrorResponse::with_status(&e, StatusCode::InternalServerError)
        }
    }
}

impl From<ErrorKind> for ErrorResponse {
    fn from(ek: ErrorKind) -> Self {
        ErrorResponse::from(Error::from(ek))
    }
}
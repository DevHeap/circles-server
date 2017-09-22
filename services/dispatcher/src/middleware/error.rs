#![allow(unused_doc_comment)]

use hyper::StatusCode;
use hyper_common::ErrorResponse;

error_chain!{
    links {
        Firebase(::firebase::Error, ::firebase::ErrorKind);
        Database(::circles_common::db::Error, ::circles_common::db::ErrorKind);
    }

    errors {
        AuthHeaderMissing {
            description("missing Authorization header")
            display("missing Authorization header")
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        match *e.kind() {
            ErrorKind::Firebase(ref e)   => ErrorResponse::from(e),
            ErrorKind::Database(ref e)   => ErrorResponse::with_status(&e, StatusCode::InternalServerError),
            ErrorKind::AuthHeaderMissing => ErrorResponse::with_status(&e, StatusCode::Unauthorized),
            ErrorKind::Msg(..)           => ErrorResponse::with_status(&e, StatusCode::InternalServerError)
        }
    }
}

impl From<ErrorKind> for ErrorResponse {
    fn from(ek: ErrorKind) -> Self {
        ErrorResponse::from(Error::from(ek))
    }
}
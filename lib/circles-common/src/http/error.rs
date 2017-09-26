
#![allow(unused_doc_comment)]
#![allow(missing_docs)]

use hyper::StatusCode;
use hyper::Method;
use http::ApiError;

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
        
        MissingUserIDHeader {
            description("missing UserID header")
            display("missing UserID header")
        }
    }
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        match *e.kind() {
            ErrorKind::Firebase(ref e) => ApiError::from(e),
            ErrorKind::Database(ref e) => {
                ApiError::with_status(&e, StatusCode::InternalServerError)
            }
            ErrorKind::AuthHeaderMissing => ApiError::with_status(&e, StatusCode::Unauthorized),
            ErrorKind::PathNotFound(..) => ApiError::with_status(&e, StatusCode::NotFound),
            ErrorKind::MissingUserIDHeader => {
                ApiError::with_status(&e, StatusCode::InternalServerError)
            }
            ErrorKind::Msg(..) => ApiError::with_status(&e, StatusCode::InternalServerError),
        }
    }
}

impl From<ErrorKind> for ApiError {
    fn from(ek: ErrorKind) -> Self {
        ApiError::from(Error::from(ek))
    }
}

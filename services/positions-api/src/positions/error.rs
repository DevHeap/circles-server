#![allow(unused_doc_comment)]

use circles_common::http::ErrorResponse;
use hyper::StatusCode;

error_chain!{
    foreign_links {
        Utf8(::std::string::FromUtf8Error);
        Json(::json::error::Error);
        Hyper(::hyper::Error);
    }

    links {
        Db(::circles_common::db::Error, ::circles_common::db::ErrorKind);
    }
}


impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        Self::from(e.kind())
    }
}

impl From<ErrorKind> for ErrorResponse {
    fn from(ek: ErrorKind) -> Self {
        Self::from(&ek)
    }
}

impl<'a> From<&'a ErrorKind> for ErrorResponse {
    fn from(ek: &'a ErrorKind) -> Self {
        use positions::error::ErrorKind::*;
        match *ek {
            Utf8(..) | Json(..)          => Self::with_status(ek, StatusCode::BadRequest),
            Hyper(..) | Db(..) | Msg(..) => Self::with_status(ek, StatusCode::InternalServerError)
        }
    }
}

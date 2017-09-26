#![allow(unused_doc_comment)]
#![allow(missing_docs)]

use hyper::StatusCode;
use http::ApiError;

// Generate error types boilerplate

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(::json::error::Error);
        Hyper(::hyper::Error);
        Utf8(::std::string::FromUtf8Error);
        RustyJwt(::jwt::Error);
        OpenSSL(::openssl::error::Error);
        OpenSSLStack(::openssl::error::ErrorStack);
        Base64(::base64::DecodeError);
        Reqwest(::reqwest::Error);
    }

    errors {
        FailedToRetrieveKeyring(status: StatusCode) {
            description("failed to retrieve google keyring")
            display("failed to retrieve google keyring: {}", status)
        }

        EmptyUserID {
            description("userid is empty")
            display("userid is empty")
        }

        UnknownKeyID {
            description("unknown key id")
            display("unknown key id")
        }
    }
}

/// This function converts ErrorKind and &ErrorKind to an ErrorResponse
impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::from(e.kind())
    }
}

impl From<ErrorKind> for ApiError {
    fn from(ek: ErrorKind) -> Self {
        ApiError::from(&ek)
    }
}

impl<'a> From<&'a ErrorKind> for ApiError {
    fn from(ek: &'a ErrorKind) -> Self {
        use firebase::ErrorKind::*;
        match *ek {
            FailedToRetrieveKeyring(..) |
            Io(..) |
            Hyper(..) |
            OpenSSL(..) |
            OpenSSLStack(..) |
            Reqwest(..) |
            Msg(..) => ApiError::with_status(&ek, StatusCode::InternalServerError),
            _ => ApiError::with_status(&ek, StatusCode::Unauthorized),
        }
    }
}

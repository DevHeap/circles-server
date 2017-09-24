//! Server Response with an error message

use std::fmt;
use std::fmt::Display;
use std::error::Error;

use hyper::Response;
use hyper::StatusCode;

/// Internal request handling errors should correspond to valid HTTP codes
/// and be returned to client as error messages
///
/// To implement this behaviour, provide a From<YourError> implementation for ErrorResponse
///
/// # Examples
///
/// ```
/// extern crate hyper;
/// extern crate circles_common;
///
/// use std::error::Error;
/// use std::fmt;
/// use hyper::StatusCode;
/// use circles_common::http::ErrorResponse;
///
/// #[derive(Debug)]
/// enum MyError {
///     ClientSentCrap,
///     ServerIsDead
/// }
///
/// impl Error for MyError {
///     fn description(&self) -> &str {
///         use MyError::*;
///         match *self {
///             ClientSentCrap => "Client's fault",
///             ServerIsDead   => "Mike's fault",
///         }
///     }
/// }
///
/// impl fmt::Display for MyError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "{}", self.description())
///     }
/// }
///
/// impl From<MyError> for ErrorResponse {
///     fn from(e: MyError) -> Self {
///         use MyError::*;
///         match e {
///             ClientSentCrap => ErrorResponse::with_status(&e, StatusCode::Unauthorized),
///             ServerIsDead   => ErrorResponse::with_status(&e, StatusCode::InternalServerError),
///         }
///     }
/// }
///
/// # fn main() {}
/// ```
#[derive(Debug)]
pub struct ErrorResponse {
    status: StatusCode,
    message: String,
}

impl ErrorResponse {
    /// Construct ErrorResponse with an Error and a custom HTTP StatusCode
    pub fn with_status<D>(d: &D, status: StatusCode) -> Self
    where
        D: Display,
    {
        ErrorResponse {
            status,
            message: format!("{}", d),
        }
    }

    // @TODO use serde here
    fn to_json(&self) -> String {
        format!(
            "{{\"error\":{{\"status\":{},\"message\":{:?}}}}}",
            self.status.as_u16(),
            self.message
        )
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}): {}", self.status, self.message)
    }
}

impl Error for ErrorResponse {
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

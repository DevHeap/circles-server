//! Server Response with an error message


use hyper::Response;
use hyper::StatusCode;

use json;
use serde::Serialize;

use std::error::Error;
use std::fmt;
use std::fmt::Display;

/// Generic Server Response
///
/// Server may return either some data or an error
///
/// # Examples
///
/// ```
/// #![feature(box_syntax)]
/// extern crate hyper;
/// extern crate futures;
/// extern crate circles_common;
///
/// use hyper::Request;
/// use futures::future;
/// use circles_common::http::FutureHandled;
/// use circles_common::http::error::ErrorKind;
/// use circles_common::http::ServerResponse;
///
/// fn call(req: Request) -> FutureHandled {
///     # let success_condition = true;
///     /* ...
///        some request handling code resulting the success_confition
///     */
///
///     // It's better to always have a custom error type with ErrorChain
///     let error_msg = ":C".to_string();
///     let data = "C:".to_string();
///
///     if success_condition {
///         box future::ok(ServerResponse::Data(data).into())
///     } else {
///         box future::ok(ServerResponse::from(ErrorKind::Msg(error_msg)).into())
///     }
/// }
/// # fn main() {}
/// ```
#[derive(Debug, Serialize)]
pub enum ServerResponse<Data>
where
    Data: Serialize,
{
    /// Response with some generic data serializible as a json
    Data(Data),
    /// Response with an error
    Error(ApiError),
}

/// Convert ServerResponse into hyper::Response that can be send to a client
impl<D> Into<Response> for ServerResponse<D>
where
    D: Serialize,
{
    fn into(self) -> Response {
        let (status, body) = match self {
            ServerResponse::Data(data) => (StatusCode::Ok, json::to_string(&data).unwrap()),
            ServerResponse::Error(error) => (error.status_code, json::to_string(&error).unwrap()),
        };

        let mut response = Response::default();
        response.set_status(status);
        response.set_body(body);
        response
    }
}

impl<E> From<E> for ServerResponse<()>
where
    ApiError: From<E>,
{
    fn from(error: E) -> Self {
        ServerResponse::Error(ApiError::from(error))
    }
}

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
/// use circles_common::http::ApiError;
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
/// impl From<MyError> for ApiError {
///     fn from(e: MyError) -> Self {
///         use MyError::*;
///         match e {
///             ClientSentCrap => ApiError::with_status(&e, StatusCode::Unauthorized),
///             ServerIsDead   => ApiError::with_status(&e, StatusCode::InternalServerError),
///         }
///     }
/// }
///
/// # fn main() {}
/// ```
#[derive(Debug, Serialize)]
pub struct ApiError {
    status: String,
    message: String,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

impl ApiError {
    /// Construct ApiError with an Error and a custom HTTP StatusCode
    pub fn with_status<D>(d: &D, status: StatusCode) -> Self
    where
        D: Display,
    {
        ApiError {
            status: format!("{}", status),
            message: format!("{}", d),
            status_code: status,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}): {}", self.status, self.message)
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        &self.message
    }
}

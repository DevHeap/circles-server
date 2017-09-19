///! Common HTTP Rust primitives for Circles Server
extern crate hyper;

pub mod error_response;
pub use error_response::ErrorResponse;

pub mod header;

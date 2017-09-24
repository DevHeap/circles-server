///! Common HTTP Rust primitives for Circles Server
extern crate hyper;
#[macro_use]
extern crate error_chain;

pub mod error_response;
pub use error_response::ErrorResponse;

pub mod header;

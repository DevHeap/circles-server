#![feature(box_syntax)]
#![allow(unused_doc_comment)]

///! Router for Hyper Services

extern crate futures;
extern crate hyper;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

extern crate hyper_common;

pub mod router;
pub use router::RouterBuilder;
pub use router::FutureRoute;
pub use router::HandlerService;

pub mod error;
pub use error::Error;
pub use error::ErrorKind;
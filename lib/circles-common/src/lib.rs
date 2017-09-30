#![feature(conservative_impl_trait, box_syntax, specialization)]
#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

//! Crate of the common building blocks for the Circles microservices
//!
//! Everything that may be used twice or more in separate microservices
//! must be placed here.
//!
//! Copy-Paste driven development is a no-no :)

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate log;
extern crate hyper;
extern crate reqwest;
extern crate openssl;
extern crate rustwt as jwt;
extern crate base64;

#[macro_use]
extern crate error_chain;

/// Database model, schema and convenience traits
pub mod db;
/// Firebase JWT tokens decode and verification routines
pub mod firebase;
/// JSON types and protocol
pub mod proto;

/// Everything for Hyper and Http servers
#[macro_use]
pub mod http;

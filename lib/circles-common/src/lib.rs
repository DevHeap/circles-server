#![feature(conservative_impl_trait)] 

#[macro_use]
extern crate diesel_codegen;
#[macro_use] 
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
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
extern crate rusty_jwt as jwt;
extern crate base64;
extern crate serde_json as json;

#[macro_use]
extern crate error_chain;

extern crate hyper_common;

pub mod db;
pub mod firebase;
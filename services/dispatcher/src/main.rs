#![feature(proc_macro, conservative_impl_trait, generators, box_syntax)]
#![recursion_limit = "1024"]

extern crate rdkafka;
extern crate tokio_core;
extern crate tokio_service;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate base64;
extern crate rusty_jwt as jwt;
extern crate openssl;
extern crate chrono;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

mod auth;
mod dispatcher;

use hyper::server::Http;

use auth::Authenticator;
use auth::AuthMiddleware;

fn main() {
    let addr = "0.0.0.0:3247".parse().unwrap();

    // Auth state with Google keyring. 
    // Shares the whole application lifetime
    let auth = Authenticator::new();

    // Here is some borrowck magic with not-so-moving the value
    // Why it is OK to move the value into an Fn closure? HELL KNOWS.
    let server = Http::new()
        .bind(&addr, move || Ok(AuthMiddleware::new(&auth))).unwrap();
    
    // Unwraps here are ok: if smth goes wrong so badly that we have no error handling,
    // it's either a bug or external failure we have no control upon.
    server.run().unwrap();
}

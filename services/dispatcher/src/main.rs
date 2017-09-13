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
use tokio_core::reactor;
use tokio_core::net::TcpListener;
use futures::Stream;

use auth::Authenticator;
use auth::AuthMiddleware;

use std::rc::Rc;
use std::net::SocketAddr;

fn main() {
    let addr = "0.0.0.0:3247".parse().unwrap();

    // Auth state with Google keyring. 
    // Shares the whole application lifetime
    let auth = Rc::new(Authenticator::new());

    // Unwraps here are ok: if smth goes wrong so badly that we have no error handling,
    // it's either a bug or external failure we have no control upon.
    
    // Starting tokio event loop
    let mut core = reactor::Core::new()
        .expect("Failed to initialize event loop");
    let handle = core.handle();

    // Starting TCP server listening for incoming commections
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        // AuthMiddleware is an entry point: it will pass requests to Dispatcher
        let service_entry = AuthMiddleware::new(auth.clone());

        // Handing TCP connections over to Hyper
        Http::new().bind_connection(&handle, sock, addr, service_entry);
        Ok(())
    });

    // Launching an event loop: unless it is spinned up, nothing happens
    core.run(server)
        .expect("Critical server failure");
}


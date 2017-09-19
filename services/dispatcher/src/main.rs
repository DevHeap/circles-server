#![feature(proc_macro, conservative_impl_trait, generators, box_syntax)]
#![recursion_limit = "1024"]

extern crate rdkafka;
extern crate tokio_core;
extern crate tokio_service;
extern crate service_fn;
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

#[macro_use]
extern crate log;
extern crate fern;

extern crate circles_router;
extern crate hyper_common;

mod firebase;
mod middleware;

use hyper::server::Http;
use tokio_core::reactor;
use tokio_core::net::TcpListener;
use futures::Stream;

use firebase::AsyncTokenVerifier;

use middleware::Authenticator;
use circles_router::RouterBuilder;
use circles_router::router::Router;

use std::rc::Rc;
use std::thread;
use std::sync::mpsc::channel;

fn init_logger() -> Result<(), log::SetLoggerError> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            print!("{}", msg);
        }
    });

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Warn)
        .level_for("dispatcher", log::LogLevelFilter::Info)
        .chain(tx)
        .apply()?;
    Ok(())
}

fn main() {
    init_logger().unwrap();
    let addr = "0.0.0.0:3247".parse().unwrap();

    // Auth state with Google keyring.
    // Shares the whole application lifetime
    let verifier = Rc::new(AsyncTokenVerifier::new());

    // Router to dispatch requests for concrete pathes to their handlers 
    let router = Rc::new(setup_router());

    // Unwraps here are ok: if smth goes wrong so badly that we have no error handling,
    // it's either a bug or external failure we have no control upon.

    // Starting tokio event loop
    let mut core = reactor::Core::new().expect("Failed to initialize event loop");
    let handle = core.handle();

    // Starting TCP server listening for incoming commections
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        // AuthMiddleware is an entry point: it will pass requests to Dispatcher
        let service_entry = Authenticator::new(verifier.clone(), 
                                               router.clone());

        // Handing TCP connections over to Hyper
        Http::new().bind_connection(&handle, sock, addr, service_entry);
        Ok(())
    });

    // Launching an event loop: unless it is spinned up, nothing happens
    core.run(server).expect("Critical server failure");
}

use service_fn::service_fn;
use hyper::server::Request;
use hyper::server::Response;
use futures::future;
use hyper_common::header::UserID;
use circles_router::FutureRoute;

// Setup routes and handlers
fn setup_router() -> Router {
    // A dummy handler for `/`
    RouterBuilder::new()
        .add_route("/", box service_fn(|req: Request| -> FutureRoute {
            let uid = req.headers().get::<UserID>().unwrap();
            let met = req.method();
            trace!("Accepted request {} to / from {:?}", met, uid);
            box future::ok(Response::new())
        }))
        .build()
}
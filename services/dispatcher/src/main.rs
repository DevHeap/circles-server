#![feature(proc_macro, conservative_impl_trait, generators, box_syntax)]
#![recursion_limit = "1024"]

extern crate tokio_core;
extern crate service_fn;
extern crate futures;
extern crate hyper;
extern crate chrono;
extern crate diesel;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate fern;

extern crate circles_router;
extern crate circles_common;
extern crate hyper_common;

use circles_common::firebase;
mod middleware;

use hyper::server::Http;
use hyper::server::NewService;
use tokio_core::reactor;
use tokio_core::net::TcpListener;
use futures::Stream;

use firebase::AsyncTokenVerifier;

use middleware::Authenticator;
use circles_router::RouterBuilder;
use circles_router::router::Router;
use circles_common::db::AsyncPgPool;

use std::rc::Rc;
use std::thread;
use std::sync::mpsc::channel;

// @TODO move to a shared library, implement log.toml config file
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
        .level_for("dispatcher",     log::LogLevelFilter::Trace)
        .level_for("circles_router", log::LogLevelFilter::Trace)
        .level_for("circles_common", log::LogLevelFilter::Trace)
        .level_for("hyper_common",   log::LogLevelFilter::Trace)
        .chain(tx)
        .apply()?;
    Ok(())
}

fn main() {
    init_logger().unwrap();
    info!("initialized logger");

    let addr = "0.0.0.0:7700".parse().unwrap();

    // Connection to database
    // @TODO read db uri from config file
    let db_uri = "postgres://devheap:Olb5Ind3rT@localhost/circles-dev";
    let pgpool = Rc::new(AsyncPgPool::connect(db_uri).unwrap());

    // Starting tokio event loop
    let mut core = reactor::Core::new().expect("Failed to initialize event loop");
    let handle = core.handle();

    // Auth state with Google keyring.
    // Shares the whole application lifetime
    let verifier = Rc::new(AsyncTokenVerifier::new());

    // Router to dispatch requests for concrete pathes to their handlers 
    let router = Rc::new(setup_router());

    // Authenticator to accept incoming request, check the token
    // and update database with user creds
    let authenticator = Authenticator::new(
        verifier.clone(),
        pgpool.clone(),
        router.clone(),
    );

    // Starting TCP server listening for incoming commections
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        let entry_service = authenticator.new_service()
        // Can never happen
            .unwrap();

        // Handing TCP connections over to Hyper
        Http::new().bind_connection(&handle, sock, addr, entry_service);
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

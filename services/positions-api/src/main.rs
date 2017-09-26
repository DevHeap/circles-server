#![feature(proc_macro, conservative_impl_trait, generators, box_syntax)]
#![recursion_limit = "1024"]

extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate chrono;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate fern;
extern crate serde_json as json;

#[macro_use]
extern crate circles_common;

mod positions;

use circles_common::db::AsyncPgPool;
use circles_common::http;

use futures::Stream;
use http::service::Authenticator;
use http::service::Health;

use hyper::server::Http;
use hyper::server::NewService;

use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;

use tokio_core::net::TcpListener;
use tokio_core::reactor;

use positions::post::PositionsPostHandler;

// @TODO move to a shared library, implement log.toml config file
fn init_logger() -> Result<(), log::SetLoggerError> {
    let (tx, rx) = channel();
    thread::spawn(move || while let Ok(msg) = rx.recv() {
        print!("{}", msg);
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
        .level_for("positions_api", log::LogLevelFilter::Trace)
        .level_for("circles_router", log::LogLevelFilter::Trace)
        .level_for("circles_common", log::LogLevelFilter::Trace)
        .level_for("hyper_common", log::LogLevelFilter::Trace)
        .chain(tx)
        .apply()?;
    Ok(())
}

fn main() {
    init_logger().unwrap();
    info!("initialized logger");

    let addr = "0.0.0.0:7701".parse().unwrap();

    // Connection to database
    // @TODO read db uri from config file
    let db_uri = "postgres://devheap:Olb5Ind3rT@localhost/circles-dev";
    let pgpool = Rc::new(AsyncPgPool::connect(db_uri).unwrap());

    // Starting tokio event loop
    let mut core = reactor::Core::new().expect("Failed to initialize event loop");
    let handle = core.handle();

    // Router to dispatch requests for concrete pathes to their handlers
    let router = router!(
        post_positions: Method::Post, "/positions" => Rc::new(PositionsPostHandler::new(pgpool.clone())),
        health: Method::Get, "/health" => Rc::new(Health),
    );

    // Authenticator for firebase token verification and user info population in the database
    let authenticator = Authenticator::new(pgpool.clone(), Rc::new(router));

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

#![feature(box_syntax)]

#[macro_use]
extern crate circles_common;
extern crate hyper;
extern crate reqwest;
extern crate futures;

use circles_common::http;
use circles_common::http::service::Health;
use hyper::server::Http;
use std::thread;

use std::time::Duration;

const HTTP_TEST_PORT: &str = "18001";

#[test]
fn health_http_service() {
    thread::spawn(move || {
        let addr = format!("127.0.0.1:{}", HTTP_TEST_PORT).parse().unwrap();

        let router =
            router!(
            health: Method::Get, "/health" => Rc::new(Health),
        );

        let server = Http::new().bind(&addr, router).unwrap();
        server.run().unwrap()
    });

    thread::sleep(Duration::from_millis(500));
    let resp = reqwest::get(&format!("http://localhost:{}/health", HTTP_TEST_PORT)).unwrap();
    assert!(resp.status().is_success());
}

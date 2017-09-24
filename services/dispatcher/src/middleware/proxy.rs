use hyper;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::{Request, Response};
use hyper::server::Service;
use tokio_core::reactor::Handle;
use std::borrow::Cow;

use circles_router::FutureRoute;

pub struct Proxy {
    proxy_url: Cow<'static, str>,
    client: Client<HttpConnector>
}


impl Proxy {
    pub fn new<C>(proxy_url: C, handle: &Handle) -> Self 
        where C: Into<Cow<'static, str>>
    {
        let proxy_url = proxy_url.into();
        Proxy {
            proxy_url: proxy_url,
            client: Client::new(handle)
        }
    }
}

impl Service for Proxy {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureRoute;

    fn call(&self, mut req: Request) -> Self::Future {
        // Concatenate base proxy target url with request path
        let uri = format!("{}{}", self.proxy_url, req.path());
        debug!("proxying request to {}", uri);
        
        // Spoof request uri
        let uri = uri.parse().unwrap();
        req.set_uri(uri);

        box self.client.request(req)
    }
}
use circles_common::db::AsyncPgPool;
use circles_common::db::query::*;
use circles_common::http;
use circles_common::http::FutureHandled;
use circles_common::http::ServerResponse;
use circles_common::http::header::UserID;
use circles_common::proto::positions::PositionUpdate;

use futures::{Future, Stream};
use futures::future::ok;

use hyper;
use hyper::{Method, Request, Response};
use hyper::server::Service;

use json;

use positions::error::Error;

use std::rc::Rc;

pub struct PositionsPostHandler {
    db_conn: Rc<AsyncPgPool>,
}

impl PositionsPostHandler {
    pub fn new(db_conn: Rc<AsyncPgPool>) -> Self {
        PositionsPostHandler { db_conn }
    }
}

impl Service for PositionsPostHandler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureHandled;

    fn call(&self, mut req: Request) -> Self::Future {
        let db_conn = self.db_conn.clone();

        // Absence of UserID header is definitely an internal server error
        let user_id = match req.headers_mut().remove::<UserID>() {
            Some(user_id) => user_id.0,
            None => {
                return box ok(
                    ServerResponse::from(http::error::ErrorKind::MissingUserIDHeader).into(),
                )
            }
        };

        // Panic in debug builds here to prevent wrong routing
        assert_eq!(req.method(), &Method::Post);
        assert_eq!(req.path(), "/positions");
        debug!("accepted POST request to /positions from {}", user_id);

        // Wait for hyper to fetch all body chunks
        let position_update = req.body().collect().map_err(Error::from).and_then(
            |chunks| {
                // Collect chunks into one vector of bytes
                let body = chunks
                    .into_iter()
                    .flat_map(IntoIterator::into_iter)
                    .collect::<Vec<u8>>();
                // Convert to a valid utf8 string
                String::from_utf8(body).map_err(Error::from).and_then(
                    |json_str| {
                        // Log received data
                        trace!("received a PositionUpdate: {:?}", json_str);
                        // Parse the body json into PositionUpdate
                        json::from_str::<PositionUpdate>(&json_str).map_err(Error::from)
                    },
                )
            },
        );

        box position_update
        // If everything've been successful so far
            .and_then(move |position_update| 
        {
            // Convert received update into a personalized PositionRecord
            let position_record = position_update.into_position_record(user_id);
            // Insert it into a DB
            position_record.insert(&db_conn).map_err(Error::from)
                // After successful insertion, send an empty JSON responce
                .and_then(|_| box ok(Response::new().with_body("{}")))
                // Or log the error and send ErrorResponce
                .or_else(|e| {
                    error!("{}", e);
                    box ok(ServerResponse::from(e).into())
                })
        })
        // Send client request's decoding error 
            .or_else(|e| box ok(ServerResponse::from(e).into()))
    }
}

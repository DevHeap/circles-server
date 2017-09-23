use hyper::{Request, Response};
use hyper::header::{Authorization, Bearer};
use hyper::server::{Service, NewService};
use hyper;
use futures::future;
use futures::Future;
use std::rc::Rc;
use std::collections::HashMap;
use std::io;

use firebase::Token;
use firebase::AsyncTokenVerifier;

use circles_router::router::Router;
use circles_router::router::FutureRoute;
use hyper_common::header::UserID;
use hyper_common::ErrorResponse;

use middleware::error::ErrorKind as MwErrorKind;
use middleware::error::Result as MwResult;
use middleware::error::Error as MwError;

use circles_common::db::AsyncPgPool;

/// Authenticator Service factory with "persistent" state
pub struct Authenticator {
    auth: Rc<AsyncTokenVerifier>,
    router: Rc<Router>,
    users_db_updater: Rc<UsersDbUpdater>,
}   

impl Authenticator {
    pub fn new(auth: Rc<AsyncTokenVerifier>, db: Rc<AsyncPgPool>, router: Rc<Router>) -> Self {
        info!("Created Authenticator (Service Factory)");
        Authenticator {
            auth,
            router,
            users_db_updater: Rc::new(UsersDbUpdater::new(db))
        }
    }
}

impl NewService for Authenticator {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = AuthenticatorService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        debug!("Created Authenticator Service");
        let service = AuthenticatorService {
            auth: self.auth.clone(),
            router: self.router.clone(),
            users_db_updater: self.users_db_updater.clone()
        };
        Ok(service)
    }
}

pub struct AuthenticatorService {
    auth: Rc<AsyncTokenVerifier>,
    router: Rc<Router>,
    users_db_updater: Rc<UsersDbUpdater>,
}

impl AuthenticatorService {
    fn extract_token(req: &Request) -> Result<String, ErrorResponse> {
        let headers = req.headers();
        let bearer: &Authorization<Bearer> = headers.get()
            .ok_or(ErrorResponse::from(MwErrorKind::AuthHeaderMissing))?;

        // @TODO can we avoid cloning here?
        Ok(bearer.token.clone())
    }
}

impl Service for AuthenticatorService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, mut req: Request) -> Self::Future {
        let uri = req.uri().clone();

        trace!("accepted {} request for {}", req.method(), req.uri());

        // Extract IDToken from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token,
            Err(error) => return box future::ok(error.into())
        };

        let router = self.router.new_service()
        // Can never happen. Really.
            .unwrap(); 

        let users_db = self.users_db_updater.clone();

        // Either pass the request to the Dispatcher or return error response to a client
        let future_response = self.auth.authenticate(token).map_err(MwError::from)
            .then(move |auth_result: MwResult<Token>| -> FutureRoute 
        {
            match auth_result {
                Ok(token) => {
                    debug!("authorized request from user {}", token.user_id());
                    
                    // Set UserID header
                    let uid = token.user_id().to_owned();
                    req.headers_mut().set(UserID(uid));

                    // Update users database table and proceed to the router 
                    let db_future = users_db.update_if_needed(&token).then(move |res| {
                        match res {
                            Ok(..) => router.call(req),
                            Err(e) => box future::ok(ErrorResponse::from(e).into())
                        }
                    });
                    
                    // Pass the request to dispatcher
                    box db_future
                },
                Err(e) => {
                    debug!("attempted unathorized access to {}", uri);
                    box future::ok(ErrorResponse::from(e).into())
                }
            }
        });

        box future_response
    }
}

use std::cell::RefCell;
use chrono::NaiveDateTime;

// Cached updater preventing unneeded sql request flood 
struct UsersDbUpdater {
    db: Rc<AsyncPgPool>,
    /// Mapping from user_id to token expiration time
    auth_table: RefCell<HashMap<String, NaiveDateTime>>
}

impl UsersDbUpdater {
    fn new(db: Rc<AsyncPgPool>) -> Self {
        UsersDbUpdater {
            db,
            auth_table: RefCell::new(HashMap::new())
        }
    }

    // Update DB only for users whose token expiration was not cached yet
    fn update_if_needed(&self, token: &Token) -> Box<Future<Item=(), Error=MwError>> {
        let user_id = token.user_id();
        let exp = NaiveDateTime::from_timestamp(token.expiration_time() as i64, 0);

        if Some(&exp) != self.auth_table.borrow().get(user_id) {
            debug!("no cached expiration entry for {}, adding & updating DB", user_id);
            self.auth_table.borrow_mut()
                .insert(user_id.to_owned(), exp);
            box self.update(token)
        } else {
            debug!("found cached expiration entry for {}, doing nothing", user_id);
            box future::ok(())
        }
    }

    fn update(&self, token: &Token) -> impl Future<Item=(), Error=MwError> {
        use circles_common::db::schema::users::dsl::*;
        use circles_common::db::models::User;
        use circles_common::db::error::Error;
        use futures::future::result;
        use diesel::insert;
        use diesel::prelude::*;
        use diesel::pg::upsert::*;

        let user = User::from(token);
        let user_id = user.uid.clone();

        debug!("started updating users db table for {}", token.user_id());

        // Insert an authentified user or, if user exists, just update 
        let db_future = self.db.request(move |conn| {
            result(
                insert(
                    &user.on_conflict(uid, do_update().set(&user.auth_data()))
                ).into(users)
                 .execute(&*conn)
                 .map_err(Error::from)
            )    
        });

        db_future.map_err(MwError::from)
            .then(move |result| {
                match result {
                    Ok(ref rows) => debug!("successfully updated {} rows for user {}", rows, user_id),
                    Err(ref e)   => error!("failed to update db for user {}: {}", user_id, e)
                }
                result.map(|_|())
            })
    }
}


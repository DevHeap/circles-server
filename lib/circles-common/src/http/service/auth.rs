//! Request authentication proxy middleware

use db::AsyncPgPool;
use firebase::AsyncTokenVerifier;

use firebase::Token;
use futures::Future;
use futures::future;
use http::ApiError;

use http::FutureHandled;
use http::HandlerFactory;
use http::ServerResponse;
use http::error::Error;
use http::error::ErrorKind;
use http::header::UserID;

use hyper;
use hyper::{Request, Response};
use hyper::header::{Authorization, Bearer};
use hyper::server::{Service, NewService};

use std::collections::HashMap;
use std::io;
use std::rc::Rc;

/// Authenticator Service factory with "persistent" state
///
/// For usage example please refer to one of already implemented microservices
pub struct Authenticator {
    auth: Rc<AsyncTokenVerifier>,
    next_chain: Rc<HandlerFactory>,
    users_db_updater: Rc<UsersDbUpdater>,
}

impl Authenticator {
    /// Create a new AuthenticatorService factory with persistent state
    pub fn new(db: Rc<AsyncPgPool>, next_chain: Rc<HandlerFactory>) -> Self {
        info!("Created Authenticator (Service Factory)");
        Authenticator {
            auth: Rc::new(AsyncTokenVerifier::new()),
            next_chain,
            users_db_updater: Rc::new(UsersDbUpdater::new(db)),
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
            next_chain: self.next_chain.clone(),
            users_db_updater: self.users_db_updater.clone(),
        };
        Ok(service)
    }
}

/// AuthenticatorService is responsible for tokens verification
/// and populating the database with user info
pub struct AuthenticatorService {
    auth: Rc<AsyncTokenVerifier>,
    next_chain: Rc<HandlerFactory>,
    users_db_updater: Rc<UsersDbUpdater>,
}

impl AuthenticatorService {
    fn extract_token(req: &Request) -> Result<&str, ApiError> {
        let headers = req.headers();
        let bearer: &Authorization<Bearer> = headers.get().ok_or(ApiError::from(
            ErrorKind::AuthHeaderMissing,
        ))?;

        Ok(&bearer.token)
    }
}

impl Service for AuthenticatorService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, mut req: Request) -> Self::Future {
        trace!("accepted {} request for {}", req.method(), req.uri());
        trace!("headers: {:?}", req.headers());

        // Extract IDToken from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token.to_owned(),
            Err(error) => return box future::ok(ServerResponse::from(error).into()),
        };

        let next_chain = self.next_chain.new_service()
        // Can never happen. Really.
            .unwrap();

        let users_db = self.users_db_updater.clone();

        // Either pass the request to the Dispatcher or return error response to a client
        let future_response = self.auth.authenticate(token).map_err(Error::from).then(
            move |auth_result| -> FutureHandled {
                match auth_result {
                    Ok(token) => {
                        debug!("authorized request from user {}", token.user_id());

                        // Set UserID header
                        let uid = token.user_id().to_owned();
                        req.headers_mut().set(UserID(uid));

                        // Update users database table and proceed to the router
                        let db_future = users_db.update_if_needed(&token).then(
                            move |res| match res {
                                Ok(..) => next_chain.call(req),
                                Err(e) => box future::ok(ServerResponse::from(e).into()),
                            },
                        );

                        // Pass the request to dispatcher
                        box db_future
                    }
                    Err(e) => {
                        debug!("attempted unathorized access to {}", req.path());
                        box future::ok(ServerResponse::from(e).into())
                    }
                }
            },
        );

        box future_response
    }
}

use chrono::NaiveDateTime;
use std::cell::RefCell;

// Cached updater preventing unneeded sql request flood
struct UsersDbUpdater {
    db: Rc<AsyncPgPool>,
    /// Mapping from user_id to token expiration time
    auth_table: RefCell<HashMap<String, NaiveDateTime>>,
}

impl UsersDbUpdater {
    fn new(db: Rc<AsyncPgPool>) -> Self {
        UsersDbUpdater {
            db,
            auth_table: RefCell::new(HashMap::new()),
        }
    }

    // Update DB only for users whose token expiration was not cached yet
    fn update_if_needed(&self, token: &Token) -> Box<Future<Item = (), Error = Error>> {
        let user_id = token.user_id();
        let exp = NaiveDateTime::from_timestamp(token.expiration_time() as i64, 0);

        if Some(&exp) != self.auth_table.borrow().get(user_id) {
            debug!(
                "no cached expiration entry for {}, adding & updating DB",
                user_id
            );
            self.auth_table.borrow_mut().insert(user_id.to_owned(), exp);
            box self.update(token)
        } else {
            debug!(
                "found cached expiration entry for {}, doing nothing",
                user_id
            );
            box future::ok(())
        }
    }

    fn update(&self, token: &Token) -> impl Future<Item = (), Error = Error> {
        use db::models::User;
        use db::query::Insert;

        let user = User::from(token);
        let user_id = user.uid.clone();

        user.insert(&self.db)
            .then(move |result| {
                match result {
                    Ok(ref rows) => {
                        debug!("successfully updated {} rows for user {}", rows, user_id)
                    }
                    Err(ref e) => error!("failed to update db for user {}: {}", user_id, e),                
                }
                result.map(|_| ())
            })
            .map_err(Error::from)
    }
}

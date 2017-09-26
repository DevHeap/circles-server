//! Ready-to-use Hyper Services

#[macro_use]
mod router;
mod auth;

pub use self::auth::Authenticator;

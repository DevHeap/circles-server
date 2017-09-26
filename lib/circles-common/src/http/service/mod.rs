//! Ready-to-use Hyper Services

#[macro_use]
mod router;
mod auth;
mod health;

pub use self::auth::Authenticator;
pub use self::health::Health;
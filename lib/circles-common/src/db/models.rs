
//! Data types reflecting actual database tables schema

use chrono::NaiveDateTime;
use db::schema::*;
use firebase::Token;

/// User model for the "users" table
#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "users"]
#[primary_key(uid)]
pub struct User {
    /// User unique identifier from Google Firebase API
    pub uid: String,
    /// Username
    pub username: Option<String>,
    /// Uri of userpic
    pub picture: Option<String>,
    /// User email
    pub email: Option<String>,
    /// Firebase token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Firebase token expiration time
    pub auth_until: NaiveDateTime,
}

/// Auth data changeset: issue time and expiration time
#[derive(Debug, Copy, Clone, AsChangeset)]
#[table_name = "users"]
pub struct UserAuthData {
    /// Firebase token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Firebase token expiration time
    pub auth_until: NaiveDateTime,
}

impl<'a> From<&'a Token> for User {
    fn from(token: &'a Token) -> Self {
        let get_string = |key| {
            token.payload.get(key).and_then(|json| {
                json.as_str().map(str::to_owned)
            })
        };

        User {
            uid: token.user_id().to_owned(),
            username: get_string("name"),
            picture: get_string("picture"),
            email: get_string("email"),
            auth_time: NaiveDateTime::from_timestamp(token.issued_at() as i64, 0),
            auth_until: NaiveDateTime::from_timestamp(token.expiration_time() as i64, 0),
        }
    }
}

impl User {
    /// Get AuthData of a User
    pub fn auth_data(&self) -> UserAuthData {
        UserAuthData {
            auth_time: self.auth_time,
            auth_until: self.auth_until,
        }
    }
}

/// PositionResord model for the "users" table
#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "position_records"]
#[primary_key(time, user_uid)]
pub struct PositionRecord {
    /// UTC time of GPS fix (from client)
    pub time: NaiveDateTime,
    /// User unique identifier from Google Firebase API
    pub user_uid: String,
    /// Latitude, in degrees.
    pub latitude: f64,
    /// Longitude, in degrees.
    pub longitude: f64,
    /// Estimated horizontal accuracy of this location, radial, in meters.
    pub accuracy: Option<f32>,
    /// Altitude in meters above the WGS 84 reference ellipsoid.
    pub altitude: Option<f64>,
    /// Estimated vertical accuracy of this location, in meters.
    pub vertical_accuracy: Option<f32>,
    /// Bearing, in degrees.
    pub bearing: Option<f32>,
    /// Speed in meters/second over ground.
    pub speed: Option<f32>,
    /// Estimated speed accuracy of this location, in meters per second.
    pub speed_accuracy: Option<f32>,
    /// Human-readable location name
    pub location: Option<String>,
}

//! JSON proto models for the /positions api

use db::models::PositionRecord;
use chrono::NaiveDateTime;

/// JSON model for position update from user
#[derive(Debug, Deserialize)]
pub struct PositionUpdate {
    time: NaiveDateTime,
    latitude: f64,
    longitude: f64,
    accuracy: Option<f32>,
    altitude: Option<f64>,
    vertical_accuracy: Option<f32>,
    bearing: Option<f32>,
    speed: Option<f32>,
    speed_accuracy: Option<f32>,
    location: Option<String>,
}

impl PositionUpdate {
    /// Convert PositionUpdate to a PositionRecord database model
    pub fn into_position_record(self, user_uid: String) -> PositionRecord {
        PositionRecord {
            time: self.time,
            user_uid: user_uid,
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy: self.accuracy,
            altitude: self.altitude,
            vertical_accuracy: self.vertical_accuracy,
            bearing: self.bearing,
            speed: self.speed,
            speed_accuracy: self.speed_accuracy,
            location: self.location,
        }
    }
}

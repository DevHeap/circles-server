#![allow(non_snake_case, missing_docs, unused_qualifications)]

//! Diesel generated schema and tables access DSL

table! {
    position_records (time, user_uid) {
        time -> Timestamp,
        user_uid -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        accuracy -> Nullable<Float4>,
        altitude -> Nullable<Float8>,
        vertical_accuracy -> Nullable<Float4>,
        bearing -> Nullable<Float4>,
        speed -> Nullable<Float4>,
        speed_accuracy -> Nullable<Float4>,
        location -> Nullable<Varchar>,
    }
}

table! {
    users (uid) {
        uid -> Varchar,
        username -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        auth_time -> Timestamp,
        auth_until -> Timestamp,
    }
}

#![allow(non_snake_case)]

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


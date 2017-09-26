//! Convenience traits for easy querying

use db::AsyncPgPool;
use db::error::Error;
use db::models::*;

use diesel;
use diesel::ExecuteDsl;

use futures_cpupool::CpuFuture;

/// Convenient trait to simplify object insertion
pub trait Insert {
    /// Insertion consumes the object Self and returns a future numbers of changed rows
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<usize, Error>;
}

impl Insert for PositionRecord {
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<usize, Error> {
        use db::schema::position_records;

        pool.request(move |conn| {
            diesel::insert(&self)
                .into(position_records::table)
                .execute(&*conn)
                .map_err(Error::from)
        })
    }
}

impl Insert for User {
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<usize, Error> {
        use db::schema::users::dsl::*;
        use futures::future::result;
        use diesel::insert;
        use diesel::prelude::*;
        use diesel::pg::upsert::*;

        // Insert an authentified user or, if user exists, just update
        pool.request(move |conn| {
            result(
                insert(&self.on_conflict(uid, do_update().set(&self.auth_data())))
                    .into(users)
                    .execute(&*conn)
                    .map_err(Error::from),
            )
        })
    }
}

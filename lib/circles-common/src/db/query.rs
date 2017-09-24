use db::AsyncPgPool;
use futures_cpupool::CpuFuture;
use db::error::Error;
use db::models::*;
use diesel;
use diesel::ExecuteDsl;

pub trait Insert {
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<usize, Error>;
}

impl Insert for PositionRecord {
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<usize, Error> {
        use db::schema::position_records;

        pool.request(move |conn| {
            diesel::insert(&self).into(position_records::table)
                .execute(&*conn)
                .map_err(Error::from)
        })
    }
}
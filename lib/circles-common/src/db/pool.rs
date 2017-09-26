//! Database connection pool

use db::Error;
use db::Result;

use diesel::pg::PgConnection;

use r2d2;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

/// Synchronous PgConnection pool
pub type SyncPgPool = Pool<ConnectionManager<PgConnection>>;

/// Pooled connection from Synchronous PgConnection pool
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Connect to PostgreSQL DMBS and produce a sync connection pool
pub fn connect(db_uri: &str) -> Result<SyncPgPool> {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_uri);
    Ok(r2d2::Pool::new(config, manager)?)
}

/// Async PgConnection pool build on top of sync connection pool and a thread pool
pub struct AsyncPgPool {
    conn_pool: SyncPgPool,
    cpu_pool: CpuPool,
}

use futures::Future;
use futures::future::IntoFuture;
use futures::future::result;
use futures_cpupool::CpuFuture;
use futures_cpupool::CpuPool;

impl AsyncPgPool {
    /// Construct AsyncPgPool from SyncPgPool
    pub fn new(conn_pool: SyncPgPool) -> Self {
        AsyncPgPool {
            conn_pool,
            cpu_pool: CpuPool::new_num_cpus(),
        }
    }

    /// Connect to PostgreSql DBMS and produce an AsyncPgPool
    pub fn connect(db_uri: &str) -> Result<Self> {
        Ok(Self::new(connect(db_uri)?))
    }

    /// Execute a request with pooled connection from AsyncPgPool.
    /// Returns the future with a query result
    pub fn request<F, R>(&self, closure: F) -> CpuFuture<R::Item, Error>
    where
        F: FnOnce(PgPooledConnection) -> R + Send + 'static,
        R: IntoFuture<Error = Error> + 'static,
        R::Future: Send + 'static,
        R::Item: Send + 'static,
    {
        let conn_pool = self.conn_pool.clone();
        self.cpu_pool.spawn_fn(move || {
            result(conn_pool.get().map_err(Error::from))
                .and_then(closure)
        })
    }
}

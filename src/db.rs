use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::*;
use r2d2::{Pool, PooledConnection};

use crate::util::env_vars::{DB_MAX_POOL_SIZE, DB_URL};

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = {
        let manager = ConnectionManager::new(&*DB_URL);
        let pool = Pool::builder()
            .max_size(*DB_MAX_POOL_SIZE)
            .build(manager)
            .unwrap();

        pool
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    DB_CONNECTION_POOL
        .get()
        .expect(&format!("Error connecting to {}", *DB_URL))
}

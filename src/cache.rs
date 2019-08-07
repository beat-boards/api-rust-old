use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use lazy_static::*;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use serde_json;

use crate::db;
use crate::models::users::User;
use crate::schema::users::dsl::*;
use crate::util::env_vars::{CACHE_MAX_POOL_SIZE, CACHE_URL};

lazy_static! {
    static ref CACHE_CONNECTION_POOL: Pool<RedisConnectionManager> = {
        let manager = RedisConnectionManager::new(&(*CACHE_URL)[..]).unwrap();
        let pool = Pool::builder()
            .max_size(*CACHE_MAX_POOL_SIZE)
            .build(manager)
            .unwrap();

        pool
    };
}

pub fn establish_connection() -> PooledConnection<RedisConnectionManager> {
    CACHE_CONNECTION_POOL
        .get()
        .expect(&format!("Error connecting to {}", *CACHE_URL))
}

pub fn update_cache() {
    let mut cache_conn = establish_connection();
    let db_conn = db::establish_connection();

    let db_list = users
        .order((rp.desc(), id.asc()))
        .limit(1000)
        .load::<User>(&db_conn)
        .expect("Querying users from database for caching");

    let db_list =
        serde_json::to_string(&db_list).expect("Serializing users from databse for caching");

    let cache_list: String = cache_conn.get("bbapi:users").unwrap_or(String::from(""));

    if cache_list != db_list {
        println!("Updating cache...");
        let _: () = cache_conn
            .set("bbapi:users", &db_list)
            .expect("Updating cache");
    }
}

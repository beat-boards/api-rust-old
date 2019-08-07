use lazy_static::*;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use serde::Serialize;
use serde_json;

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

fn update_cache<T: Serialize>(
    list: Vec<T>,
    cache_conn: &mut PooledConnection<RedisConnectionManager>,
    chunk_size: usize,
    name: &str,
) {
    for (i, chunk) in list.chunks(chunk_size).enumerate() {
        let cache_key = format!("bbapi:{}:{}", name, i);

        let db_str = serde_json::to_string(&chunk)
            .expect(&format!("Serializing {} from databse for caching", name));
        let cache_str: String = cache_conn.get(&cache_key).unwrap_or(String::from(""));

        if cache_str != db_str {
            println!("Updating {} cache...", name);
            let _: () = cache_conn.set(&cache_key, &db_str).expect("Updating cache");
        }
    }
}

pub mod users {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    use crate::cache;
    use crate::db;
    use crate::models::users::User;
    use crate::schema::users::dsl::*;

    pub fn update_cache() {
        let mut cache_conn = cache::establish_connection();
        let db_conn = db::establish_connection();

        let db_users = users
            .order((rp.desc(), id.asc()))
            .limit(1000)
            .load::<User>(&db_conn)
            .expect("Querying users from database for caching");

        cache::update_cache(db_users, &mut cache_conn, 50, "users");
    }
}

pub mod maps {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    use crate::cache;
    use crate::db;
    use crate::models::maps::Map;
    use crate::schema::maps::dsl::*;

    pub fn update_cache() {
        let mut cache_conn = cache::establish_connection();
        let db_conn = db::establish_connection();

        let db_maps = maps
            .order((max_rp.desc(), id.asc()))
            .limit(500)
            .load::<Map>(&db_conn)
            .expect("Querying maps from database for caching");

        cache::update_cache(db_maps, &mut cache_conn, 50, "maps");
    }
}

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use lazy_static::*;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use std::env;

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DB_URL").expect("DB_URL must be set");
        let manager = ConnectionManager::new(database_url);
        let database_max_pool_size: u32 = env::var("DB_MAX_POOL_SIZE")
            .unwrap_or(String::from("8"))
            .parse()
            .expect("DB_MAX_POOL_SIZE must be an unsigned integer");
        let pool = Pool::builder()
            .max_size(database_max_pool_size)
            .build(manager)
            .unwrap();

        pool
    };
}

lazy_static! {
    static ref CACHE_CONNECTION_POOL: Pool<RedisConnectionManager> = {
        dotenv().ok();

        let cache_url = env::var("CACHE_URL").expect("CACHE_URL must be set");
        let manager = RedisConnectionManager::new(&cache_url[..]).unwrap();
        let cache_max_pool_size: u32 = env::var("CACHE_MAX_POOL_SIZE")
            .unwrap_or(String::from("8"))
            .parse()
            .expect("CACHE_MAX_POOL_SIZE must be an unsigned integer");
        let pool = Pool::builder()
            .max_size(cache_max_pool_size)
            .build(manager)
            .unwrap();

        pool
    };
}

pub mod db {
    use diesel::pg::PgConnection;
    use diesel::r2d2::ConnectionManager;
    use r2d2::PooledConnection;
    use std::env;

    use crate::util::DB_CONNECTION_POOL;

    pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        let database_url = env::var("DB_URL").expect("DB_URL must be set");

        DB_CONNECTION_POOL
            .get()
            .expect(&format!("Error connecting to {}", database_url))
    }
}

pub mod cache {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use r2d2::PooledConnection;
    use r2d2_redis::RedisConnectionManager;
    use r2d2_redis::redis::Commands;
    use std::env;
    use serde_json;

    use crate::util::CACHE_CONNECTION_POOL;
    use crate::util::db;
    use crate::schema::users::dsl::*;
    use crate::models::users::User;

    pub fn establish_connection() -> PooledConnection<RedisConnectionManager> {
        let cache_url = env::var("CACHE_URL").expect("CACHE_URL must be set");

        CACHE_CONNECTION_POOL
            .get()
            .expect(&format!("Error connecting to {}", cache_url))
    }

    pub fn update_cache() {
        let mut cache_conn = establish_connection();
        let db_conn = db::establish_connection();

        let db_list = users
            .order((rp.desc(), id.asc()))
            .limit(1000)
            .load::<User>(&db_conn)
            .expect("Querying users from database for caching");

        let db_list = serde_json::to_string(&db_list)
            .expect("Serializing users from databse for caching");

        let cache_list: String = cache_conn.get("bbapi:users").unwrap_or(String::from(""));

        if cache_list != db_list {
            println!("Updating cache...");
            let _: () = cache_conn.set("bbapi:users", &db_list).expect("Updating cache");
        }
    }
}

pub mod error {
    use crate::context::Ctx;
    use serde_json;

    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone)]
    #[repr(u32)]
    pub enum HttpErrorCode {
        BadRequest = 400,
        Unauthorized = 401,
        Forbidden = 403,
        NotFound = 404,
        MethodNotAllowed = 405,
        Gone = 410,
        PayloadTooLarge = 413,
        InternalServerError = 500,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all(serialize = "camelCase"))]
    pub struct HttpError {
        code: HttpErrorCode,
        message: String,
        details: String,
    }

    impl HttpError {
        pub fn to_string(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
        pub fn set_context(&self, ctx: &mut Ctx) {
            ctx.content_type("application/json");
            ctx.status((&self.code).clone() as u32);
            ctx.body(&self.to_string());
        }

        pub fn bad_request(details: &str) -> HttpError {
            HttpError {
                code: HttpErrorCode::BadRequest,
                message: String::from("Bad Request"),
                details: String::from(details),
            }
        }
        pub fn not_found(details: &str) -> HttpError {
            HttpError {
                code: HttpErrorCode::NotFound,
                message: String::from("Not Found"),
                details: String::from(details),
            }
        }
        pub fn internal_server_error(details: &str) -> HttpError {
            HttpError {
                code: HttpErrorCode::InternalServerError,
                message: String::from("Internal Server Error"),
                details: String::from(details),
            }
        }
    }
}

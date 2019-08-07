use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use lazy_static::*;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use std::env;

pub mod env_vars {
    use lazy_static::*;
    use dotenv::dotenv;
    use std::env;

    lazy_static! {
        pub static ref DB_URL: String = {
            dotenv().ok();
            env::var("DB_URL").expect("Missing environment variable DB_URL")
        };

        pub static ref CACHE_URL: String = {
            dotenv().ok();
            env::var("CACHE_URL").expect("Missing environment variable CACHE_URL")
        };

        pub static ref DB_MAX_POOL_SIZE: u32 = {
            dotenv().ok();
            env::var("DB_MAX_POOL_SIZE").expect("Missing environment variable DB_MAX_POOL_SIZE").parse::<u32>().expect("DB_MAX_POOL_SIZE must be an unsigned integer")
        };

        pub static ref CACHE_MAX_POOL_SIZE: u32 = {
            dotenv().ok();
            env::var("CACHE_MAX_POOL_SIZE").expect("Missing environment variable CACHE_MAX_POOL_SIZE").parse::<u32>().expect("CACHE_MAX_POOL_SIZE must be an unsigned integer")
        };

        pub static ref HOST: String = {
            dotenv().ok();
            env::var("HOST").expect("Missing environment variable HOST")
        };

        pub static ref PORT: u16 = {
            dotenv().ok();
            env::var("PORT").expect("Missing environment variable PORT").parse::<u16>().expect("PORT must be an unsigned integer")
        };
    }
}

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = {
        let manager = ConnectionManager::new(&*env_vars::DB_URL);
        let pool = Pool::builder()
            .max_size(*env_vars::DB_MAX_POOL_SIZE)
            .build(manager)
            .unwrap();

        pool
    };

    static ref CACHE_CONNECTION_POOL: Pool<RedisConnectionManager> = {
        let manager = RedisConnectionManager::new(&(*env_vars::CACHE_URL)[..]).unwrap();
        let pool = Pool::builder()
            .max_size(*env_vars::CACHE_MAX_POOL_SIZE)
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
    use crate::util::env_vars::DB_URL;

    pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DB_CONNECTION_POOL
            .get()
            .expect(&format!("Error connecting to {}", *DB_URL))
    }
}

pub mod cache {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use r2d2::PooledConnection;
    use r2d2_redis::redis::Commands;
    use r2d2_redis::RedisConnectionManager;
    use serde_json;
    use std::env;

    use crate::models::users::User;
    use crate::schema::users::dsl::*;
    use crate::util::db;
    use crate::util::CACHE_CONNECTION_POOL;
    use crate::util::env_vars::CACHE_URL;

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

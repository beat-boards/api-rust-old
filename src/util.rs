use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use lazy_static::*;
use r2d2::Pool;
use std::env;

lazy_static! {
    static ref CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::new(database_url);
        let database_max_pool_size: u32 = env::var("DATABASE_MAX_POOL_SIZE")
            .unwrap_or(String::from("16"))
            .parse()
            .expect("DATABASE_MAX_POOL_SIZE must be an unsigned integer");
        let pool = r2d2::Pool::builder()
            .max_size(database_max_pool_size)
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

    use crate::util::CONNECTION_POOL;

    pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        CONNECTION_POOL
            .get()
            .expect(&format!("Error connecting to {}", database_url))
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

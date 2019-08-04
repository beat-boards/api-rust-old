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
    pub enum ErrorCode {
        Request = 400,
        NotFound = 404,
        Internal = 500,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all(serialize = "camelCase"))]
    pub struct Error {
        code: ErrorCode,
        message: String,
    }

    impl Error {
        pub fn to_string(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
        pub fn set_context(&self, ctx: &mut Ctx) {
            ctx.content_type("application/json");
            ctx.status((&self.code).clone() as u32);
            ctx.body(&self.to_string());
        }

        pub fn request_error() -> Error {
            Error {
                code: ErrorCode::Request,
                message: String::from("Invalid request"),
            }
        }
        pub fn internal_error() -> Error {
            Error {
                code: ErrorCode::Internal,
                message: String::from("Database error"),
            }
        }
        pub fn not_found_error() -> Error {
            Error {
                code: ErrorCode::NotFound,
                message: String::from("Requested resource not found"),
            }
        }
    }
}

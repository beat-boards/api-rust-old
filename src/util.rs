pub mod env_vars {
    use dotenv::dotenv;
    use lazy_static::*;
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
            env::var("DB_MAX_POOL_SIZE")
                .expect("Missing environment variable DB_MAX_POOL_SIZE")
                .parse::<u32>()
                .expect("DB_MAX_POOL_SIZE must be an unsigned integer")
        };
        pub static ref CACHE_MAX_POOL_SIZE: u32 = {
            dotenv().ok();
            env::var("CACHE_MAX_POOL_SIZE")
                .expect("Missing environment variable CACHE_MAX_POOL_SIZE")
                .parse::<u32>()
                .expect("CACHE_MAX_POOL_SIZE must be an unsigned integer")
        };
        pub static ref HOST: String = {
            dotenv().ok();
            env::var("HOST").expect("Missing environment variable HOST")
        };
        pub static ref PORT: u16 = {
            dotenv().ok();
            env::var("PORT")
                .expect("Missing environment variable PORT")
                .parse::<u16>()
                .expect("PORT must be an unsigned integer")
        };
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

pub mod query_string {
    use std::collections::HashMap;

    fn get_i64(query_params: &HashMap<String, String>, name: &str, default: i64) -> i64 {
        query_params
            .get(name)
            .unwrap_or(&format!("{}", &default))
            .parse()
            .unwrap_or(default)
    }

    pub fn get_offset_and_limit(query_params: &HashMap<String, String>) -> (i64, i64) {
        (
            get_i64(query_params, "offset", 0),
            get_i64(query_params, "limit", 50),
        )
    }
}

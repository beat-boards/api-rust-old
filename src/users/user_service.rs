use diesel;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use r2d2_redis::redis::Commands;
use uuid::Uuid;

use crate::cache;
use crate::db;
use crate::models::users::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;

pub struct Filters<'a> {
    pub steam_id: Option<i64>,
    pub oculus_id: Option<&'a String>,
}

pub fn get_users(limit: i64, filters: Filters) -> Result<Vec<User>, Error> {
    let conn = db::establish_connection();

    let mut query = users.into_boxed();
    if let Some(f_sid) = &filters.steam_id {
        query = query.filter(steam_id.eq(f_sid.clone()));
    }
    if let Some(f_oid) = &filters.oculus_id {
        query = query.filter(oculus_id.eq(f_oid.clone()));
    }
    query
        .order((rp.desc(), id.asc()))
        .limit(limit)
        .load::<User>(&conn)
}

pub fn get_cached_users(limit: i64, _filters: Filters) -> String {
    let mut result = String::new();
    let mut conn = cache::establish_connection();
    let chunk_qty = limit / 50; // Amount of 50 users chunks to get from the cache

    // Query cache for each chunk
    for i in 0..chunk_qty {
        let mut cache_str: String = conn
            .get(&format!("bbapi:users:{}", i))
            .unwrap_or(String::from(""));

        // Less users in cache than limit, finish up
        if cache_str == "" {
            if result.pop() != Some(',') {
                panic!("Corrupted cache");
            }
            result.push(']');
            break;
        }

        // Remove opening brackets except on the first chunk
        if i != 0 {
            if cache_str.remove(0) != '[' {
                panic!("Corrupted cache");
            }
        }

        // Replace closing brackets with a comma except on the last chunk
        if i != chunk_qty - 1 {
            if cache_str.pop() != Some(']') {
                panic!("Corrupted cache");
            }
            cache_str.push(',');
        }

        result += &cache_str;
    }

    result
}

pub fn create_user(new_user: NewUser) -> Result<User, Error> {
    let conn = db::establish_connection();

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn)
}

pub fn get_user(identifier: Uuid) -> Result<User, Error> {
    let conn = db::establish_connection();
    users.find(identifier).first(&conn)
}

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

pub mod rank {
    use crate::models::users::{RankedUser, User};

    pub fn rank_users(users: Vec<User>, offset: i64) -> Vec<RankedUser> {
        let mut ranked_users: Vec<RankedUser> = Vec::new();

        for (i, user) in users.into_iter().enumerate() {
            let rank = 1 + offset as u64 + i as u64;
            let User {
                id,
                steam_id,
                oculus_id,
                banned,
                username,
                role,
                country,
                rp,
                fails,
                following,
                image,
            } = user;

            ranked_users.push(RankedUser {
                id,
                rank,
                steam_id,
                oculus_id,
                banned,
                username,
                role,
                country,
                rp,
                fails,
                following,
                image,
            });
        }

        ranked_users
    }
}

pub fn get_users(offset: i64, limit: i64, filters: Filters) -> Result<Vec<User>, Error> {
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
        .offset(offset)
        .limit(limit)
        .load::<User>(&conn)
}

pub fn get_cached_users(offset: i64, limit: i64, _rank: bool, _filters: Filters) -> String {
    let mut result = String::new();
    let mut conn = cache::establish_connection();
    let chunk_offset = offset / 50;
    let chunk_limit = limit / 50; // Amount of 50 users chunks to get from the cache

    // Query cache for each chunk
    for i in chunk_offset..chunk_limit {
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
        if i != chunk_offset {
            if cache_str.remove(0) != '[' {
                panic!("Corrupted cache");
            }
        }

        // Replace closing brackets with a comma except on the last chunk
        if i != chunk_limit - 1 {
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

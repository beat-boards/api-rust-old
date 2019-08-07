use diesel;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

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

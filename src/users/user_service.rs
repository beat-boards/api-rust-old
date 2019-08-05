use diesel;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::models::users::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::util::db;

pub fn get_users(limit: i64) -> Result<Vec<User>, Error> {
    let conn = db::establish_connection();

    let query = users
        .order((rp.desc(), id.asc()))
        .limit(limit)
        .load::<User>(&conn);

    query
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

use diesel;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db;
use crate::models::scores::{NewScore, Score};
use crate::schema::scores;
use crate::schema::scores::dsl::*;

pub fn create_score(new_score: NewScore) -> Result<Score, Error> {
    let conn = db::establish_connection();

    diesel::insert_into(scores::table)
        .values(&new_score)
        .get_result(&conn)
}

pub fn get_score(identifier: Uuid) -> Result<Score, Error> {
    let conn = db::establish_connection();
    scores.find(identifier).first(&conn)
}

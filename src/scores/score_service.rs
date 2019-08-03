use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::result::Error;
use uuid::Uuid;

use crate::util::db;
use crate::models::scores::{ Score, NewScore };
use crate::schema::scores;
use crate::schema::scores::dsl::*;

pub fn create_score(new_score: NewScore) -> Result<Score, Error> {
  let conn = db::establish_connection();

  let score = diesel::insert_into(scores::table)
    .values(&new_score)
    .get_result(&conn);

  score
}

pub fn get_score(identifier: Uuid) -> Result<Score, Error> {
  let conn = db::establish_connection();

  let score = scores.find(identifier)
    .first(&conn);

  score
}

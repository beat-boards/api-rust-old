use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::result::Error;
use uuid::Uuid;

use crate::util::db;
use crate::models::maps::{ Map, NewMap };
use crate::schema::maps;
use crate::schema::maps::dsl::*;

pub fn create_map(new_map: NewMap) -> Result<Map, Error> {
  let conn = db::establish_connection();

  let map = diesel::insert_into(maps::table)
    .values(&new_map)
    .get_result(&conn);

  map
}

pub fn get_map(identifier: Uuid) -> Result<Map, Error> {
  let conn = db::establish_connection();

  let map = maps.find(identifier)
    .first(&conn);

  map
}

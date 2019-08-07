use diesel;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db;
use crate::models::maps::{Map, NewMap};
use crate::schema::maps;
use crate::schema::maps::dsl::*;

pub struct Filters<'a> {
    pub hash: Option<&'a String>,
}

pub fn get_maps(limit: i64, filters: Filters) -> Result<Vec<Map>, Error> {
    let conn = db::establish_connection();

    let mut query = maps.into_boxed();
    if let Some(f_hash) = &filters.hash {
        query = query.filter(hash.eq(f_hash.clone()));
    }
    query
        .order((max_rp.desc(), id.asc()))
        .limit(limit)
        .load::<Map>(&conn)
}

pub fn create_map(new_map: NewMap) -> Result<Map, Error> {
    let conn = db::establish_connection();

    diesel::insert_into(maps::table)
        .values(&new_map)
        .get_result(&conn)
}

pub fn get_map(identifier: Uuid) -> Result<Map, Error> {
    let conn = db::establish_connection();
    maps.find(identifier).first(&conn)
}

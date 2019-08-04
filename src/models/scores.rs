use crate::schema::scores;
use crate::schema::Modifier;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Score {
    pub id: Uuid,
    pub user: Uuid,
    pub map: Uuid,
    pub date: NaiveDateTime,
    pub raw_score: i32,
    pub raw_percentage: f64,
    pub modifiers: Vec<Modifier>,
    pub adjusted_score: i32,
    pub raw_rp: f64,
    pub adjusted_rp: f64,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name = "scores"]
pub struct NewScore {
    pub user: Uuid,
    pub map: Uuid,
    pub date: NaiveDateTime,
    pub raw_score: i32,
    pub raw_percentage: f64,
    pub modifiers: Vec<Modifier>,
    pub adjusted_score: i32,
    pub raw_rp: f64,
    pub adjusted_rp: f64,
}

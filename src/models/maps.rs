use crate::schema::maps;
use crate::schema::Difficulty;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Map {
    pub id: Uuid,
    pub hash: String,
    pub difficulty: Difficulty,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub difficulty_rating: f64,
    pub length: f64,
    pub bpm: f64,
    pub note_jump_speed: f64,
    pub note_count: i32,
    pub complexity: f64,
    pub saber_distance: f64,
    pub max_rp: f64,
    pub upvotes: i32,
    pub downvotes: i32,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name = "maps"]
pub struct NewMap {
    pub hash: String,
    pub difficulty: Difficulty,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub difficulty_rating: f64,
    pub length: f64,
    pub bpm: f64,
    pub note_jump_speed: f64,
    pub note_count: i32,
    pub complexity: f64,
    pub saber_distance: f64,
    pub max_rp: f64,
}

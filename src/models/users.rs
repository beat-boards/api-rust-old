use crate::schema::users;
use crate::schema::Role;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct User {
    pub id: Uuid,
    pub steam_id: Option<i64>,
    pub oculus_id: Option<String>,
    pub banned: bool,
    pub username: String,
    pub role: Role,
    pub country: String,
    pub rp: f64,
    pub fails: i32,
    pub following: Vec<Uuid>,
    pub image: Option<String>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub steam_id: Option<i64>,
    pub oculus_id: Option<&'a str>,
    pub username: &'a str,
    pub country: &'a str,
    pub image: Option<&'a str>,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RankedUser {
    pub id: Uuid,
    pub rank: u64,
    pub steam_id: Option<i64>,
    pub oculus_id: Option<String>,
    pub banned: bool,
    pub username: String,
    pub role: Role,
    pub country: String,
    pub rp: f64,
    pub fails: i32,
    pub following: Vec<Uuid>,
    pub image: Option<String>,
}

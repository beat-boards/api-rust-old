use crate::schema::users;
use crate::schema::Role;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
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
#[table_name = "users"]
pub struct NewUser {
    pub steam_id: Option<i64>,
    pub oculus_id: Option<String>,
    pub username: String,
    pub country: String,
    pub image: Option<String>,
}

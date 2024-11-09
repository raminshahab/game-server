use diesel::{Queryable, Insertable};
use rocket::serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub vip_level: String,
    pub new_user: bool
}
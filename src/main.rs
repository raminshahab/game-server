#[macro_use] extern crate rocket;
use rocket::{State, serde::json::Json};
use rocket_sync_db_pools::database;
use diesel::prelude::*;
use crate::models::User;
use crate::schema::users::dsl::*;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);


#[get("/users/<id>")]
async fn get_user(id: i32, conn: DbConn) -> Option<Json<User>> {
    use crate::schema::users::dsl::*;
    
    let user = conn.run(move |c| {
        users.filter(id.eq(id))
            .first::<User>(c)
            .optional()
    }).await.unwrap();
    
    user.map(Json)
}

#[post("/users", data = "<user>")]
async fn create_user(user: Json<User>, conn: DbConn) -> Json<User> {
    use crate::schema::users::dsl::*;
    
    let new_user = user.into_inner();
    
    conn.run(move |c| {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(c)
            .unwrap()
    }).await.unwrap();
    
    Json(new_user)
}

#[put("/users/<id>", data = "<user>")]
async fn update_user(id: i32, user: Json<User>, conn: DbConn) -> Option<Json<User>> {
    use crate::schema::users::dsl::*;
    
    let updated_user = user.into_inner();
    
    conn.run(move |c| {
        diesel::update(users.filter(id.eq(id)))
            .set((name.eq(updated_user.name), email.eq(updated_user.email)))
            .get_result(c)
            .optional()
    }).await.unwrap();
    
    updated_user.id = id;
    Some(Json(updated_user))
}

#[delete("/users/<id>")]
async fn delete_user(id: i32, conn: DbConn) -> Option<Json<String>> {
    use crate::schema::users::dsl::*;
    
    conn.run(move |c| {
        diesel::delete(users.filter(id.eq(id)))
            .execute(c)
            .unwrap()
    }).await.unwrap();
    
    Some(Json(format!("User with ID {} deleted.", id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_user, create_user, update_user, delete_user])
}
// For returning a Json response
use rocket_contrib::json::Json;
// DB connection and user model
use crate::{TestDbConn, models::User};
// Diesel magic
use crate::schema::users::dsl::*;
use diesel::{QueryDsl, RunQueryDsl};


#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
pub fn list_users(conn: TestDbConn) -> Json<Vec<User>> {
    let result = users.load::<User>(&*conn).unwrap();

    Json(result)
}
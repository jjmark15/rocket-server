
use rocket_contrib::json::{Json};

use crate::models::users::user::{User};

#[get("/")]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[post("/create", data = "<user>")]
pub fn create(user: Json<User>) -> Json<User> {
    user
}
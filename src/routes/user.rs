
use rocket_contrib::json::{Json};

use crate::models::users::user::{User};

#[get("/")]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[post("/create", format = "json", data = "<user>")]
pub fn create(user: Json<User>) -> Json<User> {
    let new_user: User = user.into_inner();
    println!("{:?}", new_user);
    Json(new_user)
}
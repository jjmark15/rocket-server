use rocket_contrib::json::Json;

use crate::models::users::user::User;

#[post("/create", format = "json", data = "<user>")]
pub fn create(user: Json<User>) -> Json<User> {
    let new_user: User = user.into_inner();
    Json(new_user)
}

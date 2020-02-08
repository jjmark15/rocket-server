use rocket::http::Status;
use rocket::response::status;

#[get("/status")]
pub fn status() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "OK")
}

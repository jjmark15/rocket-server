use rocket::http::Status;
use rocket::local::Client;

#[test]
fn status_returns_status_ok_when_alive() {
    let client = Client::new(rocket_service::rocket()).unwrap();
    let mut response = client.get("/api/internal/status").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("OK".into()));
}

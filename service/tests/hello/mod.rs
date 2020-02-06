use rocket::http::Status;
use rocket::local::Client;

#[test]
fn returns_hello_world_greeting() {
    let client = Client::new(rocket_service::rocket()).unwrap();
    let mut response = client.get("/api/tests.hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

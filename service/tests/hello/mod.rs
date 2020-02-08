use rocket::http::Status;

use crate::common_steps::client_steps::get_client_for_local_server;

#[test]
fn returns_hello_world_greeting() {
    let client = get_client_for_local_server();
    let mut response = client.get("/api/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

#[test]
fn returns_hello_name_given_name() {
    let client = get_client_for_local_server();
    let mut response = client.get("/api/hello/Josh").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, Josh!".into()));
}

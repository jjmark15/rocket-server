use rocket::http::Status;

use crate::common_steps::client_steps::get_client_for_local_server;

#[test]
fn status_returns_status_ok_when_alive() {
    let client = get_client_for_local_server();
    let mut response = client.get("/api/internal/status").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("OK".into()));
}

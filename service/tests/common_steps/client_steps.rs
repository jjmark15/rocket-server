use rocket::local::Client;

pub fn get_client_for_local_server() -> Client {
    Client::new(rocket_service::rocket()).expect("valid rocket instance")
}

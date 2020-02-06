#[get("/")]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
pub fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}

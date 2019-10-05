#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
    .mount("/", routes![routes::user::greeting::hello_world])
    .launch();
}
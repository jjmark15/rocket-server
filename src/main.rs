#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod models;

fn main() {
    rocket::ignite()
    .mount("/", routes![routes::user::greeting::hello_world])
    .launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod routes;
mod models;

fn main() {
    rocket::ignite()
    .mount("/api/user", routes![
        routes::user::hello_world,
        routes::user::create,
        ])
    .launch();
}
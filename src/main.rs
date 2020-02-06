#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod web;

fn main() {
    rocket::ignite()
        .mount(
            "/api/user",
            routes![web::routes::user::hello_world, web::routes::user::create],
        )
        .launch();
}

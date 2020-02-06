#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/api/user",
            routes![routes::user::hello_world, routes::user::create],
        )
        .launch();
}

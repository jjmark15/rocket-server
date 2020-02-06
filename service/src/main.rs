#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod functional_tests;
mod models;
mod web;

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api/user", routes![web::routes::user::create])
        .mount("/api/hello", routes!(web::routes::hello::hello_world))
}

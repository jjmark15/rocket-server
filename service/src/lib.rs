#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod web;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api/user", routes![web::routes::user::create])
        .mount(
            "/api/hello",
            routes!(
                web::routes::hello::hello_world,
                web::routes::hello::hello_name
            ),
        )
        .mount("/api/internal", routes![web::routes::internal::status])
}

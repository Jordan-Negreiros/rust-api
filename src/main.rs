#[macro_use]
extern crate rocket;

mod controller;
mod service;
mod dto;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(controller::climatempo_controller::stage())
}
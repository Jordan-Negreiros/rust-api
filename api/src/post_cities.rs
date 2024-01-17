use rocket::{post, routes};
use rocket::response::status::Created;
use rocket::serde::json::Json;

use application::usecase;
use domain::models::City;

#[post("/cities", format = "json", data = "<city>")]
pub fn create_city(city: Json<City>) -> Created<String> {
    usecase::post_cities::create_city(city)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Cities Stage", |rocket| async {
        rocket.mount("/cities", routes![create_city])
    })
}
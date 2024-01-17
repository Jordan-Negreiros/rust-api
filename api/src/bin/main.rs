#[macro_use]
extern crate rocket;

use api::get_tempo_handler;
use api::post_cities;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(get_tempo_handler::stage())
        .attach(post_cities::stage())
}
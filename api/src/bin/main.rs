#[macro_use]
extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(api::get_tempo_handler::stage())
}
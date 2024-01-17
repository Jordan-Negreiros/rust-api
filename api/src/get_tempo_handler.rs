use rocket::{get, routes, State};
use rocket::serde::json::{Json, Value};

use application::usecase;

#[get("/<city>")]
pub async fn get_temperature(api_url: &State<String>, city: String) -> Json<Value> {
    usecase::get_tempo::temperature(api_url, city).await
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Climatempo Stage", |rocket| async {
        rocket.mount("/temperatura", routes![get_temperature])
            .manage("https://api.open-meteo.com/v1".to_string())
    })
}
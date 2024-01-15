use rocket::{get, routes, State};
use rocket::serde::json::{Json, Value};

use application::usecase::get_tempo;

#[get("/temperatura/<city>")]
async fn temperatura(api_url: &State<String>, city: &str) -> Json<Value> {
    get_tempo::temperature(api_url, city).await
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Climatempo Stage", |rocket| async {
        rocket.mount("/", routes![temperatura])
            .manage("https://api.open-meteo.com/v1".to_string())
    })
}
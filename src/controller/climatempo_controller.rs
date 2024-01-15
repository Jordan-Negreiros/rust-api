use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::service::climatempo_service;

#[get("/temperatura/<city>")]
async fn temperatura(api_url: &State<String>, city: &str) -> Json<Value> {
    climatempo_service::temperature(api_url, city).await
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Climatempo Stage", |rocket| async {
        rocket.mount("/", routes![temperatura])
            .manage("https://api.open-meteo.com/v1".to_string())
    })
}
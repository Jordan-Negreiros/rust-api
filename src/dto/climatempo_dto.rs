use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ForecastResponse {
    pub hourly: Hourly,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Hourly {
    pub temperature_2m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DailyForecast {
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
}

#[derive(Debug)]
pub struct Location {
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}
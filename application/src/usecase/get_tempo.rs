use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl,
};
use domain::{models::*, schema::city::name};
use infrastructure::establish_connection;
use rocket::serde::json::{json, Json, Value};

use crate::dto::climatempo::{DailyForecast, ForecastResponse};

pub async fn temperature(api_url: &str, city: String) -> Json<Value> {
    let city = find_city(city);
    let url = format!(
        "{}/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        api_url, city.latitude, city.longitude
    );

    let response = reqwest::get(url);
    let forecast = response
        .await
        .unwrap()
        .json::<ForecastResponse>()
        .await
        .unwrap();

    let today = DailyForecast {
        temperature_2m_max: forecast
            .hourly
            .temperature_2m
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone(),
        temperature_2m_min: forecast
            .hourly
            .temperature_2m
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone(),
    };

    Json(json!({
        "City": city.name,
        "max": today.temperature_2m_max,
        "min": today.temperature_2m_min,
    }))
}

fn find_city(city_name: String) -> City {
    use domain::schema::city;

    let connection = &mut establish_connection();
    match city::table
        .filter(name.eq(city_name))
        .select(city::all_columns)
        .first::<City>(connection)
    {
        Ok(city) => city,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

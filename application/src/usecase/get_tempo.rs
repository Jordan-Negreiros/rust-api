use rocket::serde::json::{Json, json, Value};

use crate::dto::climatempo::{DailyForecast, ForecastResponse, Location};

pub async fn temperature(api_url: &str, city: &str) -> Json<Value> {
    let location = get_location(&city);
    let url = format!("{}/forecast?latitude={}&longitude={}&hourly=temperature_2m", api_url, location.latitude, location.longitude);

    let response: ForecastResponse = reqwest::get(url)
        .await
        .expect("request failed")
        .json()
        .await
        .expect("json parsing failed");

    let today = DailyForecast {
        temperature_2m_max: response.hourly.temperature_2m.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone(),
        temperature_2m_min: response.hourly.temperature_2m.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone(),
    };

    Json(json!({
        "city": location.city,
        "max": today.temperature_2m_max,
        "min": today.temperature_2m_min,
    }))
}

fn get_location(city: &str) -> Location {
    match city {
        "sao_paulo" => Location {
            city: "São Paulo".to_string(),
            latitude: -23.5489,
            longitude: -46.6388,
        },
        "rio_de_janeiro" => Location {
            city: "Rio de Janeiro".to_string(),
            latitude: -22.9068,
            longitude: -43.1729,
        },
        "brasilia" => Location {
            city: "Brasília".to_string(),
            latitude: -15.7797,
            longitude: -47.9297,
        },
        _ => Location {
            city: "São Paulo".to_string(),
            latitude: -23.5489,
            longitude: -46.6388,
        },
    }
}
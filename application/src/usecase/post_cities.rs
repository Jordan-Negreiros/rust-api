use diesel::RunQueryDsl;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use domain::models::City;
use infrastructure::establish_connection;

pub fn create_city(city: Json<City>) -> Created<String> {
    use domain::schema::cities;
    let city = city.into_inner();

    match diesel::insert_into(cities::table).values(&city).get_result::<City>(&mut establish_connection()) {
        Ok(city) => {
            Created::new(format!("/cities/{}", city.id))
        }
        Err(err) => match err {
            _ => {
                panic!("Database error: {:?}", err)
            }
        }
    }
}
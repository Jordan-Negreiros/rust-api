use diesel::RunQueryDsl;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use domain::models::{City, NewCity};
use infrastructure::establish_connection;

pub fn create_city(city: Json<NewCity>) -> Created<String> {
    use domain::schema::city;
    let city = city.into_inner();

    match diesel::insert_into(city::table).values(&city).get_result::<City>(&mut establish_connection()) {
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
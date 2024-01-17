use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::cities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct City {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}
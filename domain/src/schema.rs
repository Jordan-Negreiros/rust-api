// @generated automatically by Diesel CLI.

diesel::table! {
    city (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        state -> Varchar,
        #[max_length = 255]
        country -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}

diesel::table! {
    cities (id) {
        id -> Int4,
        name -> Varchar,
        state -> Varchar,
        country -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}
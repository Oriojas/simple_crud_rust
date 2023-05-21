// @generated automatically by Diesel CLI.

diesel::table! {
    rust_db (id) {
        id -> Integer,
        name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}

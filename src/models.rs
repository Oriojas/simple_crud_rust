use diesel::{Queryable, Insertable, Selectable};

use crate::schema::rust_db;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = rust_db)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = rust_db)]
pub struct NewPost {
    pub name: String,
    pub last_name: String,
    pub email: String,
}
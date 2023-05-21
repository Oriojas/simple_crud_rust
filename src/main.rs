extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;
use service::PostService;
use std::env;
use std::ops::Add;

mod schema;
mod models;
mod repository;
mod service;

use crate::repository::PostRepository;

use self::models::{NewPost};
use self::schema::rust_db::dsl::*;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &mut MysqlConnection, new_name: &'a str, new_last_name: &'a str, new_email: &'a str) -> Post{
    let new_post = NewPost {
        name : String::from(new_name),
        last_name : String::from(new_last_name),
        email: String::from(new_email),
    };

    diesel::insert_into(rust_db)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    rust_db.order(id.desc()).first(conn).unwrap()
}

fn update_post(conn: &mut MysqlConnection, post: &Post, new_name: &str) -> Post {
    diesel::update(rust_db.find(post.id))
        .set(name.eq(new_name))
        .execute(conn)
        .unwrap();

    rust_db.find(post.id).first(conn).unwrap()
}

fn main() {

    //////////////////////////////////////////////////
    ////////  Trabajar unicamente con Diesel   ///////
    //////////////////////////////////////////////////
    // let mut conn = establish_connection();

    // let new_title = "Ejemplo de Post";
    // let new_body = "Este es un ejemplo funcional para almacenar un POST";

    // let new_post = create_post(&mut conn, new_title, new_body);
    // println!("{:?}", new_post);

    // let modified_title = String::from(new_title);
    // let modified_post = update_post(&mut conn, &new_post, &modified_title.add(" (MODIFICADO)"));
    // println!("{:?}", modified_post);

    // let data = posts.load::<Post>(&mut conn).unwrap();
    // for post in data {
    //     println!("{:?}", post);
    // }

    ///////////////////////////////////////////////////////
    ////////  Trabajar Patrones Diseño con Diesel   ///////
    ///////////////////////////////////////////////////////
    let new_name = "Oscar";
    let new_last_name = "Riojas";
    let new_email = "oscar@mail.mail";

    let mut post_service = PostService::new();

    let new_post = post_service.create_post(new_name, new_last_name, new_email);

    let data_from_repository = post_service.get_posts();
    for post in data_from_repository {
        println!("{:?}", post);
    }


}



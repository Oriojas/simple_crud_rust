extern crate diesel;
extern crate dotenv;
extern crate gtk;


use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;
use service::PostService;
use std::env;
use std::ops::Add;
use gtk::{prelude::*, Orientation};
use gtk::{Button, Entry, Label, Window, WindowType};

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
    gtk::init().expect("No se puedo iniciar la ventana GTK");

    let window: Window = Window::new(WindowType::Toplevel);
    window.set_title("Polkadot curso Rust");
    window.set_default_size(320, 200);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
    window.add(&container);

    let name_val = Label::new(Some("Introduzca su nombre"));
    container.add(&name_val);

    let entry_name = Entry::new();
    container.add(&entry_name);

    let label_name = Label::new(None);
    container.add(&label_name);


    let last_name_val = Label::new(Some("Introduzca su apellido"));
    container.add(&last_name_val);

    let entry_last_name = Entry::new();
    container.add(&entry_last_name);

    let label_last_name = Label::new(None);
    container.add(&label_last_name);


    let email_val = Label::new(Some("Introduzca su email"));
    container.add(&email_val);

    let entry_email = Entry::new();
    container.add(&entry_email);

    let label_email = Label::new(None);
    container.add(&label_email);

    let button = Button::with_label("Guardar");
    button.connect_clicked(move |_| {
        let text_name = entry_name.get_text().to_string();
        label_name.set_text(&text_name);
        let text_last_name = entry_last_name.get_text().to_string();
        label_last_name.set_text(&text_last_name);
        let text_email = entry_email.get_text().to_string();
        label_email.set_text(&text_email);

        let mut post_service = PostService::new();

        let new_post = post_service.create_post(&text_name, &text_last_name, &text_email);
    
        let data_from_repository = post_service.get_posts();
        for post in data_from_repository {
            println!("{:?}", post);
        }
    
    });
    container.add(&button);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();


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


}



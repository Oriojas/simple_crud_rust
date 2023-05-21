use crate::models::{Post, NewPost};
use crate::repository::{PostRepository};

use diesel::MysqlConnection;
use diesel::result::Error;
use crate::diesel::Connection;
use dotenv::dotenv;
use std::env;


pub struct PostService {
    pub repository: PostRepository,
}

impl PostService{

    pub fn new() -> Self {
        PostService {
            repository: PostRepository::new()
        }
    }

    pub fn create_post(&mut self, name: &str, last_name: &str, email: &str) -> Result<Post, Error> {
        let new_post = NewPost {
            name : String::from(name),
            last_name : String::from(last_name),
            email: String::from(email),
        };
        self.repository.create(&new_post)
    }

    pub fn get_posts(&mut self) ->  Result<Vec<Post>, diesel::result::Error> {
        self.repository.find_all()
    }

}

use diesel::prelude::*;
use super::Post;
use super::{establish_connection, schema};
pub struct PostService;

impl PostService {
    pub fn fetch() -> Vec<Post> {
        use schema::posts::dsl::*;
        let connection = &mut establish_connection();
        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts");
        
        println!("here");
        for post in &results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
       results
    }
}
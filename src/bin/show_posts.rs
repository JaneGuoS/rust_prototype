use diesel::prelude::*;
use rust_prototype::models::Post;
use rust_prototype::{establish_connection, schema};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


fn main(){

    println!("initializing show posts...");

    use schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    
    info!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
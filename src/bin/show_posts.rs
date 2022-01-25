extern crate blog_demo;
extern crate diesel;

use blog_demo::*;
use diesel::prelude::*;
use models::*;

fn main() {
    use blog_demo::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Showing {} posts", results.len());
    for post in results {
        println!("{} by {}", post.title, post.author);
        println!("------------\n");
        println!("{}", post.body)
    }
}

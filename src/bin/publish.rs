use self::models::Post;
use blog_demo::*;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use blog_demo::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("Publish post need the post id")
        .parse::<i32>()
        .expect("Invalid post id, want int32");

    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find the post {}", id));

    println!("Published {}", post.title);
}

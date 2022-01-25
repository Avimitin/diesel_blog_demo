use blog_demo::*;

use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Input the title\n");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len()-1)];
    println!("Input the body!\n");
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
    println!("What is your name!");
    let mut author = String::new();
    stdin().read_line(&mut author).unwrap();

    let posts = create_post(&connection, &title, &body, &author);
    println!("\nSaved draft {} with id {}", title, posts.id);
}

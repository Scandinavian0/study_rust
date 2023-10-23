use crate::Summary::{Post, summary, weibo};

mod Summary;

fn main() {
    let post = Post{tile: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.sumamarize());
    println!("{}",weibo.sumamarize());
}

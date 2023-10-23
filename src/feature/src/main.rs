use crate::Summary::{notify, Post, summary, weibo};

mod Summary;
mod example;

fn main() {
    let post = Post{tile: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo1 = weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    // let notify=notify(weibo);
    println!("{}",post.summarize());
    println!("{}",weibo1.summarize());
}

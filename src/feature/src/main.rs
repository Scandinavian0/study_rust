use crate::example::{add, Point};
use crate::Summary::{notify, Post, summary, weibo};
// use crate::example::{Point};
mod Summary;
mod example;

fn main() {
    let post = Post{tile: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo1 = weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    // let notify=notify(weibo);
    println!("{}",post.summarize());
    println!("{}",weibo1.summarize());

    println!("-------------example-------------");
    // 这里使用了point的默认实现方法，
    let p1=Point{x:1.1f32,y:1.1f32};
    let p2=Point{x:2.1f32,y:2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));

    let p = Point{x:3,y:3};
    println!("{:?}",p);
}

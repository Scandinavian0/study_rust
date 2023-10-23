use std::fmt::{Debug, Display};

pub trait summary {
    fn summarize_author(&self) -> String;
    // 默认实现，这里只需要调用summarize_author就可以了。再次解耦出来
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub tile: String,
    pub author: String,
    pub content: String,
}

impl summary for Post {
    fn summarize_author(&self) -> String {
        todo!()
    }

    fn summarize(&self) -> String {
        format!("文章{},作者是{}", self.tile, self.author)
    }
}

pub struct weibo {
    pub username: String,
    pub content: String,
}

impl summary for weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特征作为函数参数
pub fn notify(item: &impl summary) {
    println!("Breaking news! {}", item.summarize());
}

// 特征约束:强制函数的两个参数是同一类型
pub fn notiry<T: summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// 多重约束
pub fn notify_more<T: summary + Display>(item: &T) {}

// where约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32{}
// 可改为
fn some_function<T, U>(t: &T, u: &U) -> i64
    where T: Display + Clone,
          U: Clone + Debug {0}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
//在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的
//模块公有并不使其内容也是公有的
//私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。

pub mod hosting;


pub mod serving {
    pub fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

//使用 pub 来设计公有的结构体和枚举
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

//如果我们将枚举设为公有，则它的所有成员都将变为公有
pub enum Appetizer {
    Soup,
    Salad,
}

pub enum test {
    Soup,
    Salad,
}

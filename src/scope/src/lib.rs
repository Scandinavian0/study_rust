// use crate::front_of_house::Breakfast;

mod front_of_house {
    //在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的
    //模块公有并不使其内容也是公有的
    //私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。
   pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    //使用 pub 来设计公有的结构体和枚举
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast {
        pub fn summer(toast:&str)->Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //如果我们将枚举设为公有，则它的所有成员都将变为公有
    pub enum Appetizer{
        Soup,
        Salad,
    }

    pub enum test{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    //父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
    // 不适用pub的话会报错module `hosting` is private
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 不适用pub的话会报错module `hosting` is private
    //相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用 pub 来设计公有的结构体和枚举
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal =front_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    //Field `seasonal_fruit` of struct `front_of_house::Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");

    // 枚举
    let order1= front_of_house::Appetizer::Soup;
    let order2=front_of_house::Appetizer::Salad;
    let order3=front_of_house::test::Salad;
}

fn deliver_order(){

}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}
}


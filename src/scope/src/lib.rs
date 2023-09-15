// use crate::front_of_house::Breakfast;

mod front_of_house;

pub use front_of_house::hosting;
// 这里和Java不同的是，mod可以作为一个文件的名字，在子mod里面就不需要定义类似于public class xxx之类的东东
pub fn eat_at_restaurant(){
    //父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
    // 不适用pub的话会报错module `hosting` is private
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 不适用pub的话会报错module `hosting` is private
    //相对路径
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
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

    // 使用use引入作用域
    hosting::add_to_waitlist();
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


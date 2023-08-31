use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is :{x}");

    x = 6;
    println!("The value of x is ：{x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS:{THREE_HOURS_IN_SECONDS}");

    let space = "     ";
    let space = space.len();
    // 布尔
    let bool_type: bool = false;

    //元组
    let compound_tuple: (i32, f64, u8) = (500, 6.4, 1);
    //tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值，
    let compound_tuple = (500, 6.4, 1);
    let (x, y, z) = compound_tuple; //解构
    println!("The value of y is: {y}");

    //当你想要在栈（stack）而不是在堆（heap）上为数据分配空间，或者是想要确保总是有固定数量的元素时，数组非常有用。
    //数组（array）。与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust 中的数组长度是固定的。
    let com_array = [1, 2, 3, 4, 5];
    //可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    let com_array: [i32; 5] = [1, 2, 3, 4, 5];
    //访问数组元素
    let first = com_array[0];
    let second = com_array[1];
    println!("first:{first}+  second:{second}");
    // find_non_array();
    another_function(5);
    print_labeled_measurement(5, 'h');

    // 具有返回值的函数
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    // if判断
    branches();

    //循环
    loops();
}

fn find_non_array() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 居然返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn branches() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else { println!("condition was false"); }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

//循环 loop while for
fn loops() {
    // loop
    let mut num = 0;
    loop {
        println!("loop_again!");
        num = num + 1;
        if num == 5 { break; }
    };
    // 从循环返回值
    loop_result();
    // 嵌套循环消除歧义--循环标签
    loop_label();
    // while 循环
    loop_while();
    // while 遍历数组
    loop_while_array();
    //for
    loop_for();
    //loop_for_rev
    loop_for_rev();
}

// 从循环返回值
fn loop_result() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 { break counter * 2; }
    };
    println!("The result is {result}");
}

// 嵌套循环消除歧义--循环标签
fn loop_label() {
    println!("----loop_label----");
    let mut count=0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining=10;

        loop {
            println!("remaining = {remaining}");
            if remaining ==9{ break; }
            // 如果count==2 则通过这个标识退出外层循环
            if count ==2{ break 'counting_up; }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count = {count}");
}

fn loop_while(){
    println!("----loop_while----");
    let mut number=3;
    while number!=0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// while遍历数组
fn loop_while_array(){
    println!("----loop_while_array----");
    let a=[10,20,30,40,50];
    let mut index=0;
    while index<5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn loop_for(){
    println!("----loop_for----");
    let a=[10,20,30,40,50];
    //我们增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素而导致的 bug。
    for element in a{
        println!("the value is: {element}");
    }
}

// 使用rev来反转range
fn loop_for_rev(){
    println!("----loop_for_rev----");
    for number in (1..4).rev()  {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


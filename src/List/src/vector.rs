use std::collections::HashMap;
pub fn add_element() ->Vec<i32>{
    let mut v=Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v
}

pub fn get_vector_element(){
    let v=add_element();
    let third:&i32=&v[2];
    println!("The third element is {third}");
    let third:Option<&i32>=v.get(2);
    match third {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {third}"),
    }
}

pub fn for_vector_element(){
    let v=add_element();
    for i in &v{
        println!("{i}");
    }

    println!("遍历可变值的vector");
    let mut v=add_element();
    for i in &mut v {
        println!("{i}");
        // 解引
        *i+=50;
    }
    for i in &mut v {
        println!("{i}");
    }
}

pub fn string_code(){
    // updata
    let mut s1=String::from("foo");
    let mut s2=String::from("foo");
    let s3="bar";
    s1.push_str(s3);
    // push默认是追加一个char
    // s2.push(s3);
    println!("s3 is {s3}");

    // format 宏
    format();
}

fn format(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 使用宏定义不会获取参数或者变量的所有权

    let s=format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

pub fn hash_map(){
    let mut scores=HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("yellow"),50);
    let team_name=String::from("blue");
    //程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零
    //如果没有使用unwrap_or，则会打印出来Some(10)
    let score=scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}",score);


    // hashmap和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //Help: consider cloning the value if the performance cost is acceptable
    //这里 field_name 和 field_value 不再有效
    // println!("{:?}",field_name);

    // 覆盖scores的值
    scores.insert(String::from("blue"),15);
    let score=scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}",score);
}



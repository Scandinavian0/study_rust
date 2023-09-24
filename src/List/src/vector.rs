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
    //entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值

    let mut scores=HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    println!("{:?}",scores);

    // 根据旧值更新一个值
    let test="hello world wonderful world";
    let mut map=HashMap::new();
    for word in test.split_whitespace() {
        //or_insert 方法返回这个键的值的一个可变引用（&mut V）
        let count=map.entry(word).or_insert(0);
        //*解引用
        *count+=1;

    }

    println!("{:?}",map);
}



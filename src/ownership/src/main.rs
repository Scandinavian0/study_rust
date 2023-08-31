fn main() {
    // println!("Hello, world!");
    //
    // let mut s = String::from("hello");
    // s.push_str(",world");
    // println!("{}", s);
    //
    // let s1 = String::from("hello");
    // let s2 = s1;
    // // println!("{}",s1);

    println!("----------reference--------------");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of  '{}' is {}", s1, len);
}

// 格式需要对齐
fn calculate_length(s: &String) -> usize {
    //当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权
    s.len()
}
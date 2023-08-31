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
    //正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
    let len = calculate_length(&s1);
    println!("The length of  '{}' is {}", s1, len);

    // 可变引用
    let mut s=String::from("hello");

    change(&mut s);

    println!("change string:{}",s);
}

// 格式需要对齐
//我们将创建一个引用的行为称为 借用（borrowing）
fn calculate_length(s: &String) -> usize {
    //当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权
    s.len()
}

fn change(some_string:&mut String){
    some_string.push_str(",world");
}
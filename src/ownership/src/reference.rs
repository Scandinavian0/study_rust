
pub fn reference_code(){
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
    //可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
    //编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    //  两个或更多指针同时访问同一数据。
    //  至少有一个指针被用来写入数据。
    //  没有同步数据访问的机制。
    let mut s=String::from("hello");

    change(&mut s);

    println!("change string:{}",s);

    //不能在拥有不可变引用的同时拥有可变引用。
    /*
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);
     */

    // 垂悬引用：指向的内存可能已经被分配给其他持有者

    dangling_references();

    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
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

fn dangling_references(){
    let reference_to_nothing=dangle();
}
fn dangle()->String{
    let s=String::from("hello");
    s
}
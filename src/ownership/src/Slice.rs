pub fn slice_code() {
    let mut word_for_frist=String::from("Hello world");

    let word=first_word(&word_for_frist);

    word_for_frist.clear();
    println!("the first word is :{}",word);

    let mut word_for_frist_2=String::from("hello world");

    let word=new_frist_word(&word_for_frist_2);


    deref_coercions(&word_for_frist_2);
}


//该函数接收一个用空格分隔单词的字符串，
// 并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
fn first_word(s: &String) -> usize {
    // as_bytes 方法将 String 转化为字节数组。
    let bytes=s.as_bytes();
    //enumerate 返回的是一个元组
    for (i,&item) in bytes.iter().enumerate() {
        // b' ' 表示空格字符的字节表示。在 ASCII 编码中，空格字符的字节表示是 32。
        if item==b' ' {
            return i;
        }
    }

    s.len()
}


//&str 是字符串切片的引用，它是一个对一段字符串数据的引用，而不是具体的字符串对象。
//&str 是不可变的，它只允许你读取字符串数据，但不能修改它。
//&str 是静态生命周期的，它可以引用编译时已知的字符串常量，也可以引用在函数内创建的字符串。
// 通常，&str 用于函数参数，表示函数接受一个字符串引用作为输入，并且通常用于字符串切片，例如：fn example(s: &str) { ... }。
fn new_frist_word(s:&str)->&str{
    // as_bytes 方法将 String 转化为字节数组。
    let bytes=s.as_bytes();
    //enumerate 返回的是一个元组
    for (i,&item) in bytes.iter().enumerate() {
        // b' ' 表示空格字符的字节表示。在 ASCII 编码中，空格字符的字节表示是 32。
        if item==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn deref_coercions(s:&str){
    let word=new_frist_word(&s[0..6]);
    let word=new_frist_word(&s[..]);
    // 等价于
    let word=new_frist_word(&s);

    let my_string_literal="hello world";

    let word=new_frist_word(my_string_literal);


}
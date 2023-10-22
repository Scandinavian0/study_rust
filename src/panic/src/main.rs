use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    let greeting_file_result=File::open("hello.txt");

    let gretting_file=match greeting_file_result {
        Ok(file)=>file,
        // 报错类型区分
        //File::open 返回的 Err 成员中的值类型 io::Error，它是一个标准库中提供的结构体。这个结构体有一个返回 io::ErrorKind 值的 kind 方法可供调用
        Err(error)=> match error.kind() {
            ErrorKind::NotFound=>match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file:{:?}",e),
            },
            other_error=>{
                panic!("Problem opening the file :{:?}",other_error);
            }
        },
    };
}

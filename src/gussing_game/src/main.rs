// io 输入输出 std标准库
// use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is :{}", secret_number);

    loop {
        println!("Please input your guess");

        // let 定义变量，mut 使变量可变
        let mut guess = String::new();
        // 可以写成这样，std标准库,预导入
        /*
     *read_line 的工作是，无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中，
     *因此它需要字符串作为参数。这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去。
     *&表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
     *引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。
     */
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line.");
        println!("You guessed :{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

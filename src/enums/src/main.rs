// use std::net::Ipv4Addr;
use std::net::IpAddr;

//每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddrKind_2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
    let home_2 = IpAddrKind_2::V4(127, 0, 0, 1);

    // let home_3=Ipv4Addr

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    println!("{}", x.expect("fruits are healthy"));

    let five=Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?},{:?}",six,none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
//当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值
//它获取一个 Option<i32> ，如果其中含有一个值，将其加一。如果其中没有值，函数应该返回 None 值，而不尝试执行任何操作。
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
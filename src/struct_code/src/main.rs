#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 多参数引用
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数,更轻松地创建一个正方形
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 利用元组传输数据
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );

    // 利用结构体传输数据
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    // 方法语法 它们在结构体的上下文中被定义
    // （或者是枚举或 trait 对象的上下文，将分别在第六章和第十七章讲解），并且它们第一个参数总是 self，
    // 它代表调用该方法的结构体实例。
    // 类似于Java对象里的方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectangle) -> u32 {
    //访问对结构体的引用的字段不会移动字段的所有权
    rectangle.width * rectangle.height
}
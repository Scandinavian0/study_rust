fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T>(list:&[T])->&T{
//     let mut largest=&list[0];
//
//     for item in list{
//         // 这里会报错，显示largest会不适用与T的所有可能类型。
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }


// 结构体的定义
// struct Point<T>{
// 定义多个泛型
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T,U> Point<T,U>{
    fn mixup<V,W>(self,other:Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

// impl Point<f32, U>{
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


fn struct_code() {
    let integer = Point { x: 5, y: 10 };
    // 这里的x，y类型必须相同，不然就会报错无法编译。
    let float = Point { x: 1.0, y: 2.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

}

use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
pub(crate) struct Point<T: Add<T, Output=T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    pub x: T,
    pub y: T,
}

impl <T:Add<T,Output=T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x+rhs.x,
            y:self.y+rhs.y,
        }
    }
}

pub fn add<T:Add<T,Output=T>>(a:T,b:T)->T{
    a+b
}


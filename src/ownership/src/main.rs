use std::io::{Stdin, stdin};
use crate::reference::reference_code;
use crate::Slice::slice_code;

mod reference;
mod Slice;

fn main() {

    // 引用与借用
    reference_code();

    // Slice类型
    slice_code()

}

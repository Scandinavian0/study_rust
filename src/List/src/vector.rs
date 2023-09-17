pub fn add_element()->Vec<i32>{
    let mut v=Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v
}

pub fn get_vector_element(){
    let v=add_element();
    let third:&i32=&v[2];
    println!("The third element is {third}");
    let third:Option<&i32>=v.get(2);
    match third {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {third}"),
    }
}
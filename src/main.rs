// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {

    let s = String::from("Hello");
    take_ownership(&s);
    println!("Success s! {}", s);


    let x = 5;
    makes_copy(x);
    println!("Success! {}", x)
}

fn take_ownership(s: &String) -> String {
    println!("Printed param {}", s);
    s.to_string()
}


fn makes_copy(x:i32){
    println!("Copy wala fn {}", x)
}









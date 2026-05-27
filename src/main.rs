// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let s = String::from("hello, ");
    let mut s1 = s;
    s1.push_str("world");

    println!("{}",s1)
}





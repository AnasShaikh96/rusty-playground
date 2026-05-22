// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let mut s1 = String::from("hello");
    let  s2 = s1;    

    s1 = s2.clone()

    println!("Shouldnt run {}", s2)
}






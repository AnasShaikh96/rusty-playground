// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let s1 = String::from("Hello, World");
    let s2 = take_ownership(s1);

    println!("{}", s2);    
}

fn take_ownership(s:String) ->String {
    println!("{}",s);
    s
}




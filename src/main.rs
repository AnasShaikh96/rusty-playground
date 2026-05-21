// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    never_return();
    println!("Success",)
}

fn never_return() -> ! {
    todo!();
}




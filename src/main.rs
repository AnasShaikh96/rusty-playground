// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let c = '%';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1,*r2);
    assert_eq!(get_address(&r1),get_address(&r2));

    println!("Success")
}

fn get_address(c: &char) -> String{
    format!("{:p}",c)
}










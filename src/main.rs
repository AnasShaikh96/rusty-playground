
// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let _v : () = ();

    let v:(i32,i32) = (3,4);
    assert_eq!(_v, implicitly_ret_unit());
    println!("Success")
}


fn implicitly_ret_unit(){
    println!("I will return ()");
}
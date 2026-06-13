// use std::mem::size_of_val;
#[allow(unused_variables)]

enum Number {
    Zero = 0,
    One = 1,
    Two = 2
}

fn main(){
    assert_eq!(Number::One as u8, 1);
    assert_eq!(Number::Two as u8, 2);
    assert_eq!(Number::Zero as u8, 0);

    println!("Succes")
}






// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    struct User {
        name: String,
        number: u64,
        age: u16,
        email: String
    };

    let user = User {
        name: "John Doe",
        number: 123456789,
        age: 60,
        email: "some@admin.com"
    };

    println!("Success {}", user)
}






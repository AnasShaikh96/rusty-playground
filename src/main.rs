// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    struct User {
        name: String,
        number: u64,
        age: u16,
        email: String
    }

    let user = User {
        name: String::from("John Doe"),
        number: 123456789,
        age: 60,
        email: String::from("some@admin.com")
    };

    println!("Success {}", user.name)
}






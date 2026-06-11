// use std::mem::size_of_val;

#[allow(unused_variables)]
struct User {
        name: String,
        number: u64,
        age: u16,
        email: String
    }
fn main() {
    let user1 = User {
        name: String::from("John Doe"),
        number: 123456789,
        age: 60,
        email: String::from("some@admin.com")
    };

    let user2 = User {
        email:String::from("someuser@admin.com"),
        ..user1
    };

    println!("Success {}", user2.email);
}






// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age : Box<u8>,
    }

    let person : Person = Person {
        name: String::from("Alice"),
        age: Box::new(20)
    };

    let Person {name, ref age} = person;

    println!("person age is {}", person.age);
    println!("person name is {}", name);

}





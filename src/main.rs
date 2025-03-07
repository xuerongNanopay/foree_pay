#![allow(unused)]

use foam_rust::{foam_class, GettersSetters};

// #[derive(GettersSetters)]
// struct Person {
//     name: String,
//     age: u32,
// }

foam_class! {
    Person, 
    name: String, 
    age: u32
}

fn main() {
    // let mut person = Person {
    //     name: "Alice".to_string(),
    //     age: 25,
    // };

    // println!("Name: {}", person.get_name());
    // println!("Age: {}", person.get_age());

    // person.set_name("Bob".to_string());
    // person.set_age(30);

    // println!("Updated Name: {}", person.get_name());
    // println!("Updated Age: {}", person.get_age());
}
#![allow(unused)]

use foam_rust::{foam_class, foam_class_proc, GettersSetters};

// #[derive(GettersSetters)]
// struct Person {
//     name: String,
//     age: u32,
// }

// foam_class! {
//     Person, 
//     name: String, 
//     age: u32
// }

// foam_class! {
//     Person1, 
//     name: String, 
//     age: u32
// }


foam_class_proc! {
    name: "dasfads",
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
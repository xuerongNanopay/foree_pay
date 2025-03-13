#![allow(unused)]

trait PropertyClass {
}

fn to_property_class(c: &str) -> Result<Box<dyn PropertyClass>, ()> {
    Ok(match c {
        "Int" => Box::new(IntProperty),
        "Long" => Box::new(LongProperty),
        "Double" => Box::new(DoubleProperty),
        "Date" => Box::new(DateProperty),
        _ => {
            return Err(())
        }
    })
}


// #[derive(Debug)]
// pub(crate) enum PropertyType {
//     Int(IntProperty),
//     Long,
//     Double,
//     Vec,
//     Class,
// }


struct IntProperty;

impl PropertyClass for IntProperty {

}

struct LongProperty;

impl PropertyClass for LongProperty {

}

struct DoubleProperty;

impl PropertyClass for DoubleProperty {

}

struct DateProperty;

impl PropertyClass for DateProperty {

}
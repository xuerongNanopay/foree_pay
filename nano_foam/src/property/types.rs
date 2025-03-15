#![allow(unused)]

trait PropertyClass {
}

fn new_property(c: &str) -> Result<Box<dyn PropertyClass>, ()> {
    Ok(match c {
        "Int" => Box::new(IntProperty),
        "UInt" => Box::new(UIntProperty),
        "Long" => Box::new(LongProperty),
        "ULong" => Box::new(ULongProperty),
        "Double" => Box::new(DoubleProperty),
        "String" => Box::new(StringProperty),
        "Date" => Box::new(DateProperty),
        "Struct" => Box::new(StructProperty),
        "Enum" => Box::new(EnumProperty),
        "Vec" => Box::new(VecProperty),
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

struct UIntProperty;

impl PropertyClass for UIntProperty {

}

struct LongProperty;

impl PropertyClass for LongProperty {

}


struct ULongProperty;

impl PropertyClass for ULongProperty {

}

struct DoubleProperty;

impl PropertyClass for DoubleProperty {

}

struct DateProperty;

impl PropertyClass for DateProperty {

}

struct StringProperty;

impl PropertyClass for StringProperty {

}

struct StructProperty;

impl PropertyClass for StructProperty {

}

struct EnumProperty;

impl PropertyClass for EnumProperty {

}

struct VecProperty;

impl PropertyClass for VecProperty {

}
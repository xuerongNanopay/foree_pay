#![allow(unused)]

trait PropertyInfo {
}

fn new_property(c: &str) -> Result<Box<dyn PropertyInfo>, ()> {
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

impl PropertyInfo for IntProperty {

}

struct UIntProperty;

impl PropertyInfo for UIntProperty {

}

struct LongProperty;

impl PropertyInfo for LongProperty {

}


struct ULongProperty;

impl PropertyInfo for ULongProperty {

}

struct DoubleProperty;

impl PropertyInfo for DoubleProperty {

}

struct DateProperty;

impl PropertyInfo for DateProperty {

}

struct StringProperty;

impl PropertyInfo for StringProperty {

}

struct StructProperty;

impl PropertyInfo for StructProperty {

}

struct EnumProperty;

impl PropertyInfo for EnumProperty {

}

struct VecProperty;

impl PropertyInfo for VecProperty {

}
#![allow(unused)]

trait PropertyClass {

    fn to_class(c: &str) -> Result<impl PropertyClass, ()> {
        Ok(match c {
            "Int" => IntProperty,
            _ => {
                return Err(())
            }
        })
    }
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
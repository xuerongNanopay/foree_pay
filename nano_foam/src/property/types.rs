#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;

use super::Property;

trait PropertyInfo {
    fn to_setter_token_stream(&self, property: &Property) -> Result<TokenStream, TokenStream> {
        Err(quote! {
            compile_error!("Do not support");
        })
    }
}

fn new_property(c: &str, field_name: TokenStream) -> Result<Box<dyn PropertyInfo>, ()> {
    Ok(match c {
        "Int" => Box::new(I32Property()),
        "UInt" => Box::new(U32Property()),
        "Long" => Box::new(I64Property()),
        "ULong" => Box::new(U64Property()),
        // "Double" => Box::new(DoubleProperty),
        // "String" => Box::new(StringProperty),
        // "Date" => Box::new(DateProperty),
        // "Struct" => Box::new(StructProperty),
        // "Enum" => Box::new(EnumProperty),
        // "Vec" => Box::new(VecProperty),
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


struct I32Property();

impl PropertyInfo for I32Property {

}

struct U32Property();

impl PropertyInfo for U32Property {

}

struct I64Property();

impl PropertyInfo for I64Property {

}


struct U64Property();

impl PropertyInfo for U64Property {

}

struct DoubleProperty();

impl PropertyInfo for DoubleProperty {

}

struct DateProperty();

impl PropertyInfo for DateProperty {

}

struct StringProperty();

impl PropertyInfo for StringProperty {

}

struct StructProperty();

impl PropertyInfo for StructProperty {

}

struct EnumProperty();

impl PropertyInfo for EnumProperty {

}

struct VecProperty();

impl PropertyInfo for VecProperty {

}
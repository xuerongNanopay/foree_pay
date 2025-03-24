#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;

use super::Property;

pub(crate) trait PropertyInfo {
    fn to_setter_token_stream(&self, property: &Property) -> Result<TokenStream, TokenStream> {
        Err(quote! {
            compile_error!("Do not support");
        })
    }
}

pub(crate) fn new_property_info(property: &Property) -> Result<Box<dyn PropertyInfo + '_>, TokenStream> {
    Ok(match property.property_type().as_str() {
        "Int" => Box::new(I32Property(property)),
        "UInt" => Box::new(U32Property()),
        "Long" => Box::new(I64Property()),
        "ULong" => Box::new(U64Property()),
        "Float" => Box::new(F32Property()),
        "Double" => Box::new(F64Property()),
        // "Double" => Box::new(DoubleProperty),
        // "String" => Box::new(StringProperty),
        // "Date" => Box::new(DateProperty),
        // "Struct" => Box::new(StructProperty),
        // "Enum" => Box::new(EnumProperty),
        // "Vec" => Box::new(VecProperty),
        _ => {
            let t = property.property_type();
            let txt = format!("unsupport property type `{}`", t);
            return Err(quote! {
                compile_error!(#txt);
            });
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


struct I32Property<'p>(&'p Property);

impl PropertyInfo for I32Property<'_> {

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

struct F32Property();

impl PropertyInfo for F32Property {

}

struct F64Property();

impl PropertyInfo for F64Property {

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
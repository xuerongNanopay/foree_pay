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
        "UInt" => Box::new(U32Property(property)),
        "Long" => Box::new(I64Property(property)),
        "ULong" => Box::new(U64Property(property)),
        "Float" => Box::new(F32Property(property)),
        "Double" => Box::new(F64Property(property)),
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

struct U32Property<'p>(&'p Property);

impl PropertyInfo for U32Property<'_> {

}

struct I64Property<'p>(&'p Property);

impl PropertyInfo for I64Property<'_> {

}


struct U64Property<'p>(&'p Property);

impl PropertyInfo for U64Property<'_> {

}

struct F32Property<'p>(&'p Property);

impl PropertyInfo for F32Property<'_> {

}

struct F64Property<'p>(&'p Property);

impl PropertyInfo for F64Property<'_> {

}

struct DateProperty<'p>(&'p Property);

impl PropertyInfo for DateProperty<'_> {

}

struct StringProperty<'p>(&'p Property);

impl PropertyInfo for StringProperty<'_> {

}

struct StructProperty<'p>(&'p Property);

impl PropertyInfo for StructProperty<'_> {

}

struct EnumProperty<'p>(&'p Property);

impl PropertyInfo for EnumProperty<'_> {

}

struct VecProperty<'p>(&'p Property);

impl PropertyInfo for VecProperty<'_> {

}
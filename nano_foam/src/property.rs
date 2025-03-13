#![allow(unused)]

use syn::{parse::ParseStream, punctuated::Punctuated, LitStr, Token};

use crate::token;

pub(super) struct Properties {
    properties: Punctuated<Property, Token![,]>
}

// impl syn::parse::Parse for Properties {
// }

pub(super) struct Property {
    name: Option<PropertyName>,
    class: Option<PropertyClass>,
    of: Option<PropertyOf>
}

pub(crate) struct PropertyName {
    name: LitStr,
}

impl syn::parse::Parse for PropertyName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
        })
    }
}

pub(crate) struct PropertyClass {
    class: LitStr,
}

impl syn::parse::Parse for PropertyClass {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::class>()?;
        input.parse::<syn::Token![:]>()?;
        let class: LitStr = input.parse()?;

        Ok(Self{
            class,
        })
    }
}

pub(crate) struct PropertyOf {
    of: LitStr,
}

impl syn::parse::Parse for PropertyOf {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::class>()?;
        input.parse::<syn::Token![:]>()?;
        let of: LitStr = input.parse()?;

        Ok(Self{
            of,
        })
    }
}
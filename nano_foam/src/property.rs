#![allow(unused)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::ParseStream, punctuated::Punctuated, Ident, LitStr, Token};

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

impl ToTokens for PropertyName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = Ident::new(&self.name.value(), self.name.span());
        struct_name.to_tokens(tokens);
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
#![allow(unused)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, Ident, LitStr};

mod class_token {
    syn::custom_keyword!(name);
}

pub(crate) struct StructParser {
    struct_name: StructName,
    // pub(crate) table_name: String,
}

impl syn::parse::Parse for StructParser {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let struct_name = input.parse::<StructName>()?;
        let remaining_tokens: TokenStream = input.parse()?;
        Ok(Self{
            struct_name
        })
    }
}

pub(crate) fn expand(parser: StructParser) -> TokenStream {
    let struct_name = &parser.struct_name;

    quote::quote! {
        struct #struct_name {
            a: u32,
        }
    }
}

pub(crate) struct StructName {
    name: LitStr,
    _colon: syn::Token![:],
    _ident: class_token::name,
}

impl syn::parse::Parse for StructName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _ident = input.parse()?;

        let _colon = input.parse()?;
        println!("_colon");

        let name: LitStr = input.parse()?;
        println!("name: {}", name.value());


        Ok(Self{
            name,
            _colon,
            _ident,
        })
    }
}

impl ToTokens for StructName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = Ident::new(&self.name.value(), self.name.span());
        struct_name.to_tokens(tokens);
    }
}

#![allow(unused)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{bracketed, parse::ParseStream, punctuated::Punctuated, Ident, LitStr, Token};

mod class_token {
    syn::custom_keyword!(struct_name);
    syn::custom_keyword!(imports);
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

pub(crate) struct Features {
    
}

pub(crate) struct StructName {
    name: LitStr,
    _colon: syn::Token![:],
    _key: class_token::struct_name,
}

impl syn::parse::Parse for StructName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _key = input.parse()?;
        let _colon = input.parse()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
            _colon,
            _key,
        })
    }
}

impl ToTokens for StructName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = Ident::new(&self.name.value(), self.name.span());
        struct_name.to_tokens(tokens);
    }
}

pub(crate) struct UseStatements {
    _key: class_token::imports,
    _colon: syn::Token![:],
    use_statements: Punctuated<LitStr, Token![,]>
}

impl syn::parse::Parse for UseStatements {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _key = input.parse()?;
        let _colon = input.parse()?;

        let content;
        bracketed!(content in input);

        let use_statements = Punctuated::parse_terminated(&content)?;

        Ok(Self{
            use_statements,
            _colon,
            _key,
        })
    }
}

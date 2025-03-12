#![allow(unused)]

use std::fmt::format;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Ident, LitStr, Token};

pub mod class_token {
    syn::custom_keyword!(struct_name);
    syn::custom_keyword!(use_imports);
    syn::custom_keyword!(features);
}

pub(crate) struct StructParser {
    struct_name: StructName,
    features: Features,
    // pub(crate) table_name: String,
}

impl syn::parse::Parse for StructParser {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut struct_name: Option<StructName> = None;
        let mut features: Option<Features> = None;

        'parsing: loop {
            match () {
                _ if input.peek(class_token::struct_name) => {
                    struct_name = Some(input.parse::<StructName>()?);
                },
                _ if input.peek(class_token::features) => {
                    features = Some(input.parse::<Features>()?);
                },
                _ => {
                    let remain_name: TokenStream = input.parse()?;
                    return Err(syn::Error::new(remain_name.span(), remain_name));
                }
            };

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
                
                if !input.is_empty() {
                    continue 'parsing;
                }
            }

            break 'parsing;
        }

        Ok(Self{
            struct_name: struct_name.unwrap(),
            features: features.unwrap(),
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
    _key: class_token::use_imports,
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

pub(crate) struct Features {
    _key: class_token::features,
    _colon: syn::Token![:],
    features: Punctuated<Ident, Token![,]>
}

impl syn::parse::Parse for Features {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _key = input.parse()?;
        let _colon = input.parse()?;

        let content;
        bracketed!(content in input);

        let features = Punctuated::parse_terminated(&content)?;
        println!("features: {}", features.last().unwrap());
        Ok(Self{
            features,
            _colon,
            _key,
        })
    }
}
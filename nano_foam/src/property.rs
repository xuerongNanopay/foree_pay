#![allow(unused)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{braced, bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Ident, LitStr, Token};

use crate::token;

mod types;

pub(super) struct Properties {
    properties: Punctuated<Property, Token![,]>
}

impl syn::parse::Parse for Properties {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::properties>()?;
        input.parse::<syn::Token![:]>()?;
        let content;
        bracketed!(content in input);

        let properties = Punctuated::<Property, Token![,]>::parse_terminated(&content)?;

        Ok(Self { properties })
    }
}

#[derive(Default)]
pub(super) struct Property {
    name: Option<PropertyName>,
    r#type: Option<PropertyType>,
}

impl syn::parse::Parse for Property {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        braced!(content in input);

        let mut property = Property::default();

        'parsing: loop {
            match () {
                _ if content.peek(token::name) => {
                    property.name = Some(content.parse::<PropertyName>()?);
                },
                _ if content.peek(token::r#type) => {
                    property.r#type = Some(content.parse::<PropertyType>()?);
                },
                _ => {
                    if ! content.is_empty() {
                        let remain_name: TokenStream = content.parse()?;
                        return Err(syn::Error::new(remain_name.span(), format!("unknown token start at `{}`", remain_name.to_string())));
                    }
                }
            };

            if content.peek(syn::Token![,]) {
                content.parse::<syn::Token![,]>()?;
                
                if !content.is_empty() {
                    continue 'parsing;
                }
            }

            break 'parsing;
        }

        Ok(property)
    }
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
        let name = Ident::new(&self.name.value(), self.name.span());
        name.to_tokens(tokens);
    }
}

pub(crate) struct PropertyType {
    r#type: LitStr,

}

impl syn::parse::Parse for PropertyType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::r#type>()?;
        input.parse::<syn::Token![:]>()?;
        let r#type: LitStr = input.parse()?;
        
        Ok(Self{
            r#type,
        })
    }
}
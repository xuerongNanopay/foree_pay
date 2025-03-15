#![allow(unused)]

use std::{collections::HashSet, fmt::format};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Ident, LitStr, Token};

use crate::{feature::{Feature, Features}, sql::SqlConfig, token::{self, features}};

#[derive(Default)]
pub(crate) struct StructParser {
    struct_name: Option<StructName>,
    features: Option<Features>,
    sql_config: Option<SqlConfig>,
    // pub(crate) table_name: String,
}

impl syn::parse::Parse for StructParser {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut struct_parser = StructParser::default();

        'parsing: loop {
            match () {
                _ if input.peek(token::name) => {
                    struct_parser.struct_name = Some(input.parse::<StructName>()?);
                },
                _ if input.peek(token::features) => {
                    struct_parser.features = Some(input.parse::<Features>()?);
                },
                _ if input.peek(token::sql) => {
                    struct_parser.sql_config = Some(input.parse::<SqlConfig>()?);
                },
                _ => {
                    let remain_name: TokenStream = input.parse()?;
                    return Err(syn::Error::new(remain_name.span(), format!("unknown token start at `{}`", remain_name.to_string())));
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

        Ok(struct_parser)
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
}

impl syn::parse::Parse for StructName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
        })
    }
}

impl ToTokens for StructName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = Ident::new(&self.name.value(), self.name.span());
        struct_name.to_tokens(tokens);
    }
}

pub struct UseStatements {
    use_statements: Punctuated<LitStr, Token![,]>
}

impl syn::parse::Parse for UseStatements {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::use_imports>()?;
        input.parse::<syn::Token![:]>()?;

        let content;
        bracketed!(content in input);

        let use_statements = Punctuated::parse_terminated(&content)?;
        
        Ok(Self{
            use_statements,
        })
    }
}
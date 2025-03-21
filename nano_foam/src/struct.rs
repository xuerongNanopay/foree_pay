#![allow(unused)]

use std::{collections::HashSet, fmt::format};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Ident, LitBool, LitStr, Token};

use crate::{feature::{Feature, Features}, property::Properties, sql::{SqlConfig, SqlQueries}, token::{self, features}};

#[derive(Default)]
pub(crate) struct StructParser {
    struct_name: Option<StructName>,
    features: Option<Features>,
    sql_config: Option<SqlConfig>,
    properties: Option<Properties>,
    sql_queries: Option<SqlQueries>
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
                _ if input.peek(token::properties) => {
                    struct_parser.properties = Some(input.parse::<Properties>()?);
                },
                _ if input.peek(token::sql_queries) => {
                    struct_parser.sql_queries = Some(input.parse::<SqlQueries>()?);
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

pub(crate) struct StructName {
    name: LitStr,
}

impl StructName {
    pub(crate) fn value(&self) -> String {
        self.name.value()
    }
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

pub(crate) struct UseStatements {
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

pub(crate) struct Transient {
    value: LitBool
}

impl Transient {
    fn value(&self) -> bool {
        self.value.value()
    }
}

impl syn::parse::Parse for Transient {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::transient>()?;
        input.parse::<syn::Token![:]>()?;
        
        let value = input.parse::<LitBool>()?;
        
        Ok(Self{
            value
        })
    }
}

pub(crate) fn expand(parser: StructParser) -> TokenStream {
    let struct_name = &parser.struct_name;
    let struct_name = match struct_name {
        None => {
            let txt = format!("FoamStruct require name");
            return quote::quote! {
                compile_error!(#txt);
            }
        },
        Some(s) => { s }
    };

    //TODO: Properties.
    let properties = &parser.properties;

    let properties = match properties {
        None => {
            let txt = format!("FoamStruct {} is missing properties", struct_name.value());
            return quote::quote! {
                compile_error!(#txt);
            }
        },
        Some(p) => { p }
    };

    let test = format_ident!("i32");
    let impls = quote! {
        impl #struct_name {
        }
    };

    let struct_fields = match properties.to_struct_fields_token_stream() {
        Ok(o) => o ,
        Err(e) => return e,
    };

    quote::quote! {
        struct #struct_name {
            #struct_fields
        }

        #impls
    }
}
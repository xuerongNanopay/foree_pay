#![allow(unused)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sql::PropertySqlConfig;
use syn::{braced, bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Ident, LitBool, LitStr, Token};
use types::{new_property_info, PropertyInfo};

use crate::token::{self, properties};


mod types;
mod json;
mod xml;
mod sql;

pub(super) struct Properties {
    properties: Punctuated<Property, syn::Token![,]>
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

impl Properties {
    pub fn to_struct_fields_token_stream(&self) -> Result<TokenStream, TokenStream> {
        let tmp_fields = self.properties.iter().map(|p| p.to_struct_field_token_stream()).collect::<Vec<_>>();
        
        let mut fields = Vec::with_capacity(tmp_fields.len());

        for field in tmp_fields {
            match field {
                Err(t) => { return Err(t); },
                Ok(t) => {
                    fields.push(t);
                }
            }
        }

        Ok(quote! {
            #(#fields)*
        })
    }
}

pub(super) struct Property {
    name: PropertyName,
    r#type: PropertyType,
    optional: Option<Optinal>,
    sql_config: Option<PropertySqlConfig>,
}

impl Property {
    pub(crate) fn property_name(&self) -> String {
        self.name.value()
    }

    pub(crate) fn property_type(&self) -> Result<Box<dyn PropertyInfo>, TokenStream> {
        new_property_info(&self.property_name())
    }

    pub(crate) fn to_struct_field_token_stream(&self) -> Result<TokenStream, TokenStream> {
        let name = &self.name;
        let field_type = &self.r#type;

        Ok(quote! {
            #name: #field_type,
        })
    }

    pub(crate) fn sql_column_name(&self) -> String {
        match self.sql_config.as_ref() {
            Some(sql) => {
                
            },
            _ => {},
        };

        self.property_name()
    }
}

impl syn::parse::Parse for Property {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        braced!(content in input);

        let mut name: Option<PropertyName> = None;
        let mut r#type: Option<PropertyType> = None;
        let mut optional: Option<Optinal> = None;
        let mut sql_config: Option<PropertySqlConfig> = None;
    
        'parsing: loop {
            match () {
                _ if content.peek(token::name) => {
                    name = Some(content.parse::<PropertyName>()?);
                },
                _ if content.peek(token::r#type) => {
                    r#type = Some(content.parse::<PropertyType>()?);
                },
                _ if content.peek(token::optional) => {
                    optional = Some(content.parse::<Optinal>()?);
                },
                _ if content.peek(token::sql) => {
                    sql_config = Some(content.parse::<PropertySqlConfig>()?);
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

        let name = match name {
            Some(v) => {v},
            None => {
                return Err(syn::Error::new(content.span(), format!("`name` is required in property")));
            }
        };

        let r#type = match r#type {
            Some(v) => {v},
            None => {
                return Err(syn::Error::new(content.span(), format!("`r#type` is required in property")));
            }
        };

        Ok(Self{
            name,
            r#type,
            optional,
            sql_config,
        })
    }
}

pub(crate) struct PropertyName {
    value: LitStr,
}

impl PropertyName {
    pub(crate) fn value(&self) -> String {
        self.value.value()
    }
}

impl syn::parse::Parse for PropertyName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::name>()?;
        input.parse::<syn::Token![:]>()?;
        let value: LitStr = input.parse()?;
        
        Ok(Self{
            value,
        })
    }
}


impl ToTokens for PropertyName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = Ident::new(&self.value.value(), self.value.span());
        value.to_tokens(tokens);
    }
}

pub(crate) struct PropertyType {
    value: Ident,
}

impl PropertyType {
    pub(crate) fn value(&self) -> String {
        self.value.to_string()
    }
}

impl syn::parse::Parse for PropertyType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::r#type>()?;
        input.parse::<syn::Token![:]>()?;
        let value: Ident = input.parse()?;
        
        Ok(Self{
            value,
        })
    }
}

impl ToTokens for PropertyType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.value.to_tokens(tokens);
    }
}

pub(crate) struct Optinal {
    value: LitBool
}

impl Optinal {
    fn value(&self) -> bool {
        self.value.value()
    }
}

impl syn::parse::Parse for Optinal {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::optional>()?;
        input.parse::<syn::Token![:]>()?;
        
        let value = input.parse::<LitBool>()?;
        
        Ok(Self{
            value
        })
    }
}

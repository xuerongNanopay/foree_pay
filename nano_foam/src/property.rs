#![allow(unused)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sql::PropertySql;
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

#[derive(Default)]
pub(super) struct Property {
    name: Option<PropertyName>,
    r#type: Option<PropertyType>,
    optional: Option<Optinal>,
    sql_config: Option<PropertySql>,
}

impl Property {
    pub(crate) fn property_name(&self) -> String {
        self.name.as_ref().unwrap().value()
    }

    pub(crate) fn property_type(&self) -> Result<Box<dyn PropertyInfo>, TokenStream> {
        new_property_info(&self.property_name())
    }

    pub(crate) fn to_struct_field_token_stream(&self) -> Result<TokenStream, TokenStream> {
        let name = match self.name.as_ref() {
            Some(v) => { 
                if v.value().trim() == "" {
                    let txt = format!("`name` is requireds in property.");
                    return Err(quote! {{
                        compile_error!(#txt);
                    }})
                }
                v
            },
            _ => {
                let txt = format!("`name` is requireds in property.");
                return Err(quote! {{
                    compile_error!(#txt);
                }})
            }
        };

        let field_type = match self.r#type.as_ref() {
            Some(v) => { v },
            _ => {
                let txt = format!("`r#type` is requireds in property.");
                return Err(quote! {{
                    compile_error!(#txt);
                }})
            }
        };

        Ok(quote! {
            #name: #field_type,
        })
    }

    fn validate_after_parse(&self) -> Result<(), syn::Error> {
        //TODO: 
        Ok(())
    }

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
                _ if content.peek(token::optional) => {
                    property.optional = Some(content.parse::<Optinal>()?);
                },
                _ if content.peek(token::sql) => {
                    property.sql_config = Some(content.parse::<PropertySql>()?);
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

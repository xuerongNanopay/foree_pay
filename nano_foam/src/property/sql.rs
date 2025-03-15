#![allow(unused)]

use proc_macro2::TokenStream;
use syn::{braced, parse::ParseStream, spanned::Spanned, LitStr};

use crate::{r#struct::Transient, token};

#[derive(Default)]
pub(crate) struct PropertySql {
    column_name: Option<SqlColumnName>,
    transient: Option<Transient>,
}

impl syn::parse::Parse for PropertySql {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::sql>()?;
        input.parse::<syn::Token![:]>()?;

        let content;
        braced!(content in input);

        let mut property_sql = PropertySql::default();

        'parsing: loop {
            match () {
                _ if content.peek(token::column_name) => {
                    property_sql.column_name = Some(content.parse::<SqlColumnName>()?);
                },
                _ if content.peek(token::transient) => {
                    property_sql.transient = Some(content.parse::<Transient>()?);
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

        Ok(property_sql)
    }

}

pub(crate) struct SqlColumnName {
    name: LitStr,
}

impl syn::parse::Parse for SqlColumnName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::column_name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;
        
        Ok(Self{
            name,
        })
    }
}
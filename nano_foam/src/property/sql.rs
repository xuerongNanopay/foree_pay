#![allow(unused)]

use proc_macro2::TokenStream;
use syn::{braced, parse::ParseStream, spanned::Spanned, LitStr};

use crate::{r#struct::Transient, token};

#[derive(Default)]
pub(crate) struct PropertySqlConfig {
    column_name: Option<SqlColumnName>,
    transient: Option<Transient>,
}

impl PropertySqlConfig {
    fn column_name(&self) -> Option<String> {
        match self.column_name.as_ref() {
            Some(n) => { Some(n.value()) },
            _ => None,
        }
    }
}

impl syn::parse::Parse for PropertySqlConfig {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::sql>()?;
        input.parse::<syn::Token![:]>()?;

        let content;
        braced!(content in input);

        let mut property_sql = PropertySqlConfig::default();

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
    value: LitStr,
}

impl SqlColumnName {
    pub(crate) fn value(&self) -> String {
        self.value.value()
    }
}

impl syn::parse::Parse for SqlColumnName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::column_name>()?;
        input.parse::<syn::Token![:]>()?;
        let value: LitStr = input.parse()?;
        
        Ok(Self{
            value,
        })
    }
}
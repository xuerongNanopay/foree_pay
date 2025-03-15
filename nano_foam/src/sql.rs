#![allow(unused)]

use proc_macro2::TokenStream;
use syn::{braced, parse::ParseStream, spanned::Spanned, LitStr};

use crate::token;

#[derive(Default)]
pub(crate) struct SqlConfig {
    table_name: Option<TableName>,
    db_name: Option<DBName>,
}

impl syn::parse::Parse for SqlConfig {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::sql>()?;
        input.parse::<syn::Token![:]>()?;
        let content;
        braced!(content in input);

        let mut sql_config = SqlConfig::default();

        'parsing: loop {
            match () {
                _ if content.peek(token::table_name) => {
                    sql_config.table_name = Some(content.parse::<TableName>()?);
                },
                _ if content.peek(token::db_name) => {
                    sql_config.db_name = Some(content.parse::<DBName>()?);
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
        Ok(sql_config)
    }
}

pub(crate) struct TableName {
    name: LitStr,
}

impl TableName {
    pub(crate) fn get_name(&self) -> String {
        self.name.value()
    }
}

impl syn::parse::Parse for TableName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::table_name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
        })
    }
}

pub(crate) struct DBName {
    name: LitStr,
}

impl DBName {
    pub(crate) fn get_name(&self) -> String {
        self.name.value()
    }
}

impl syn::parse::Parse for DBName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::db_name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
        })
    }
}
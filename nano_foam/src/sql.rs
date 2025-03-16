#![allow(unused)]

use proc_macro2::TokenStream;
use syn::{braced, bracketed, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, token::Token, LitStr};

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

pub(crate) struct SqlQueries {
    queries: Punctuated<SqlQuery, syn::Token![,]>
}

impl syn::parse::Parse for SqlQueries {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::sql_queries>()?;
        input.parse::<syn::Token![:]>()?;
        let content;
        bracketed!(content in input);

        let queries = Punctuated::<SqlQuery, syn::Token![,]>::parse_terminated(&content)?;

        Ok(Self { queries })
    }
}

#[derive(Default)]
pub(crate) struct SqlQuery {
    fn_name: Option<FnName>,
    query: Option<Query>,
}

impl syn::parse::Parse for SqlQuery {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        braced!(content in input);

        let mut sql_query = SqlQuery::default();

        'parsing: loop {
            match () {
                _ if content.peek(token::fn_name) => {
                    sql_query.fn_name = Some(content.parse::<FnName>()?);
                },
                _ if content.peek(token::query) => {
                    sql_query.query = Some(content.parse::<Query>()?);
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

        Ok(sql_query)
    }
}

pub(crate) struct FnName {
    name: LitStr,
}

impl FnName {
    pub(crate) fn value(&self) -> String {
        self.name.value()
    }
}

impl syn::parse::Parse for FnName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::fn_name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;
        
        Ok(Self{
            name,
        })
    }
}

pub(crate) struct Query {
    name: LitStr,
}

impl Query {
    pub(crate) fn value(&self) -> String {
        self.name.value().split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

impl syn::parse::Parse for Query {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::query>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;

        Ok(Self{
            name,
        })
    }
}
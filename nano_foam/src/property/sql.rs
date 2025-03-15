#![allow(unused)]

use syn::{parse::ParseStream, LitStr};

use crate::token;

#[derive(Default)]
pub(crate) struct PropertySql {
    column_name: Option<SqlColumnName>,

}

impl syn::parse::Parse for PropertySql {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut property_sql = PropertySql::default();

        Ok(property_sql)
    }
}

pub(crate) struct SqlColumnName {
    name: LitStr,
}

impl syn::parse::Parse for SqlColumnName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;
        
        Ok(Self{
            name,
        })
    }
}
#![allow(unused)]

use proc_macro2::TokenStream;
use syn::{braced, parse::ParseStream, spanned::Spanned, LitStr};

use crate::{r#struct::Transient, token};

#[derive(Default)]
pub(crate) struct PropertyXml {
    tag_name: Option<XmlTagName>,
    transient: Option<Transient>,
}

impl syn::parse::Parse for PropertyXml {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::xml>()?;
        input.parse::<syn::Token![:]>()?;

        let content;
        braced!(content in input);

        let mut property_sql = PropertyXml::default();

        'parsing: loop {
            match () {
                _ if content.peek(token::xml_tag) => {
                    property_sql.tag_name = Some(content.parse::<XmlTagName>()?);
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

pub(crate) struct XmlTagName {
    name: LitStr,
}

impl XmlTagName {
    pub(crate) fn value(&self) -> String {
        self.name.value()
    }
}

impl syn::parse::Parse for XmlTagName {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::table_name>()?;
        input.parse::<syn::Token![:]>()?;
        let name: LitStr = input.parse()?;
        
        Ok(Self{
            name,
        })
    }
}
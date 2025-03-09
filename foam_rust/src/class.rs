#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

pub(crate) struct ClassParser {
    pub(crate) name: String,
    // pub(crate) table_name: String,
}

impl syn::parse::Parse for ClassParser {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let remaining_tokens: TokenStream = input.parse()?;
        println!("remaining tokens `{}`", remaining_tokens);

        let name = "111".to_owned();
        
        Ok(Self{
            name
        })
    }
}

pub(crate) fn expand(parser: ClassParser) -> TokenStream {
    quote::quote! {
        struct CCCC {
            a: u32,
        }
    }
}
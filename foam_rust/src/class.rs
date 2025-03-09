#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

pub(crate) struct EntityDecl {
    pub(crate) name: String,
    // pub(crate) table_name: String,
}

impl syn::parse::Parse for EntityDecl {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let remaining_tokens: TokenStream = input.parse()?;
        println!("vvvvvv {}", remaining_tokens);

        let name = "111".to_owned();
        
        Ok(Self{
            name
        })
    }
}

pub(crate) fn expand(input: EntityDecl) -> TokenStream {
    println!("vvvvvv11");

    quote::quote! {
        struct CCCC {
            a: u32,
        }
    }
}
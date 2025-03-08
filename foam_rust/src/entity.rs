#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

pub(crate) struct EntityDecl {

}

impl syn::parse::Parse for EntityDecl {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self{})
    }
}

pub(crate) fn expand(input: EntityDecl) -> TokenStream {
    quote::quote! {
        compile_error!("aaaaa");
    }
}
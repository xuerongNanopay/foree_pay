#![allow(unused)]

use syn::{punctuated::Punctuated, Token};

use crate::token;

pub(super) struct Properties {
    _key: token::properties,
    _colon: syn::Token![:],
    properties: Punctuated<Property, Token![,]>
}

// impl syn::parse::Parse for Properties {
// }

pub(super) struct Property {

}
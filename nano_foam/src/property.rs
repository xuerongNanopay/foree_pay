#![allow(unused)]

use syn::{punctuated::Punctuated, Token};

use crate::token;

pub(super) struct Properties {
    properties: Punctuated<Property, Token![,]>
}

// impl syn::parse::Parse for Properties {
// }

pub(super) struct Property {

}
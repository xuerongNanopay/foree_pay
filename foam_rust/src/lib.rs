#![allow(unused)]

mod class;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::ParseStream, parse_macro_input, DeriveInput, Token};



#[proc_macro_derive(GettersSetters)]
pub fn getters_setters_derive(input: TokenStream) -> TokenStream {
    println!("******** Received tokens: {}", input);
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(s) = &input.data {
        match &s.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("GettersSetters only supports named fields"),
        }
    } else {
        panic!("GettersSetters can only be used on structs");
    };

    let getters = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        let getter_name = quote::format_ident!("get_{}", name.as_ref().unwrap());
        quote! {
            pub fn #getter_name(&self) -> &#ty {
                &self.#name
            }
        }
    });

    let setters = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        let setter_name = quote::format_ident!("set_{}", name.as_ref().unwrap());
        quote! {
            pub fn #setter_name(&mut self, value: #ty) {
                self.#name = value;
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#getters)*
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

use syn::{Ident, Type};
use syn::punctuated::Punctuated;
use syn::parse::{Parse};

struct StructInput {
    name: Ident, // Struct name
    fields: Punctuated<(Ident, Type), Token![,]>, // Struct fields
}

impl Parse for StructInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?; // Parse struct name
        input.parse::<Token![,]>()?; // Consume the comma

        // Parse the list of fields (field_name: Type)
        let fields = Punctuated::parse_terminated_with(input, |input| {
            let field_name: Ident = input.parse()?; // Parse field name
            input.parse::<Token![:]>()?; // Consume `:`
            let field_type: Type = input.parse()?; // Parse field type
            Ok((field_name, field_type))
        })?;

        Ok(StructInput { name, fields })
    }
}

#[proc_macro]
pub fn foam_class(input: TokenStream) -> TokenStream {
    println!("******** Received tokens: {}", input);
    let StructInput { name, fields } = parse_macro_input!(input as StructInput);

    let field_definitions = fields.iter().map(|(name, ty)| {
        quote! { pub #name: #ty }
    });

    let expanded = quote! {
        struct #name {
            #(#field_definitions),*
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

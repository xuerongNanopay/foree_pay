use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(GettersSetters)]
pub fn getters_setters_derive(input: TokenStream) -> TokenStream {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

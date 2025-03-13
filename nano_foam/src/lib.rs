use proc_macro::TokenStream;

mod r#struct;
mod feature;
mod property;
mod token;
mod types;

#[proc_macro]
pub fn foam_struct_proc(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(input) => r#struct::expand(input).into(),
        Err(e) => {
            println!("ERROR: {}", e);
            quote::quote! {
                compile_error!("Parse Error");
            }.into()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
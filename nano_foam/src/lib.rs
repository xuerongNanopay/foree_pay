use proc_macro::TokenStream;

mod r#struct;

#[proc_macro]
pub fn foam_struct_proc(input: TokenStream) -> TokenStream {
    println!("******** Received tokens: {}", input);
    match syn::parse(input) {
        Ok(input) => r#struct::expand(input).into(),
        Err(_) => {
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
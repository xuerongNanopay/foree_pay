use proc_macro::TokenStream;

mod r#struct;
mod feature;
mod property;
mod token;
mod json;
mod xml;
mod sql;
mod tests;

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
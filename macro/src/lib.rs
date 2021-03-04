extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn lazy_regex(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::LitStr);
    
    match regex_static_impl::lazy_regex(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

#[proc_macro]
pub fn regex(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::LitStr);
    
    match regex_static_impl::regex(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

use proc_macro2::TokenStream;
use quote::quote;
use regex_syntax::Parser;
use syn::LitStr;

pub fn lazy_regex(item: LitStr) -> Result<TokenStream, syn::Error> {
    let _hir = Parser::new()
        .parse(&quote! { #item }.to_string())
        .map_err(|e| syn::Error::new(item.span(), e.to_string()))?;
    Ok(quote! {
        regex_static::once_cell::sync::Lazy::<regex_static::Regex>::new(|| {
            regex_static::Regex::new(#item).unwrap()
        })
    })
}

pub fn static_regex(item: LitStr) -> Result<TokenStream, syn::Error> {
    let _hir = Parser::new()
        .parse(&quote! { #item }.to_string())
        .map_err(|e| syn::Error::new(item.span(), e.to_string()))?;
    Ok(quote! {
        {
            static regex: regex_static::once_cell::sync::OnceCell::<regex_static::Regex> = regex_static::once_cell::sync::OnceCell::new();
            regex.get_or_init(|| regex_static::Regex::new(#item).unwrap())
        }
    })
}

pub fn regex(item: LitStr) -> Result<TokenStream, syn::Error> {
    let _hir = Parser::new()
        .parse(&quote! { #item }.to_string())
        .map_err(|e| syn::Error::new(item.span(), e.to_string()))?;
    Ok(quote! {
        regex_static::Regex::new(#item).unwrap()
    })
}

mod hex;

use proc_macro::TokenStream;
use quote::quote;

/// Clean up a [`TokenStream`] by removing tokens we don't need and converting to a [`String`].
fn clean_stream(input: &TokenStream) -> String {
    let mut input = input.to_string();
    input.retain(|f| !(f.is_whitespace() || f == '"' || f == '\'' || f == '#'));
    input
}

#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let values = hex::parse(&input);
    quote! { [#(#values,)*] }.into()
}

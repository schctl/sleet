use convert_case::{Converter, Pattern};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Try to extract a name in space separated Pascal case from a file path.
#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    let conv = Converter::new()
        .set_pattern(Pattern::Capital)
        .set_delim(' ');

    let output = parse_macro_input!(input as LitStr);

    let output = conv.convert(
        output
            .value()
            .replace('"', "")
            .replace('\'', "")
            .split('/')
            .last()
            .unwrap()
            .split('\\')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap(),
    );

    quote! { #output }.into()
}

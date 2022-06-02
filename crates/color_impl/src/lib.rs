use proc_macro::TokenStream;
use quote::quote;

fn parse_stream(input: &TokenStream) -> Vec<String> {
    let mut input = input.to_string();
    // Get rid of unnecessary characters
    input.retain(|f| !(f.is_whitespace() || f == '"' || f == '\'' || f == '#'));
    // Split multiple values
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>()
}

fn rgba_from_hex(input: &str) -> [f32; 4] {
    // Single character representing all channels
    // #F -> #FFFFFFFF
    if input.len() == 1 {
        return [f32::from(u8::from_str_radix(input, 16).unwrap()) / 255.0; 4];
    }

    let input = input.chars().collect::<Vec<_>>();
    let mut out = [1.0, 1.0, 1.0, 1.0];

    // Single character for each channel
    // #EAF -> #EEAAFF
    // #EAFB -> #EEAAFFBB
    if [3, 4].contains(&input.len()) {
        for i in 0..input.len() {
            let byte = input[i].to_string().repeat(2);
            out[i] = f32::from(u8::from_str_radix(&byte, 16).unwrap()) / 255.0;
        }
    // Two characters for each channel
    // #RRGGBBAA
    } else if [6, 8].contains(&input.len()) {
        for (n, i) in (0..input.len()).step_by(2).enumerate() {
            let byte = format!("{}{}", input[i], input[i + 1]);
            out[n] = f32::from(u8::from_str_radix(&byte, 16).unwrap()) / 255.0;
        }
    } else {
        panic!("unknown format for color code")
    }

    out
}

/// Parse one or more hexadecimal color codes into their component RGBA channels.
#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    // Parse the stream into strings so its easier to handle
    let colors = parse_stream(&input);

    // Parse and quote values
    if colors.is_empty() {
        panic!("no color code provided")
    } else if colors.len() == 1 {
        let values = rgba_from_hex(&colors[0]);
        quote! { [#(#values,)*] }.into()
    } else {
        let values = colors.into_iter().map(|s| {
            rgba_from_hex(&s).to_vec() // why is this needed?
        });
        quote! { [#( [#(#values),*] ),*] }.into()
    }
}

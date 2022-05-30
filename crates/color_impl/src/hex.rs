//! Hex conversion.

use proc_macro::TokenStream;

pub fn parse(input: &TokenStream) -> Vec<f32> {
    // Get a clean token stream
    let s = crate::clean_stream(input);
    let mut chars = s.chars();

    let mut values = Vec::new();

    while let Some(i) = chars.next() {
        let second = chars.next().unwrap();

        // Both digits of the value
        let num = format!("{}{}", i, second);

        // Convert to u8
        let val = u8::from_str_radix(&num, 16).unwrap();

        // Push value as float normalized to [0, 1]
        values.push(f32::from(val) / 255.0);
    }

    values
}

use sleet::stylesheets;
use sleet::color::hex;
use iced_core::{Color, Background};

fn get_col() -> Color {
    [0xff as f32, 0x66 as f32, 0xab as f32].into()
}

stylesheets! {
    pub text_input SomeInput {
        active(): {
            background: Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
        focused(): {
            background: Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
        hovered(): {
            background: Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
        value_color() -> Color: {
            Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
        },
        selection_color() -> Color: hex!(#ff00ab).into(),
        placeholder_color() -> Color: get_col(),
    }
}

fn main() {}

use sleet::stylesheets;
use iced_core::{Color, Background};
use iced_style::container::Style;

const STYLE: Style = Style {
    text_color: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
    background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
    border_radius: 0.5,
    border_width: 2.0,
    border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
};

stylesheets! {
    container SomeContainer1 {
        style: {
            ..STYLE
        },
    }

    pub container SomeContainer2 {
        style: {
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            ..STYLE
        },
    }

    container SomeContainer0 {
        style: {
            text_color: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            background: Some(Background::Color(Color::new(1.0, 1.0, 1.0, 1.0))),
            ..Default::default()
        },
    }
}

fn main() {}

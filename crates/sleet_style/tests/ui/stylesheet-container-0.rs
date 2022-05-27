use sleet_style::stylesheet;
use iced_core::{Color, Background};

stylesheet! {
    pub container SomeContainer0 {
        style: {
            text_color: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
    }
}

stylesheet! {
    pub native container SomeContainer1 {
        style: {
            text_color: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
    }
}

stylesheet! {
    pub pure container SomeContainer2 {
        style: {
            text_color: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        },
    }
}

fn main() {}

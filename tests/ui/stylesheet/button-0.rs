use sleet::stylesheets;
use iced_core::{Color, Background, Vector};

stylesheets! {
    button SomeButton0 {
        active(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        },
        hovered(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        },
        pressed(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        },
        disabled(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    pub button SomeButton1 {
        active(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        },
        hovered(): {
            text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
}

fn main() {}

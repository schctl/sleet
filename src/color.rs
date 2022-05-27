//! Color related helpers.

#![allow(dead_code)]

use iced_core::Color;

#[cfg(feature = "palette")]
use palette::{
    encoding::Srgb as SrgbEncoding,
    rgb::{Rgb, RgbStandard, Rgba, Srgb, Srgba},
    Component, IntoComponent,
};

/// A trait that represents something that can be converted into a [`Color`].
pub trait IntoColor {
    fn into_color(self) -> Color;
}

#[cfg(feature = "palette")]
impl<S, C> IntoColor for Rgb<S, C>
where
    S: RgbStandard<Space = SrgbEncoding>,
    C: Component + IntoComponent<f32>,
{
    fn into_color(self) -> Color {
        let rgbf: Rgb<S, f32> = self.into_format();
        let srgb: Srgb<f32> = rgbf.into_encoding();

        Color {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
            a: 1.0,
        }
    }
}

#[cfg(feature = "palette")]
impl<S, C> IntoColor for Rgba<S, C>
where
    S: RgbStandard<Space = SrgbEncoding>,
    C: Component + IntoComponent<f32>,
{
    fn into_color(self) -> Color {
        let rgbaf: Rgba<S, f32> = self.into_format();
        let srgba: Srgba<f32> = rgbaf.into_encoding();

        Color {
            r: srgba.red,
            g: srgba.green,
            b: srgba.blue,
            a: srgba.alpha,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn are_colors_eq(a: Color, b: Color) -> bool {
        // arbitrary value that seemed low enough
        const TOLERANCE: f32 = 0.0000001;

        (a.r - b.r).abs() < TOLERANCE
            && (a.g - b.g).abs() < TOLERANCE
            && (a.b - b.b).abs() < TOLERANCE
            && (a.a - b.a).abs() < TOLERANCE
    }

    #[test]
    #[cfg(feature = "palette")]
    fn palette_colors() {
        assert!(are_colors_eq(
            Srgb::new(1.0, 1.0, 1.0).into_color(),
            Color::WHITE
        ));

        assert!(are_colors_eq(
            Srgb::new(0.0, 0.0, 0.0).into_color(),
            Color::BLACK
        ));

        assert!(are_colors_eq(
            Srgba::new(0.0, 0.0, 0.0, 0.0).into_color(),
            Color::TRANSPARENT
        ));

        assert!(are_colors_eq(
            Srgba::from(0xef7c8eff).into_color(),
            Color {
                r: 0xef as f32 / 255.0,
                g: 0x7c as f32 / 255.0,
                b: 0x8e as f32 / 255.0,
                a: 1.0
            }
        ));

        assert!(are_colors_eq(
            Srgba::from(0x8fdde780).into_color(),
            Color {
                r: 0x8f as f32 / 255.0,
                g: 0xdd as f32 / 255.0,
                b: 0xe7 as f32 / 255.0,
                a: 0x80 as f32 / 255.0
            }
        ));
    }
}

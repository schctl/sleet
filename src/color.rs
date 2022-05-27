//! Color related helpers.

#![allow(dead_code)]

use iced_core::Color;

#[cfg(feature = "palette")]
use palette::{
    encoding::Srgb as SrgbEncoding,
    rgb::{Rgb, RgbStandard, Rgba, Srgb, Srgba},
    Component, FromComponent, IntoComponent,
};

/// A trait that represents something that can be converted into a [`Color`].
pub trait IntoColor {
    fn into_color(self) -> Color;
}

// TODO: Maybe this can be merged with iced?

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

/// A trait that represents something that can be converted from a [`Color`].
pub trait FromColor {
    fn from_color(color: Color) -> Self;
}

#[cfg(feature = "palette")]
impl<S, C> FromColor for Rgb<S, C>
where
    S: RgbStandard<Space = SrgbEncoding>,
    C: Component + FromComponent<f32>,
{
    fn from_color(color: Color) -> Self {
        let srgb: Srgb<f32> = Srgb::new(color.r, color.g, color.b);
        let rgbf: Rgb<S, f32> = srgb.into_encoding();
        rgbf.into_format()
    }
}

#[cfg(feature = "palette")]
impl<S, C> FromColor for Rgba<S, C>
where
    S: RgbStandard<Space = SrgbEncoding>,
    C: Component + FromComponent<f32>,
{
    fn from_color(color: Color) -> Self {
        let srgba: Srgba<f32> = Srgba::new(color.r, color.g, color.b, color.a);
        let rgbaf: Rgba<S, f32> = srgba.into_encoding();
        rgbaf.into_format()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check if two colors are the same, given a tolerance between channels.
    fn are_colors_eq(a: Color, b: Color, t: f32) -> bool {
        // Check if the difference between each channel is less
        // than the allowed tolerance.
        (a.r - b.r).abs() < t
            && (a.g - b.g).abs() < t
            && (a.b - b.b).abs() < t
            && (a.a - b.a).abs() < t
    }

    /// Check if two colors are the same, with a higher tolerance for differences.
    fn are_colors_eq_lenient(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.009)
    }

    /// Check if two colors are the same.
    fn are_colors_eq_comfy(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.0000001)
    }

    /// Check if two colors are the same, with a higher tolerance for differences.
    fn are_colors_eq_strict(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.0000005)
    }

    /// Test conversion between a few [`Srgb`], [`Srgba`] and [`Color`].
    #[test]
    #[cfg(feature = "palette")]
    fn palette_colors() {
        assert!(are_colors_eq_strict(
            Srgb::new(1.0, 1.0, 1.0).into_color(),
            Color::WHITE
        ));

        assert!(are_colors_eq_strict(
            Srgb::new(0.0, 0.0, 0.0).into_color(),
            Color::BLACK
        ));

        assert!(are_colors_eq_strict(
            Srgba::new(0.0, 0.0, 0.0, 0.0).into_color(),
            Color::TRANSPARENT
        ));

        assert!(are_colors_eq_comfy(
            Srgba::from(0xef7c8eff).into_color(),
            Color {
                r: 0xef as f32 / 255.0,
                g: 0x7c as f32 / 255.0,
                b: 0x8e as f32 / 255.0,
                a: 1.0
            }
        ));

        assert!(are_colors_eq_comfy(
            Srgba::from(0x8fdde780).into_color(),
            Color {
                r: 0x8f as f32 / 255.0,
                g: 0xdd as f32 / 255.0,
                b: 0xe7 as f32 / 255.0,
                a: 0x80 as f32 / 255.0
            }
        ));
    }

    /// Do a roundtrip conversion between colors.
    #[test]
    #[cfg(feature = "palette")]
    fn palette_roundtrip() {
        let s1 = Color::new(0.2, 0.2, 0.2, 1.0);
        let n1: Srgba<f32> = Srgba::from_color(s1);

        assert!(are_colors_eq_strict(
            n1.into_color(),
            Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0
            }
        ));
    }

    /// Do a roundtrip conversion between colors, with format changing in between.
    #[test]
    #[cfg(feature = "palette")]
    fn palette_roundtrip_format() {
        let s1 = Color::new(0.2, 0.2, 0.2, 1.0);

        // Format changing
        let n1: Srgba<u8> = Srgba::from_color(s1);
        let n1: Srgba<u32> = n1.into_format();

        assert!(are_colors_eq_lenient(
            n1.into_color(),
            Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0
            }
        ));
    }

    /// Do a roundtrip conversion between colors, with manipulation in between.
    #[test]
    #[cfg(feature = "palette")]
    fn palette_roundtrip_manipulation() {
        use palette::Blend;

        // Initialize colors
        let s1 = Color::new(0.2, 0.2, 0.2, 1.0);
        let s2 = Color::new(0.4, 0.1, 0.5, 1.0);

        // Convert into Srgba
        let n1: Srgba<f32> = Srgba::from_color(s1);
        let n2: Srgba<f32> = Srgba::from_color(s2);

        // Convert into linear
        let l1 = n1.into_linear();
        let l2 = n2.into_linear();

        // Lighten
        let lighter = l1.lighten(l2);
        let lighter = lighter.into_color();

        assert!(are_colors_eq_strict(
            lighter,
            Color {
                r: 0.4,
                g: 0.2,
                b: 0.5,
                a: 1.0
            }
        ));
    }

    /// Do a roundtrip conversion between colors, with manipulation and format changing in between.
    #[test]
    #[cfg(feature = "palette")]
    fn palette_roundtrip_manipulation_format() {
        use palette::Blend;

        // Initialize colors
        let s1 = Color::new(0.2, 0.2, 0.2, 1.0);
        let s2 = Color::new(0.4, 0.1, 0.5, 1.0);

        // Convert into Srgba
        let n1: Srgba<u8> = Srgba::from_color(s1);
        let n1: Srgba<f32> = n1.into_format();

        let n2: Srgba<u8> = Srgba::from_color(s2);
        let n2: Srgba<f32> = n2.into_format();

        // Convert into linear
        let l1 = n1.into_linear();
        let l2 = n2.into_linear();

        // Lighten
        let lighter = l1.lighten(l2);
        let lighter = lighter.into_color();

        // Check
        assert!(are_colors_eq_lenient(
            lighter,
            Color {
                r: 0.4,
                g: 0.2,
                b: 0.5,
                a: 1.0
            }
        ));
    }
}

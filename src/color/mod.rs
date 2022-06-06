//! Color related helpers.

pub mod into_color;
pub use iced_core::Color;

/// Parse a hexadecimal color code.
///
/// # Examples
/// ```rust
/// use sleet::color::hex;
/// use iced_core::Color;
///
/// assert_eq!(Color::WHITE, hex!(FFFFFF).into());
/// assert_eq!([0.0, 0.0, 0.0, 0.0], hex!(00000000));
/// ```
pub use hex_impl::hex;

#[allow(dead_code)]
mod util {
    use super::Color;

    /// Check if two colors are the same, given a tolerance between channels.
    pub fn are_colors_eq(a: Color, b: Color, t: f32) -> bool {
        // Check if the difference between each channel is less
        // than the allowed tolerance.
        (a.r - b.r).abs() < t
            && (a.g - b.g).abs() < t
            && (a.b - b.b).abs() < t
            && (a.a - b.a).abs() < t
    }

    /// Check if two colors are the same, with a higher tolerance for differences.
    #[inline]
    pub fn are_colors_eq_lenient(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.009)
    }

    /// Check if two colors are the same.
    #[inline]
    pub fn are_colors_eq_comfy(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.0000001)
    }

    /// Check if two colors are the same, with a higher tolerance for differences.
    #[inline]
    pub fn are_colors_eq_strict(a: Color, b: Color) -> bool {
        are_colors_eq(a, b, 0.0000005)
    }
}

/// Shorthand for [`Color::from_rgb`].
#[inline]
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}

/// Shorthand for [`Color::from_rgba`].
#[inline]
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::from_rgba(r, g, b, a)
}

/// Shorthand for [`Color::from_rgb8`].
#[inline]
pub fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}

/// Shorthand for [`Color::from_rgba8`].
#[inline]
pub fn rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::from_rgba8(r, g, b, a)
}

/// Alias for [`hsv`].
#[inline]
pub fn hsl(hue: u16, sat: f32, val: f32) -> Color {
    hsv(hue, sat, val)
}

/// Alias for [`hsva`].
#[inline]
pub fn hsla(hue: u16, sat: f32, val: f32, a: f32) -> Color {
    hsva(hue, sat, val, a)
}

/// Convert an HSVA color into a [`Color`].
#[inline]
pub fn hsva(hue: u16, sat: f32, val: f32, a: f32) -> Color {
    debug_assert!((0.0..=1.0).contains(&a));

    Color {
        a,
        ..hsv(hue, sat, val)
    }
}

/// Convert an HSV color into a [`Color`].
///
/// Based on [this](https://cs.stackexchange.com/questions/64549/convert-hsv-to-rgb-colors)
/// stackexchange post.
pub fn hsv(hue: u16, sat: f32, val: f32) -> Color {
    debug_assert!(hue <= 360);
    debug_assert!((0.0..=1.0).contains(&sat));
    debug_assert!((0.0..=1.0).contains(&val));

    // Maximum of (R, G, B) values
    let max = val;
    // Difference between maximum and minimum
    let chroma = sat * val;
    // Minimum of (R, G, B) values
    let min = max - chroma;

    // Will be in [-1, 5)
    let h1 = if hue <= 300 {
        hue as i16
    } else {
        hue as i16 - 360
    } as f32
        / 60.0;

    let r;
    let g;
    let b;

    // Solve for (R, G, B)
    if h1 < 0.0 {
        r = max;
        g = min;
        b = g - h1 * chroma;
    } else if h1 < 1.0 {
        r = max;
        b = min;
        g = b + h1 * chroma;
    } else if h1 < 2.0 {
        g = max;
        b = min;
        r = b - (h1 - 2.0) * chroma;
    } else if h1 < 3.0 {
        g = max;
        r = min;
        b = r + (h1 - 2.0) * chroma;
    } else if h1 < 4.0 {
        b = max;
        r = min;
        g = r - (h1 - 4.0) * chroma;
    } else {
        b = max;
        g = min;
        r = g + (h1 - 4.0) * chroma;
    }

    Color { r, g, b, a: 1.0 }
}

#[cfg(test)]
mod tests {
    use super::util::*;
    use super::*;

    #[test]
    fn convert_hsv() {
        assert!(are_colors_eq_lenient(
            rgb8(250, 38, 160),
            hsv(325, 0.85, 0.98)
        ));
        assert!(are_colors_eq_lenient(
            hex!(2ff3e0).into(),
            hsl(174, 0.81, 0.95)
        ));
        assert!(are_colors_eq_lenient(
            hex!(#a4939390).into(),
            hsva(0, 0.10, 0.64, 144.0 / 255.0)
        ));
        assert!(are_colors_eq_lenient(
            hex!(#76b947).into(),
            hsla(95, 0.62, 0.73, 1.0)
        ));
    }

    #[test]
    #[should_panic]
    fn convert_hsv_fail() {
        hsv(400, 0.5, 0.9);
        hsl(0, -0.5, 0.9);
        hsva(0, -0.5, 1.2, 1.0);
    }
}

//! # sleet
//!
//! `sleet` is a helper library in Rust for styling applications made with [`iced`]. Currently, `sleet`
//! provides helpers for dealing with widget stylesheets, and definitions for a few common color schemes.
//!
//! [`iced`]: https://github.com/iced-rs/iced/

// Macros
mod stylesheet;

/// Parse a hexadecimal color code.
///
/// # Examples
/// ```rust
/// use sleet::hex;
/// use iced_core::Color;
///
/// assert_eq!(Color::WHITE, hex!(FFFFFF).into());
/// ```
pub use sleet_color_impl::hex;

/// Other
pub mod color;
#[cfg(feature = "colors-extra")]
#[rustfmt::skip]
pub mod colorscheme;

// Re-exports
pub use iced_core;
pub use iced_style;

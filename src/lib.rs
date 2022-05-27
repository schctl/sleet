//! # sleet
//!
//! `sleet` is a helper library in Rust for styling applications made with [`iced`]. Currently, `sleet`
//! provides helpers for dealing with widget stylesheets, and definitions for a few common color schemes.
//!
//! [`iced`]: https://github.com/iced-rs/iced/

pub mod color;
pub mod stylesheet;

#[cfg(feature = "colors-extra")]
#[rustfmt::skip]
pub mod colorscheme;

pub use iced_core;
pub use iced_pure;
pub use iced_style;

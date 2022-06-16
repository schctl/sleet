//! # sleet
//!
//! `sleet` is a helper library in Rust for styling applications made with [`iced`]. Currently, `sleet`
//! provides helpers for dealing with widget stylesheets, and definitions for a few common color schemes.
//!
//! [`iced`]: https://github.com/iced-rs/iced/

pub mod color;
mod font;
mod stylesheet;
#[rustfmt::skip]
pub mod colorscheme;

// Re-exports
pub use iced_core;
pub use iced_style;
#[doc(hidden)]
pub use name_from_path;

//! Helpers for working with fonts.

/// A macro for defining fonts.
///
/// The general usage of this macro is:
///
/// ```rust,ignore
/// fonts! {
///     {name}: {value}
/// }
/// ```
///
/// Where `{value}` can be an expression returning a [`Font`](iced_core::Font), or a path
/// to a font file.
///
/// # Examples
///
/// ```rust,ignore
/// use sleet::fonts;
///
/// fonts! {
///     pub ROBOTO: "res/fonts/RobotoMono-Regular.ttf"
/// }
/// ```
#[macro_export]
macro_rules! fonts {
    (
        $(
            $vis:vis $name:ident: $value:expr
        ),+
    ) => {
        $(
            $vis const $name: $crate::iced_core::Font = $crate::__p_font_from_expr!($value);
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __p_font_from_expr {
    ($path:literal) => {
        $crate::iced_core::Font::External {
            name: $crate::name_from_path::parse!($path),
            bytes: ::std::include_bytes!($path),
        }
    };

    ($other:expr) => {
        $other
    };
}

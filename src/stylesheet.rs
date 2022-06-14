//! Helpers for widget stylesheets.

/// Generic macro for auto-implementing the `StyleSheet` trait for a widget.
///
/// This is the general format for using this macro:
///
/// ```rust,ignore
/// stylesheets! {
///     {pub} {widget} {name} {
///         {attr}({args}): {style},
///     }
/// }
/// ```
///
/// - `{attr}` is the trait method to be implemented.
/// - `{style}` is the structure of the `Style` to be returned. This is specified in the usual
/// struct construction form.
///
/// # Examples
///
/// ## Container
///
/// See [`container::StyleSheet`](https://docs.rs/iced_style/latest/iced_style/container/trait.StyleSheet.html).
///
/// ```rust
/// use sleet::stylesheets;
/// use iced_core::{Color, Background};
///
/// stylesheets! {
///     container SomeContainer {
///         style(): {
///             text_color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
///             background: Some(Background::Color(Color::new(0.0, 0.0, 0.0, 1.0))),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color::new(1.0, 0.0, 1.0, 1.0),
///         },
///     }
/// }
/// ```
///
/// ## Button
///
/// See [`button::StyleSheet`](https://docs.rs/iced_style/latest/iced_style/button/trait.StyleSheet.html).
///
/// ```rust
/// use sleet::stylesheets;
/// use iced_core::{Color, Background, Vector};
///
/// stylesheets! {
///     pub button SomeButton {
///         active(): {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///         hovered(): {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///         pressed(): {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             ..Default::default()
///         },
///     }
/// }
/// ```
#[macro_export]
macro_rules! stylesheets {
    (
        $(
            $vis:vis $ty:ident $name:ident {
                $($method:ident($($arg:ident: $argty:ty),*): {
                    $($field:ident: $value:expr,)*
                    $(..$cont:expr)?
                },)*
            }
        )+
    ) => {
        $(
            $vis struct $name;

            impl $crate::iced_style::$ty::StyleSheet for $name {
                $(
                    fn $method(&self $(, $arg: $argty)*) -> $crate::iced_style::$ty::Style {
                        $crate::iced_style::$ty::Style {
                            $($field: $value,)*
                            $(..$cont)?
                        }
                    }
                )*
            }
        )+
    };
}

//! Helpers for widget stylesheets.

/// Generic macro for auto-implementing widget stylesheets.
///
/// This is the general format for using this macro:
///
/// ```rust,ignore
/// stylesheet! {
///     {pub} {widget} {name} {
///         {attr}: {style definition},
///     }
/// }
/// ```
///
/// `{attr}` is the trait method to be implemented, and `{style definition}` is the structure to be
/// returned. See the [`iced` docs](https://docs.rs/iced_style/latest/iced_style/) to see the
/// corresponding definitions for widgets.
///
/// # Examples
///
/// ## [Container](https://docs.rs/iced_style/latest/iced_style/container/trait.StyleSheet.html)
///
/// ```rust
/// use sleet::stylesheet;
/// use iced_core::{Color, Background};
///
/// stylesheet! {
///     container SomeContainer {
///         style: {
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
/// ## [Button](https://docs.rs/iced_style/latest/iced_style/button/trait.StyleSheet.html)
///
/// ```rust
/// use sleet::stylesheet;
/// use iced_core::{Color, Background, Vector};
///
/// stylesheet! {
///     pub button SomeButton {
///         active: {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///         hovered: {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///         pressed: {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///         disabled: {
///             text_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             background: Some(Background::Color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })),
///             border_radius: 0.5,
///             border_width: 2.0,
///             border_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
///             shadow_offset: Vector::new(0.0, 0.0),
///         },
///     }
///     }
/// ```
#[macro_export]
macro_rules! stylesheet {
    (
        $vis:vis $ty:ident $name:ident {
            $($method:ident: { $($field:ident: $value:expr,)* },)*
        }
    ) => {
        $vis struct $name {}

        impl $crate::iced_style::$ty::StyleSheet for $name {
            $(
                fn $method(&self) -> $crate::iced_style::$ty::Style {
                    $crate::iced_style::$ty::Style {
                        $($field: $value,)*
                    }
                }
            )*
        }
    };
}

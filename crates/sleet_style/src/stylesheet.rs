#[macro_export]
macro_rules! stylesheet {
    // Native by default
    (
        $vis:vis $ty:ident $name:ident {
            $($method:ident: { $($field:ident: $value:expr,)* },)*
        }
    ) => {
        $crate::stylesheet! {
            $vis native $ty $name {
                $($method: { $($field: $value,)* },)*
            }
        }
    };

    // Native widgets
    (
        $vis:vis native $ty:ident $name:ident {
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

    // Pure widgets
    (
        $vis:vis pure $ty:ident $name:ident {
            $($method:ident: { $($field:ident: $value:expr,)* },)*
        }
    ) => {
        $vis struct $name {}

        impl $crate::iced_pure::widget::$ty::StyleSheet for $name {
            $(
                fn $method(&self) -> $crate::iced_pure::widget::$ty::Style {
                    $crate::iced_pure::widget::$ty::Style {
                        $($field: $value,)*
                    }
                }
            )*
        }
    };
}

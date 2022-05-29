# sleet

`sleet` is a helper library in Rust for styling applications made with [`iced`].

Use `sleet` in your application by adding it as a dependency in its `Cargo.toml`:

```toml
[dependencies]
sleet = { git = "https://github.com/schctl/sleet" }
```

### Features

- `sleet` provides some pre-built colorschemes that can be activated using the `colors-extra` feature.

- `sleet` provides the `stylesheets!` macro to define widget stylesheets more concisely. An example is
shown below:

```rust
use sleet::stylesheets;
use sleet::colorscheme::catppuccin::frappe;

stylesheets! {
    container SomeContainer {
        style: {
            text_color: Some(frappe::TEXT),
            background: Some(frappe::SURFACE_0),
            border_radius: 0.5,
            border_width: 2.0,
            border_color: frappe::OVERLAY_0,
        },
    }
}
```

[`iced`]: https://github.com/iced-rs/iced/

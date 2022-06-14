# sleet

`sleet` is a highly opinionated helper library for styling applications made with [`iced`]. I created
this library for my own use cases and how I prefer writing code.

Use `sleet` in your application by adding it as a dependency in its `Cargo.toml`:

```toml
[dependencies]
sleet = { git = "https://github.com/schctl/sleet" }
```

### Features

- Some pre-defined colorschemes that can be activated using the `colorschemes` feature.

- A few `color` related helpers (like the `hex!` macro for parsing hexadecimal colors).

- `stylesheets!` macro to define widget stylesheets more concisely. An example is
shown below:

See the [documentation].

[`iced`]: https://github.com/iced-rs/iced/
[documentation]: https://schctl.github.io/sleet/

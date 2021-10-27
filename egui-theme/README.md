# egui-theme: A container for reproducible styles and visual information between egui applications.
[![Latest version](https://img.shields.io/crates/v/egui-theme.svg)](https://crates.io/crates/egui-theme)
[![Documentation](https://docs.rs/egui-theme/badge.svg)](https://docs.rs/egui-theme)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

`egui-theme` defines a common interface for serializing and storing [egui](https://crates.io/crates/egui)'s style and font data for use between applications. The goal is to allow users of `egui` to easily create and share themes between relevant applications.

`egui-theme` makes no assumptions about serialization format and will support any [serde](https://crates.io/crates/serde)).

## How to use

The following example demonstrates how to create an `EguiTheme` and serialize it and deserialize it.

```rust
let theme = EguiTheme::new(
    Style::default(),
    FontDefinitions::default(),
);
let serialized_theme = ron::to_string(&theme).expect("this should serialize"):
let theme = ron::from_string::<EguiTheme>(serialized_theme).expect("this should deserialize");
let (style, font_definitions) = theme.extract();
```

After this point you can set the style and font definitions using [`egui::Context::set_style`](https://docs.rs/egui/0.14.2/egui/struct.Context.html#method.set_style) and [`egui::Context::set_fonts`](https://docs.rs/egui/0.14.2/egui/struct.Context.html#method.set_fonts) respectively.

## Migrating old egui-themes.

The migrations module contains functions that can be used to migrate old themes to new theme versions when a theme is added. Use the migration with the relevant egui theme to update the data to reflect the specific version migrations.
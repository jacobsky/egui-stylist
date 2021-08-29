# egui styler

This project is to help create reusable themes that can be serialized into and from egui. This way it should be much easier to track, design and skin egui natively inside of egui.

This library also strives to be embeddable into any existing application.

While egui already has the ability to natively make changes to the settings directly by using `Context::settings_ui()` this is not necessarily optimal for theme creation. This tool seeks to optimize this process.

This library is available as a native application which can be run with `cargo run` or `cargo run --release` or can be embedded such as how this has been embedded in an addon for [godot-egui](https://github.com/setzer22/godot-egui)

## Embedding in other applications

This library is built with [egui](https://docs.rs/egui/) and can be run as a widget in any egui application. To do so, include this application as a dependency in your Cargo.toml file and use the `views::StylerView` directly.

If you only require a part of the application, you can also include only the views that you require for your application.

## Contributions

Contributions to should be made in the form of GitHub pull requests. I will be happy to review and include any additional changes that make sense for the project.

Please make sure to run `cargo fmt` and `cargo clippy` before submitting your pull requests. To keep the main repository as clean as possible, please also ensure that the repository has any `warnings` from the compiler fixed.

All contributions freely made to this projects are licensed as per the terms of the MIT License.
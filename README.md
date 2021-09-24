# egui stylist

[![Latest version](https://img.shields.io/crates/v/egui-stylist.svg)](https://crates.io/crates/egui-stylist)
[![Documentation](https://docs.rs/egui-stylist/badge.svg)](https://docs.rs/egui-stylist)
[![Build Status](https://github.com/jacobsky/egui-stylist/workflows/CI/badge.svg)](https://github.com/jacobsky/egui-stylist/actions?workflow=CI)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

This project is to help create reusable themes that can be serialized into and from egui. This way it should be much easier to track, design and skin egui natively inside of egui.

This library also strives to be embeddable into any existing application and was built with the intent of embedding itself as an editor script in [Godot Engine](https://godotengine.org/) leveraging [godot-egui](https://github.com/setzer22/godot-egui)

While egui already has the ability to natively make changes to the settings directly by using `Context::settings_ui()` this is not necessarily optimal for theme creation. This tool seeks to optimize this process.

This library is available as a native application which can be built using `cargo build` or `cargo build --release` and run natively or embedded in a game engine, such as the sample available with [godot-egui](https://github.com/setzer22/godot-egui)
## Embedding in other applications

This library is built with [egui](https://docs.rs/egui/) and can be run as a widget in any egui application. To do so, include this application as a dependency in your Cargo.toml file and use the `StylerState` and `EguiTheme` types directly directly.

See more about this in the [egui-stylist](egui-stylist/README.md) readme file.

## Road Map

- [ ] Enhance the Theme Previewer to move away from the using the Demo widget gallery and be more comprehensive
- [ ] Theme inspector
- [ ] Small set of selected default themes
## Contributions

Contributions to should be made in the form of GitHub pull requests. I will be happy to review and include any additional changes that make sense for the project.

Please make sure to run `cargo fmt` and `cargo clippy` before submitting your pull requests. To keep the main repository as clean as possible, please also ensure that the repository has any `warnings` from the compiler fixed. Also, as `egui` does not require any use of `unsafe` at this moment, usage of unsafe is currently forbidden in this project.

All contributions freely made to this projects are licensed as per the terms of the MIT License.


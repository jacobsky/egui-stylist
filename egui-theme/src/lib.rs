#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod theme;
pub use theme::EguiTheme;
#[cfg(test)]
mod tests;

const DEFAULT_FONTS: [&str; 4] = [
    "Hack",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];

const EGUI_VERSION: &str = "0.18";
const EGUI_THEME_VERSION: &str = "0.2.0";

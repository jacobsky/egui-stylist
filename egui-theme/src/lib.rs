#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod theme;
pub use theme::EguiTheme;
#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/generated/meta.rs"));

const DEFAULT_FONTS: [&str; 4] = [
    "Hack",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];

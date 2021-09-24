#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod views;
pub use views::{StylistFileDialog, StylistFileDialogFunction, StylistState};

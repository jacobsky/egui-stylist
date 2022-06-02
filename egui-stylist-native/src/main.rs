#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::StylistApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app_name = "egui-stylist";
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Box::new(StylistApp::new(cc))),
    );
}

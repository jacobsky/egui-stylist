use crate::{StylerFileDialog, StylerState};
use eframe::{egui, epi};
use std::fs::File;
use std::io::Read;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
use std::path::PathBuf;

fn open_error_window(ctx: &egui::CtxRef, title: &str, text: &str, open: &mut bool) {
    let window = egui::Window::new(title.to_owned())
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
        .auto_sized()
        .collapsible(false)
        .scroll(false)
        .open(open);
    window.show(ctx, |ui| {
        ui.colored_label(egui::Color32::RED, text);
    });
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct StylerApp {
    // This is the main
    state: StylerState,
    show_error_window: bool,
    error_msg: String,
}

impl Default for StylerApp {
    fn default() -> Self {
        Self {
            state: StylerState::default(),
            show_error_window: false,
            error_msg: "".to_owned(),
        }
    }
}
/// Native filedialogs for Windows, Unix and MacOs via rfd crate.
#[cfg(not(target_arch = "wasm32"))]
fn open_file_dialog(kind: StylerFileDialog, filter: Option<(&str, &[&str])>) -> Option<PathBuf> {
    // Option a popup to save the file to a given directory
    let path = std::env::current_dir().expect("there should be a current directory");
    match kind {
        StylerFileDialog::Open => {
            let mut builder = rfd::FileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            builder.set_directory(&path).pick_file()
        }
        StylerFileDialog::Save => {
            let mut builder = rfd::FileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            builder.set_directory(&path).save_file()
        }
    }
}

// WASM specific settings due to different level of supprot in wasm32
#[cfg(target_arch = "wasm32")]
fn open_file_dialog(
    file_dialog: StylerFileDialog,
    filter: Option<(&str, &[&str])>,
) -> Option<PathBuf> {
    use futures::executor::block_on;
    match file_dialog {
        StylerFileDialog::Open => {
            let mut builder = rfd::AsyncFileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            block_on(builder.pick_file());
        }
        StylerFileDialog::Save => {
            // Save file dialogs are not supported with rfd at this time.
            None
        }
    }
}

impl epi::App for StylerApp {
    fn name(&self) -> &str {
        "egui styler"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        self.state
            .set_file_dialog_function(Box::new(open_file_dialog));
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Save").clicked() {
                        // Option a popup to save the file to a given directory
                        if let Some(path) = self.state.file_dialog(
                            StylerFileDialog::Save,
                            Some(("eguitheme", &["ron", "eguitheme"])),
                        ) {
                            let theme = self.state.export_theme();
                            match ron::to_string(&theme) {
                                Ok(value) => match File::create(path) {
                                    Ok(mut f) => {
                                        if let Err(err) = f.write_all(value.as_bytes()) {
                                            self.error_msg = format!("Saving failed with {}", err);
                                            self.show_error_window = true;
                                        }
                                    }
                                    Err(err) => {
                                        self.error_msg =
                                            format!("Creating file failed with {}", err);
                                        self.show_error_window = true;
                                    }
                                },
                                Err(err) => {
                                    self.error_msg =
                                        format!("Opening file path failed with {}", err);
                                    self.show_error_window = true;
                                }
                            }
                        }
                    }
                    if ui.button("Load").clicked() {
                        if let Some(path) = self.state.file_dialog(
                            StylerFileDialog::Open,
                            Some(("eguitheme", &["ron", "eguitheme"])),
                        ) {
                            match File::open(path) {
                                Ok(mut f) => {
                                    let mut buf = String::new();
                                    f.read_to_string(&mut buf).expect("this should work");
                                    match ron::from_str(&buf) {
                                        Ok(theme) => self.state.import_theme(theme),
                                        Err(err) => {
                                            self.error_msg = format!(
                                                "Loading theme failed with the following error {}",
                                                err
                                            );
                                            self.show_error_window = true;
                                        }
                                    }
                                }
                                Err(err) => {
                                    self.error_msg = format!(
                                        "Loading theme failed with the following error {}",
                                        err
                                    );
                                    self.error_msg = format!("Failed to open file due to: {}", err);
                                    self.show_error_window = true;
                                }
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                egui::menu::menu(ui, "Options", |ui| {
                    if ui.button("Set current theme as app theme").clicked() {
                        let theme = self.state.export_theme();
                        // The font_data is only used for serialization
                        let (style, font_definitions) = theme.extract();
                        ctx.set_style(style);
                        ctx.set_fonts(font_definitions);
                    }
                    if ui.button("Clear settings").clicked() {
                        self.state = StylerState::default();
                        ctx.set_fonts(egui::FontDefinitions::default());
                    }
                    if ui.button("Reset App Theme Theme").clicked() {
                        ctx.set_style(egui::Style::default());
                        ctx.set_fonts(egui::FontDefinitions::default())
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| self.state.ui(ui));
        open_error_window(
            ctx,
            "Error",
            self.error_msg.as_str(),
            &mut self.show_error_window,
        );
    }
}

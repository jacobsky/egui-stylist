use eframe::egui;
use eframe::egui::Style;
use egui_stylist::{StylistFileDialog, StylistState};
use std::fs::File;
use std::io::Read;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
use std::path::PathBuf;

// fn open_error_window(ctx: &egui::Context, title: &str, text: &str, open: &mut bool) {
//     let window = egui::Window::new(title.to_owned())
//         .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
//         .auto_sized()
//         .collapsible(false)
//         .vscroll(false)
//         .open(open);
//     window.show(ctx, |ui| {
//         ui.colored_label(egui::Color32::RED, text);
//     });
// }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct StylistApp {
    // This is the main
    state: StylistState,
    show_error_window: bool,
    error_msg: String,
}

impl StylistApp {
    #[cfg(feature = "persistence")]
    fn get_app_state(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let raw_data = storage.get_string(eframe::APP_KEY).unwrap_or_default();
            let state = ron::from_str(raw_data.as_str());
            if let Ok(state) = state {
                state
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
    #[cfg(not(feature = "persistence"))]
    fn get_app_state(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut app = Self::get_app_state(cc);
        cc.egui_ctx.set_style(Style::default());
        // TODO: Allow persistence
        app.state
            .set_file_dialog_function(Box::new(open_file_dialog));

        app
    }
}

impl Default for StylistApp {
    fn default() -> Self {
        Self {
            state: StylistState::default(),
            show_error_window: false,
            error_msg: "".to_owned(),
        }
    }
}
/// Native filedialogs for Windows, Unix and MacOs via rfd crate.
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn open_file_dialog(kind: StylistFileDialog, filter: Option<(&str, &[&str])>) -> Option<PathBuf> {
    // Option a popup to save the file to a given directory
    let path = std::env::current_dir().expect("there should be a current directory");
    match kind {
        StylistFileDialog::Open => {
            let mut builder = rfd::FileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            builder.set_directory(&path).pick_file()
        }
        StylistFileDialog::Save => {
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
    file_dialog: StylistFileDialog,
    filter: Option<(&str, &[&str])>,
) -> Option<PathBuf> {
    use futures::executor::block_on;
    match file_dialog {
        StylistFileDialog::Open => {
            let mut builder = rfd::AsyncFileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            let result = block_on(builder.pick_file());
            result
        }
        StylistFileDialog::Save => {
            // Save file dialogs are not supported with rfd at this time.
            None
        }
    }
}

impl eframe::App for StylistApp {
    // fn name(&self) -> &str {
    //     "egui styler"
    // }

    /// Called by the framework to load old app state (if any).
    // #[cfg(feature = "persistence")]
    // fn setup(
    //     &mut self,
    //     _ctx: &egui::Context,
    //     _frame: &mut eframe::Frame<'_>,
    //     storage: Option<&dyn eframe::Storage>,
    // ) {
    //     #[cfg(feature = "persistence")]
    //     if let Some(storage) = storage {
    //         *self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
    //     }
    //     self.state
    //         .set_file_dialog_function(Box::new(open_file_dialog));
    // }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(app_state) = ron::to_string(self) {
            storage.set_string(eframe::APP_KEY, app_state);
        } else {
            // TODO: Do some logging here.
        }
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Save").clicked() {
                        // Option a popup to save the file to a given directory
                        if let Some(path) = self.state.file_dialog(
                            StylistFileDialog::Save,
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
                            StylistFileDialog::Open,
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
                egui::menu::menu_button(ui, "Options", |ui| {
                    if ui.button("Set current theme as app theme").clicked() {
                        let theme = self.state.export_theme();
                        // The font_data is only used for serialization
                        let (style, font_definitions) = theme.extract();
                        ctx.set_style(style);
                        ctx.set_fonts(font_definitions);
                    }
                    if ui.button("Clear settings").clicked() {
                        self.state = StylistState::default();
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
    }
}

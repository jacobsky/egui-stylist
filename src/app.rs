use crate::StylerState;
use eframe::{egui, epi};
use std::fs::File;
use std::io::Read;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct StylerApp {
    // This is the main
    state: StylerState,
}

impl Default for StylerApp {
    fn default() -> Self {
        Self {
            state: StylerState::default(),
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
                    use futures::executor::block_on;
                    // TODO: Make a generic FileDialog Modal
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Save").clicked() {
                        // Option a popup to save the file to a given directory
                        let path =
                            std::env::current_dir().expect("there should be a current directory");

                        if let Some(path) = block_on(
                            rfd::AsyncFileDialog::new()
                                .add_filter("egui theme", &["ron", "eguitheme"])
                                .set_directory(&path)
                                .save_file(),
                        ) {
                            let theme = self.state.export_theme();
                            if let Ok(value) = ron::to_string(&theme) {
                                if let Ok(mut f) = File::create(path.path()) {
                                    f.write_all(value.as_bytes()).expect("this should work");
                                }
                                // TODO: Post an error message if this fails.
                            }
                            // TODO: Post some error modal
                        }
                    }
                    if ui.button("Load").clicked() {
                        // Option a popup to load the file
                        let path =
                            std::env::current_dir().expect("there should be a current directory");
                        if let Some(path) = block_on(
                            rfd::AsyncFileDialog::new()
                                .add_filter("egui theme", &["ron", "eguitheme"])
                                .set_directory(&path)
                                .pick_file(),
                        ) {
                            if let Ok(mut f) = File::open(path.file_name()) {
                                let mut buf = String::new();
                                f.read_to_string(&mut buf).expect("this should work");
                                if let Ok(theme) = ron::from_str(&buf) {
                                    self.state.import_theme(theme);
                                }
                                // TODO: Post an error message if this fails.
                            }
                            // TODO: Post some error modal
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
                        let (style, font_definitions, ..) = theme.extract();
                        ctx.set_style(style);
                        ctx.set_fonts(font_definitions);
                    }
                    if ui.button("Clear settings").clicked() {
                        self.state = StylerState::default();
                        ctx.set_fonts(egui::FontDefinitions::default());
                    }
                    if ui.button("Reset App Theme Theme").clicked() {
                        ctx.set_style(egui::Style::default());
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| self.state.ui(ui));
    }
}

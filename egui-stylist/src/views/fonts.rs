use super::StylistFileDialog;
use std::io::Read;
use std::path::Path;

use egui::FontData;

// TODO: Reference egui-theme which is where this info should be stored.
const DEFAULT_FONTS: [&str; 4] = [
    "Hack",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];
use egui::{
    Button, Checkbox, CollapsingHeader, ComboBox, DragValue, FontDefinitions, FontFamily, Grid,
    Label, Style, TextEdit, TextStyle, Ui, Widget,
};

pub struct FontViewState {
    to_add_name: String,
    to_add_path: String,
    to_add_family: String,
    to_delete: Vec<String>,
    pub(crate) pixels_per_point: f32,
}

impl Default for FontViewState {
    fn default() -> Self {
        Self {
            to_add_name: "".to_owned(),
            to_add_path: "".to_owned(),
            to_add_family: "".to_owned(),
            to_delete: Vec::new(),
            pixels_per_point: 1f32,
        }
    }
}

fn add_font(
    state: &mut FontViewState,
    font_definitions: &mut FontDefinitions,
    file_dialog_callback: Option<&super::StylistFileDialogFunction>,
    ui: &mut Ui,
) {
    // let mut fonts_updated = false;
    ui.vertical(|ui| {
        Grid::new("_properties").num_columns(2).show(ui, |ui| {
            Label::new("Name:").ui(ui);
            TextEdit::singleline(&mut state.to_add_name)
                .hint_text("The name to register font as")
                .ui(ui);
            ui.end_row();
            if let Some(file_dialog) = file_dialog_callback {
                Label::new("Font File:").ui(ui);
                let btn_text = if !state.to_add_path.is_empty() {
                    state.to_add_path.clone()
                } else {
                    "Open file dialog".to_owned()
                };
                if Button::new(btn_text).ui(ui).clicked() {
                    if let Some(path) = file_dialog(
                        StylistFileDialog::Open,
                        Some(("font file", &["ttf", "otf"])),
                    ) {
                        state.to_add_name = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_owned();
                        state.to_add_path = path.to_str().unwrap_or_default().to_owned();
                    }
                }
            } else {
                // This is the fallback in case the file_dialog_callback is not set.
                Label::new("Path to Font File:").ui(ui);
                TextEdit::singleline(&mut state.to_add_path)
                    .hint_text("filepath to the font data (.ttf or .otf file)")
                    .ui(ui);
                ui.end_row();
            }
        });
        let path = Path::new(&state.to_add_path);
        let enabled = {
            !(state.to_add_name.is_empty()
                || font_definitions
                    .font_data
                    .contains_key(state.to_add_name.as_str()))
                && path.is_file()
        };
        if ui
            .add_enabled(enabled, Button::new("Add font to theme"))
            .clicked()
        {
            use std::fs::File;
            use std::io::BufReader;

            if let Ok(file) = File::open(path) {
                let mut reader = BufReader::new(file);
                let mut contents = Vec::new();
                if reader.read_to_end(&mut contents).is_ok() {
                    font_definitions
                        .font_data
                        .insert(state.to_add_name.clone(), FontData::from_owned(contents));
                }
                // TODO: Give some sort of error popup or modal if this fails
            }
            // TODO: Add an error message or modal if this fails.
        }
    });
    // fonts_updated
}

fn font_priority(
    id: &str,
    family: &FontFamily,
    font_definitions: &mut FontDefinitions,
    ui: &mut Ui,
) {
    enum Direction {
        Up,
        Down,
    }
    // TODO: Implement this
    let mut fonts = font_definitions
        .families
        .get(family)
        .expect("this should be valid")
        .clone();
    let mut swap = None;

    Grid::new(id).num_columns(3).show(ui, |ui| {
        for (i, name) in fonts.iter().enumerate() {
            ui.label(name);
            if i > 0 {
                if ui.button("increase").clicked() {
                    swap = Some((Direction::Up, i));
                }
            } else {
                ui.label("");
            }
            if i < fonts.len() - 1 && ui.button("decrease").clicked() {
                swap = Some((Direction::Down, i));
            }
            ui.end_row();
        }
    });
    if let Some((dir, index)) = swap {
        let new_index = match dir {
            Direction::Up => index - 1,
            Direction::Down => index + 1,
        };
        let font = fonts.remove(index);
        fonts.insert(new_index, font);
        font_definitions.families.insert(family.to_owned(), fonts);
    }
}

/// Displays the current font definition from the core app widget and displays the ui to detect any addition changes.
pub fn fonts_view(
    state: &mut FontViewState,
    file_dialog_callback: Option<&super::StylistFileDialogFunction>,
    font_definitions: &mut FontDefinitions,
    style: &mut Style,
    ui: &mut Ui,
) {
    // Flag to indicate if we need to update the Context font data.
    // let mut fonts_updated = false;
    // font_definitions.
    ui.heading("Fonts Menu");
    // This is a workaround for the default fonts which will crash the interface if they are deleted.
    CollapsingHeader::new("General Settings")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("_general_settings").show(ui, |ui| {
                ui.label("Pixels Per Point");
                DragValue::new(&mut state.pixels_per_point)
                    .clamp_range(0.001f32..=4.0f32)
                    .min_decimals(2)
                    .max_decimals(3)
                    .ui(ui);
                ui.end_row();

                ui.label("Body Text Style");
                ComboBox::from_id_source("_body_text_style")
                    .selected_text(format!("{:?}", &style.override_text_style))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut style.override_text_style, None, "None");
                        ui.selectable_value(
                            &mut style.override_text_style,
                            Some(TextStyle::Heading),
                            "Heading",
                        );
                        ui.selectable_value(
                            &mut style.override_text_style,
                            Some(TextStyle::Body),
                            "Body",
                        );
                        ui.selectable_value(
                            &mut style.override_text_style,
                            Some(TextStyle::Small),
                            "Small",
                        );
                        ui.selectable_value(
                            &mut style.override_text_style,
                            Some(TextStyle::Button),
                            "Button",
                        );
                        ui.selectable_value(
                            &mut style.override_text_style,
                            Some(TextStyle::Monospace),
                            "Monospace",
                        );
                    });
                ui.end_row();

                ui.label("Override Text Style");
                let mut override_text_style = style.override_text_style.is_some();
                let response = Checkbox::new(&mut override_text_style, "").ui(ui);
                if response.clicked() {
                    if override_text_style && style.override_text_style.is_none() {
                        style.override_text_style = Some(TextStyle::Body);
                    } else if style.override_text_style.is_some() {
                        style.override_text_style = None
                    }
                }
                ui.end_row();
                if let Some(text_style) = &mut style.override_text_style {
                    ui.label("Override text style");
                    ComboBox::from_id_source("_override_text_style")
                        .selected_text(format!("{:?}", text_style))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(text_style, TextStyle::Heading, "Heading");
                            ui.selectable_value(text_style, TextStyle::Body, "Body");
                            ui.selectable_value(text_style, TextStyle::Small, "Small");
                            ui.selectable_value(text_style, TextStyle::Button, "Button");
                            ui.selectable_value(text_style, TextStyle::Monospace, "Monospace");
                        });
                }
                ui.end_row();
                // pub override_text_style: Option<TextStyle>,
            });
        });
    CollapsingHeader::new("Installed Fonts")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("_fonts").num_columns(3).show(ui, |ui| {
                for (name, _data) in font_definitions.font_data.iter() {
                    ui.label(name);
                    // Cannot delete the default egui fonts without breaking things.
                    if !DEFAULT_FONTS.contains(&name.as_str()) && ui.button("Delete").clicked() {
                        state.to_delete.push(name.to_owned());
                    }
                    ui.end_row();
                }

                for key in state.to_delete.iter() {
                    font_definitions.font_data.remove(key);
                }
                state.to_delete.clear();
            });
        });
    CollapsingHeader::new("Add font")
        .default_open(true)
        .show(ui, |ui| {
            add_font(state, font_definitions, file_dialog_callback, ui)
        });
    CollapsingHeader::new("Add/Remove Font Families")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("_add_remove_families")
                .num_columns(2usize)
                .show(ui, |ui| {
                    let mut to_delete = Vec::new();
                    for family in font_definitions.families.keys() {
                        ui.label(family.to_string());
                        match family {
                            egui::FontFamily::Monospace | egui::FontFamily::Proportional => {}
                            _ => {
                                let response = ui.button("delete");
                                if response.clicked() {
                                    to_delete.push(family.to_owned());
                                }
                            }
                        }
                        ui.end_row();
                    }
                    ui.text_edit_singleline(&mut state.to_add_family);
                    let response = ui.button("add");
                    if response.clicked() {
                        font_definitions.families.insert(
                            FontFamily::Name(state.to_add_family.to_owned().into()),
                            Vec::new(),
                        );
                        state.to_add_family = "".to_owned();
                    }
                    ui.end_row();

                    for key in to_delete {
                        font_definitions.families.remove(&key);
                    }
                });
        });

    CollapsingHeader::new("Edit Font Families")
        .default_open(true)
        .show(ui, |ui| {
            let num_cols = font_definitions.families.keys().count();
            Grid::new("_families")
                .num_columns(num_cols + 1usize)
                .show(ui, |ui| {
                    ui.label("Font");
                    let map = font_definitions
                        .families
                        .iter()
                        .map(|(k, _)| k.to_string())
                        .collect::<Vec<String>>();
                    for entry in map {
                        ui.label(entry);
                    }
                    ui.end_row();
                    let families = font_definitions.families.clone();
                    for (name, _) in font_definitions.font_data.iter() {
                        ui.label(name);
                        for (family, names) in families.iter() {
                            let mut is_set = names.contains(name);
                            let response = ui.add(Checkbox::new(&mut is_set, ""));
                            if response.clicked() {
                                if let Some(strings) = font_definitions.families.get_mut(family) {
                                    if is_set {
                                        strings.push(name.to_owned());
                                    } else if let Some(idx) = strings
                                        .iter()
                                        .enumerate()
                                        .find(|(_, string)| *string == name)
                                        .map(|(idx, _)| idx)
                                    {
                                        strings.remove(idx);
                                    }
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    CollapsingHeader::new("Font Priority")
        .default_open(true)
        .show(ui, |ui| {
            let families = font_definitions
                .families
                .iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<FontFamily>>();
            for family in families.iter() {
                CollapsingHeader::new(format!("{family} Priority").as_str())
                    .default_open(true)
                    .show(ui, |ui| {
                        let id = format!("_{family}_priority");
                        font_priority(id.as_str(), family, font_definitions, ui);
                        ui.end_row();
                    });
            }
        });
}

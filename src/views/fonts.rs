use std::io::Read;
use std::path::Path;

use eframe::egui::{Button, Checkbox, CollapsingHeader, DragValue, FontDefinitions, FontFamily, Grid, Label, TextEdit, TextStyle, Ui, Widget};

const DEFAULT_FONTS: [&str; 4]  = [
    "ProggyClean",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font"
];
#[derive(Default)]
pub struct FontViewState {
    to_add_name: String,
    to_add_path: String,
    to_delete: Vec<String>,
}

impl FontViewState {
    pub fn new() -> Self {
        Self {
            to_add_name: "".to_owned(),
            to_add_path: "".to_owned(),
            to_delete: Vec::new(),
        }
    }
}

fn add_font(state: &mut FontViewState, font_definitions: &mut FontDefinitions, ui: &mut Ui)  {
    // let mut fonts_updated = false;
    ui.vertical(|ui|{
        Grid::new("_properties").num_columns(2).show(ui, |ui| {
            Label::new("Name:").ui(ui);
            TextEdit::singleline(&mut state.to_add_name)
                .hint_text("The name to register font as")
                .ui(ui);
            ui.end_row();
            Label::new("Path to Font File:").ui(ui);
            TextEdit::singleline(&mut state.to_add_path)
                .hint_text("filepath to the font data (.ttf or .otf file)")
                .ui(ui);
            ui.end_row();
        });
        let path = Path::new(&state.to_add_path);
        let enabled = {
            !(state.to_add_name.is_empty() || font_definitions.font_data.contains_key(state.to_add_name.as_str())) && path.is_file()
        };
        if Button::new("Add font to theme").enabled(enabled).ui(ui).clicked() {
            use std::fs::File;
            use std::io::BufReader;
            
            if let Ok(file) = File::open(path) {
                let mut reader = BufReader::new(file);
                let mut contents = Vec::new();
                if reader.read_to_end(&mut contents).is_ok() {
                    font_definitions.font_data.insert(state.to_add_name.clone(), contents.into());
                    // fonts_updated = true;
                }
                // TODO: Give some sort of error popup or modal if this fails
            }
            // TODO: Add an error message or modal if this fails.
        }
    });
    // fonts_updated
}


/// Displays the current font definition from the core app widget and displays the ui to detect any addition changes.
pub fn fonts_view(state: &mut FontViewState, font_definitions: &mut FontDefinitions, ui: &mut Ui) {
    // Flag to indicate if we need to update the Context font data.
    // let mut fonts_updated = false;
    // font_definitions.
    ui.heading("Fonts Menu: WIP");
    // This is a workaround for the default fonts which will crash the interface if they are deleted.

    CollapsingHeader::new("Installed Fonts").default_open(true).show(ui, |ui|{
        Grid::new("_fonts").num_columns(3).show(ui, |ui| {
            for (name, _data) in font_definitions.font_data.iter() {
                ui.label(name);
                // Cannot delete the default egui fonts without breaking things.
                if !DEFAULT_FONTS.contains(&name.as_str()) {
                    if ui.button("Delete").clicked()  {
                        state.to_delete.push(name.to_owned());
                    }
                }
                ui.end_row();
            }
            
            for key in state.to_delete.iter() {
                font_definitions.font_data.remove(key);
                // fonts_updated = true;
            }
            state.to_delete.clear();
        });
    });
    CollapsingHeader::new("Add font").default_open(true).show(ui, |ui|{
        
        add_font(state, font_definitions, ui)
        
    });
    //TODO: Add a grid that lists all the fonts and their relevant font families
    CollapsingHeader::new("Font Families").default_open(true).show(ui, |ui|{
        Grid::new("_families").num_columns(3).show(ui, |ui|{
            ui.label("Font");
            ui.label("Proportional");
            ui.label("Monospace");
            ui.end_row();
            for (name, _) in font_definitions.font_data.iter() {
                ui.label(name);
                let mut is_monospace = if let Some(fonts) = font_definitions.fonts_for_family.get(&eframe::egui::FontFamily::Monospace) {
                    fonts.contains(name)
                } else {
                    false
                };
                let mut is_proportional = if let Some(fonts) = font_definitions.fonts_for_family.get(&eframe::egui::FontFamily::Proportional) {
                    fonts.contains(name)
                } else {
                    false
                };
                
                let response = ui.add(Checkbox::new(&mut is_proportional, "Proportional"));
                if response.clicked() {
                    let fonts = font_definitions.fonts_for_family.get_mut(&eframe::egui::FontFamily::Proportional).expect("this should exist");
                    if is_proportional {
                        fonts.push(name.to_owned());
                    } else {
                        if let Some(idx) = fonts.iter().position(|a| a.eq(name)) {
                            fonts.remove(idx);
                        }
                    }
                }
                let response = ui.add(Checkbox::new(&mut is_monospace, "Monospace"));
                if response.clicked() {                    
                    let fonts = font_definitions.fonts_for_family.get_mut(&eframe::egui::FontFamily::Monospace).expect("this should exist");
                    if is_monospace {
                        fonts.push(name.to_owned());
                    } else {
                        if let Some(idx) = fonts.iter().position(|a| a.eq(name)) {
                            fonts.remove(idx);
                        }
                    }
                }
                
                ui.end_row();
            }
        });
    });
    //TODO: Add a grid for each font which has a text edit for each font type.s
    CollapsingHeader::new("Font Style Sizes").default_open(true).show(ui, |ui|{
        Grid::new("_families").num_columns(3).show(ui, |ui|{
            let mut mono_small = font_definitions.family_and_size.get(&TextStyle::Small).map_or(12.0, |(_, s)| *s);
            let mut mono_body = font_definitions.family_and_size.get(&TextStyle::Body).map_or(12.0, |(_, s)| *s);
            let mut mono_button = font_definitions.family_and_size.get(&TextStyle::Button).map_or(12.0, |(_, s)| *s);
            let mut mono_heading = font_definitions.family_and_size.get(&TextStyle::Heading).map_or(12.0, |(_, s)| *s);
            let mut mono_monospace = font_definitions.family_and_size.get(&TextStyle::Monospace).map_or(12.0, |(_, s)| *s);
            ui.label("Family");
            ui.label("Small");
            ui.label("Body");
            ui.label("Button");
            ui.label("Heading");
            ui.label("Monospace");
            ui.end_row();
            ui.label("Monospace");
            if DragValue::new(&mut mono_small).clamp_range(1..=100).ui(ui).changed() {
                font_definitions.family_and_size.insert(TextStyle::Small, (FontFamily::Monospace, mono_small));
                // fonts_updated = true;
            }
            if DragValue::new(&mut mono_body).clamp_range(1..=100).ui(ui).changed() {
                font_definitions.family_and_size.insert(TextStyle::Body, (FontFamily::Monospace, mono_body));
                // fonts_updated = true;
            }
            if DragValue::new(&mut mono_button).clamp_range(1..=100).ui(ui).changed() {
                font_definitions.family_and_size.insert(TextStyle::Button, (FontFamily::Monospace, mono_button));
                // fonts_updated = true;
            }
            if DragValue::new(&mut mono_heading).clamp_range(1..=100).ui(ui).changed() {
                font_definitions.family_and_size.insert(TextStyle::Heading, (FontFamily::Monospace, mono_heading));
                // fonts_updated = true;
            }
            if DragValue::new(&mut mono_monospace).clamp_range(1..=100).ui(ui).changed() {
                font_definitions.family_and_size.insert(TextStyle::Monospace, (FontFamily::Monospace, mono_monospace));
                // fonts_updated = true;
            }
            

            ui.end_row();
            ui.label("Proportional");
            let mut prop_small = font_definitions.family_and_size.get(&TextStyle::Small).map_or(12.0, |(_, s)| *s);
            let mut prop_body = font_definitions.family_and_size.get(&TextStyle::Body).map_or(12.0, |(_, s)| *s);
            let mut prop_button = font_definitions.family_and_size.get(&TextStyle::Button).map_or(12.0, |(_, s)| *s);
            let mut prop_heading = font_definitions.family_and_size.get(&TextStyle::Heading).map_or(12.0, |(_, s)| *s);
            let mut prop_monospace = font_definitions.family_and_size.get(&TextStyle::Monospace).map_or(12.0, |(_, s)| *s);
            if DragValue::new(&mut prop_small).clamp_range(1..=100).ui(ui).changed() {
                // fonts_updated = true;
                font_definitions.family_and_size.insert(TextStyle::Small, (FontFamily::Proportional, prop_small));
            }
            if DragValue::new(&mut prop_body).clamp_range(1..=100).ui(ui).changed() {
                // fonts_updated = true;
                font_definitions.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, prop_body));
            }
            if DragValue::new(&mut prop_button).clamp_range(1..=100).ui(ui).changed() {
                // fonts_updated = true;
                font_definitions.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, prop_button));
            }
            if DragValue::new(&mut prop_heading).clamp_range(1..=100).ui(ui).changed() {
                // fonts_updated = true;
                font_definitions.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, prop_heading));
            }
            if DragValue::new(&mut prop_monospace).clamp_range(1..=100).ui(ui).changed() {
                // fonts_updated = true;
                font_definitions.family_and_size.insert(TextStyle::Monospace, (FontFamily::Proportional, prop_monospace));
            }
            ui.end_row();
        });
    });
}

pub fn update_fonts(ui: &mut Ui, font_definitions: &FontDefinitions) {
    let ctx = ui.ctx();
    ctx.set_fonts(font_definitions.clone());
}
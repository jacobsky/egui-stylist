use std::{io::Read, path::Path};

use eframe::egui::{Button, CollapsingHeader, FontDefinitions, Grid, Label, TextEdit, Ui, Widget};

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

fn add_font(state: &mut FontViewState, font_definitions: &mut FontDefinitions, ui: &mut Ui) {
    Label::new("New font").ui(ui);
    TextEdit::singleline(&mut state.to_add_name)
        .hint_text("The name to register font as")
        .ui(ui);
    TextEdit::singleline(&mut state.to_add_path)
        .hint_text("filepath to the font data (.ttf or .otf file)")
        .ui(ui);
    let path = Path::new(&state.to_add_path);
    let enabled = {
        !(state.to_add_name.is_empty() || font_definitions.font_data.contains_key(state.to_add_name.as_str())) && path.is_file()
    };
    if Button::new("Add").enabled(enabled).ui(ui).clicked() {
        use std::fs::File;
        use std::io::BufReader;
        if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);
            let mut contents = Vec::new();
            if reader.read_to_end(&mut contents).is_ok() {
                font_definitions.font_data.insert(state.to_add_name.clone(), contents.into());
            }
        }
        // TODO: Do the adding of stuff.
        
        // let data = ;
        // font_definitions.font_data.insert(state.to_add_name.clone(), data);
    }
}


/// Displays the current font definition from the core app widget and displays the ui to detect any addition changes.
pub fn fonts_view(state: &mut FontViewState, font_definitions: &mut FontDefinitions, ui: &mut Ui) {
    // font_definitions.
    ui.heading("Fonts Menu: WIP");
    CollapsingHeader::new("Installed Fonts").default_open(true).show(ui, |ui|{
        Grid::new("_fonts").num_columns(3).show(ui, |ui| {
                for (name, _data) in font_definitions.font_data.iter() {
                    ui.label(name);
                    if ui.button("Delete").clicked() {
                        state.to_delete.push(name.to_owned());
                    }
                    ui.end_row();
                }
                
                for key in state.to_delete.iter() {
                    font_definitions.font_data.remove(key.as_str());
                }
                state.to_delete.clear();
            });
        });
        ui.horizontal(|ui|{
            
            add_font(state, font_definitions, ui);
        });
    // Print a table with all the registered font data
    // Include a button that allows for adding a new font to be loaded into the font definitions.
    // Add a grid that lists all the fonts and their relevant font families
    // Add a grid for each font which has a text edit for each font type.
}
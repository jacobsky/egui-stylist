use std::ops::RangeInclusive;

use egui::{
    Button, CollapsingHeader, ComboBox, DragValue, FontFamily, FontId, Grid, Style, TextEdit,
    TextStyle, Ui, Widget,
};

pub struct TextStyleViewState {
    new_style_name: String,
    new_style_size: f32,
    new_style_family: FontFamily,
}

impl TextStyleViewState {
    fn new() -> Self {
        Self {
            new_style_name: "".to_owned(),
            new_style_size: 8.0f32,
            new_style_family: FontFamily::Monospace,
        }
    }
}

impl Default for TextStyleViewState {
    fn default() -> Self {
        Self::new()
    }
}

const TEXT_STYLE_RANGE: RangeInclusive<f32> = 0f32..=128f32;

pub fn text_styles_view(
    state: &mut TextStyleViewState,
    style: &mut Style,
    families: Vec<FontFamily>,
    ui: &mut Ui,
) {
    // TODO: Make a more ergonic spacing UI
    ui.heading("Text Style Settings");
    Grid::new("Text Styles").num_columns(3).show(ui, |ui| {
        for ts in style.text_styles() {
            ui.label(ts.to_string());
            if let Some(font_id) = style.text_styles.get_mut(&ts) {
                ComboBox::new(format!("_{ts}_family"), "")
                    .selected_text(font_id.family.to_string())
                    .show_ui(ui, |ui| {
                        for family in families.iter() {
                            ui.selectable_value(
                                &mut font_id.family,
                                family.to_owned(),
                                family.to_string(),
                            );
                        }
                    });
                ui.add(DragValue::new(&mut font_id.size).clamp_range(TEXT_STYLE_RANGE));
            } else {
                ui.label("No FontID associated");
            }
            ui.end_row();
        }
    });
    CollapsingHeader::new("Add Custom TextStyle")
        .default_open(true)
        .show(ui, |ui| {
            TextEdit::singleline(&mut state.new_style_name)
                .desired_width(f32::INFINITY)
                .ui(ui);
            Grid::new("add_text_style_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Font Size");
                    ui.add(DragValue::new(&mut state.new_style_size).clamp_range(TEXT_STYLE_RANGE));
                    ui.end_row();
                    // probably put the selectable values and stuff here.
                    ui.label("Font Family");

                    ComboBox::new("add_text_style_select_family", "")
                        .selected_text(state.new_style_family.to_string())
                        .show_ui(ui, |ui| {
                            for family in families.iter() {
                                let selected_value = family.to_owned();
                                let text = family.to_string();
                                ui.selectable_value(
                                    &mut state.new_style_family,
                                    selected_value,
                                    text,
                                );
                            }
                        });
                    ui.end_row();
                    let btn = Button::new("Add TextStyle");
                    let enabled = !state.new_style_name.is_empty();
                    let response = ui.add_enabled(enabled, btn);
                    if response.clicked() {
                        style.text_styles.insert(
                            TextStyle::Name(state.new_style_name.to_owned().into()),
                            FontId::new(state.new_style_size, state.new_style_family.to_owned()),
                        );
                    }
                });
        });
}

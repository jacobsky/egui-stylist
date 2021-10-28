use crate::{EguiTheme, DEFAULT_FONTS};
use egui::FontDefinitions;

/// This can be used to migrate egui themes from egui 0.14.x to 0.15.x
fn migrate_fonts_from_14_to_15(font: &mut FontDefinitions) -> FontDefinitions {
    let mut new_defaults = FontDefinitions::default();
    for (key, value) in font.font_data.iter() {
        if !DEFAULT_FONTS.contains(&key.as_str()) && !new_defaults.font_data.contains_key(key) {
            new_defaults.font_data.insert(key.clone(), value.clone());
        }
    }
    for (_, fonts) in font.fonts_for_family.iter_mut() {
        for i in fonts.len() - 1..0 {
            if fonts[i].eq("ProggyClean") {
                fonts.remove(i);
            }
        }
    }
    new_defaults
}

pub fn migration_from_14_to_15(mut theme: EguiTheme) -> EguiTheme {
    theme.font_definitions = migrate_fonts_from_14_to_15(&mut theme.font_definitions);
    theme
}

use std::collections::{HashMap, BTreeMap};
use egui::FontDefinitions;

const FONT_DATA_KEY: &str = "font_data";
const FONTS_FOR_FAMILY_KEY: &str = "fonts_for_family";
const FAMILY_AND_SIZE_KEY: &str = "family_and_size";

fn remove_default_fonts<'a>(mut font_data: BTreeMap<String, std::borrow::Cow<'a, [u8]>>) -> BTreeMap<String, std::borrow::Cow<'a, [u8]>> {
    for font_name in crate::DEFAULT_FONTS {
        font_data.remove(&font_name.to_owned());
    }
    font_data
}

pub fn from_fonts(FontDefinitions { font_data, fonts_for_family, family_and_size }: FontDefinitions) -> HashMap<String, serde_json::Value> {
    let font_data = remove_default_fonts(font_data);
    let default_defs = FontDefinitions::default();
    let mut hash_map = HashMap::new();
    hash_map.insert(
        FONT_DATA_KEY.to_owned(),
        serde_json::to_value(font_data)
        .unwrap_or_else(|_err| {
            serde_json::to_value(default_defs.fonts_for_family.to_owned())
            .expect("the fallback should work")
        })
    );
    hash_map.insert(
        FONTS_FOR_FAMILY_KEY.to_owned(),
        serde_json::to_value(fonts_for_family)
            .unwrap_or_else(|_err| {
                serde_json::to_value(default_defs.fonts_for_family.to_owned())
                .expect("the fallback should work")
            })
    );
    hash_map.insert(
        FAMILY_AND_SIZE_KEY.to_owned(),
        serde_json::to_value(family_and_size)
            .unwrap_or_else(|_err| {
                serde_json::to_value(default_defs.family_and_size.to_owned())
                .expect("the fallback should work")
            })
    );

    hash_map
}

pub fn to_fonts(hash_map: HashMap<String, serde_json::Value>) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    
    hash_map.get(&FONT_DATA_KEY.to_owned()).map(|value| {
        fonts.font_data = serde_json::from_value(value.to_owned()).unwrap_or_default();
    });
    hash_map.get(&FONTS_FOR_FAMILY_KEY.to_owned()).map(|value| {
        fonts.fonts_for_family = serde_json::from_value(value.to_owned()).unwrap_or_default();
    });
    hash_map.get(&FAMILY_AND_SIZE_KEY.to_owned()).map(|value| {
        fonts.family_and_size = serde_json::from_value(value.to_owned()).unwrap_or_default();
    });
    // Text fonts does not have a default, so it has to be implemented manually instead of being macroed.
        
    fonts
}


use egui::{FontData, FontDefinitions};
use std::collections::{BTreeMap, HashMap};

const FONT_DATA_KEY: &str = "font_data";
const FAMILIES_KEY: &str = "families";

/// Removes the default font data when serializing the fonts.
/// This is done to trim down the size of the data saved into the theme.
fn remove_default_fonts(mut font_data: BTreeMap<String, FontData>) -> BTreeMap<String, FontData> {
    for font_name in crate::DEFAULT_FONTS {
        font_data.remove(&font_name.to_owned());
    }
    font_data
}
/// Helper function for serializing the `egui::FontDefinitions`
pub fn from_fonts(
    FontDefinitions {
        font_data,
        families,
    }: FontDefinitions,
) -> HashMap<String, serde_json::Value> {
    let font_data = remove_default_fonts(font_data);
    let mut hash_map = HashMap::new();

    hash_map.insert(
        FONT_DATA_KEY.to_owned(),
        serde_json::to_value(font_data).expect("serialization error occurred"),
    );

    // Workaround due to FontFamily not properly serializing to "String" when attempting to serialize the BTreeMap<FontFamily, Vec<String>>
    let families = {
        families
            .iter()
            .map(|(family, list)| {
                (
                    serde_json::to_string(family).expect("serialization failed"),
                    list.clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    hash_map.insert(
        FAMILIES_KEY.to_owned(),
        serde_json::to_value(families).expect("serialization error occurred"),
    );

    hash_map
}

/// Helper function for deserializing the `egui::FontDefinitions`
pub fn to_fonts(hash_map: HashMap<String, serde_json::Value>) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    if let Some(value) = hash_map.get(&FONT_DATA_KEY.to_owned()) {
        if let Ok(font_data) =
            serde_json::from_value::<BTreeMap<String, FontData>>(value.to_owned())
        {
            for (k, v) in font_data.iter() {
                let _ = fonts.font_data.insert(k.to_owned(), v.to_owned());
            }
        }
    }

    if let Some(value) = hash_map.get(&FAMILIES_KEY.to_owned()) {
        // Workaround due to FontFamily not properly serializing to "String" when attempting to serialize the BTreeMap<FontFamily, Vec<String>>
        if let Ok(families) = serde_json::from_value::<Vec<(String, Vec<String>)>>(value.to_owned())
        {
            // println!("{:?}", families);
            for (family, list) in families {
                let family = serde_json::from_str(family.as_str()).expect("serialization failed");
                fonts.families.insert(family, list.clone());
            }
        };
    }

    fonts
}

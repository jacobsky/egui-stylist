use std::collections::{HashMap, BTreeMap};
use egui::FontDefinitions;
use egui::FontData;

const FONT_DATA_KEY: &str = "font_data";
const FAMILIES_KEY: &str = "families";


/// Removes the default font data when serializing the fonts.
/// This is done to trim down the size of the data saved into the theme.
fn remove_default_fonts<'a>(mut font_data: BTreeMap<String, FontData>) -> BTreeMap<String, FontData> {
    for font_name in crate::DEFAULT_FONTS {
        font_data.remove(&font_name.to_owned());
    }
    font_data
}
/// TODO: Comment this function
pub fn from_fonts(FontDefinitions { font_data, families }: FontDefinitions) -> HashMap<String, serde_json::Value> {
    let font_data = remove_default_fonts(font_data);
    let default_defs = FontDefinitions::default();
    let mut hash_map = HashMap::new();
    hash_map.insert(
        FONT_DATA_KEY.to_owned(),
        serde_json::to_value(font_data)
        .unwrap_or_else(|_err| {
            serde_json::to_value(default_defs.font_data.to_owned())
            .expect("the fallback should work")
        })
    );
    
    /// TODO: Comment this function
    hash_map.insert(
        FAMILIES_KEY.to_owned(),
        serde_json::to_value(families)
            .unwrap_or_else(|_err| {
                serde_json::to_value(default_defs.families.to_owned())
                .expect("the fallback should work")
            })
    );

    hash_map
}

pub fn to_fonts(hash_map: HashMap<String, serde_json::Value>) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    
    hash_map.get(&FONT_DATA_KEY.to_owned()).map(|value| {
        let font_data= serde_json::from_value::<BTreeMap<String, FontData>>(value.to_owned()).unwrap_or_default();
        for (k, v) in font_data.iter() {
            let _ = fonts.font_data.insert(k.to_owned(), v.to_owned());
        }
    });

    hash_map.get(&FAMILIES_KEY.to_owned()).map(|value| {
        fonts.families = serde_json::from_value(value.to_owned()).unwrap_or_default();
    });
    // Text fonts does not have a default, so it has to be implemented manually instead of being macroed.
        
    fonts
}


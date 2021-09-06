use egui::{FontDefinitions, Style};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const DEFAULT_FONTS: [&str; 4] = [
    "ProggyClean",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];

/// The EguiTheme is the serializable contents of the relevant font information. this is only useful for writing and reading the Style and FontDefinitions from disk.
/// This is essentially a container for `Style` and `FontDefinitions`
/// In addition, it will also serialize the `egui::FontData` into `base64` format to encode the font data directly into the theme
#[derive(Serialize, Deserialize)]
pub struct EguiTheme {
    style: Style,
    font_definitions: FontDefinitions,
    // Need to hold a reference to the font data as FontDefinitions does not serialize it automatically.
    font_data: BTreeMap<String, String>,
}

impl EguiTheme {
    /// Create a new style from
    /// `style` the egui style information
    /// `font_definitions` the current font definitions.
    pub fn new(style: Style, font_definitions: FontDefinitions) -> Self {
        println!("new egui theme!");
        // TODO: Determine if there is a better way to exclude the defaults.
        let mut font_data = BTreeMap::new();
        for (name, data) in font_definitions.font_data.iter() {
            if !DEFAULT_FONTS.contains(&name.as_str()) {
                font_data.insert(name.clone(), base64::encode(data));
            }
        }
        Self {
            style,
            font_definitions,
            font_data,
        }
    }

    /// Extracts the file information  and consumes the theme to retreive the `Style` and `FontDefinitions`
    /// This is the only supported way to move the style and font data into Egui.
    /// This should only be used when loading the theme from disk for the first time.
    pub fn extract(mut self) -> (Style, FontDefinitions) {
        // This is a workaround since the font_data is not automatically serialized.
        // If the keys are not found in the font data, we need to add them before allowing the data to be extracted
        println!("extract!");
        for (key, value) in self.font_data.iter() {
            println!("{}", key);
            if !self.font_definitions.font_data.contains_key(key) {
                println!("data len: {}", value.len());
                let data = base64::decode(value).expect("this should work");
                self.font_definitions
                    .font_data
                    .insert(key.to_owned(), std::borrow::Cow::Owned(data));
            }
        }
        (self.style, self.font_definitions)
    }
}

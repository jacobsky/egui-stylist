#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

pub mod migrations;

use egui::{FontDefinitions, Style};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// This should be incremented anytime there is a new version.
// In addition, a migration feature MUST exist for each version of this in order to preserve compatibility between older version of the
// `EguiTheme` should changes be required.
const CURRENT_VERSION: u32 = 0u32;

const DEFAULT_FONTS: [&str; 4] = [
    "Hack",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];

/// The EguiTheme is the serializable contents of the relevant font information. This is intended to only be used when reading and writing the `Style` and `FontDefinition` information to/from disk.
///
/// The FontData is stored directly into the theme as base64 encoded Strings.
///
/// # Important
/// This should only be used during the Serialization/Deserialization process. Once the loading has completed, this should be extracted directly to an egui::Context as soon as it has been fully loaded.
#[derive(Serialize, Deserialize)]
pub struct EguiTheme {
    /// The version is used internally to determine how it should represent the data.
    version: u32,
    /// This is the serialized EguiFont style information.
    style: Style,
    /// This is the collection of the font definition information required.
    font_definitions: FontDefinitions,
    /// Font data is used to store the serializable font information that is not otherwise serialized by egui.
    font_data: BTreeMap<String, String>,
}

impl EguiTheme {
    /// Create a new style from the required information.
    /// `style` the egui style information
    /// `font_definitions` the current font definitions.
    pub fn new(style: Style, font_definitions: FontDefinitions) -> Self {
        // TODO: Determine if there is a better way to exclude the defaults.
        let mut font_data = BTreeMap::new();
        for (name, data) in font_definitions.font_data.iter() {
            if !DEFAULT_FONTS.contains(&name.as_str()) {
                font_data.insert(name.clone(), base64::encode(data));
            }
        }
        Self {
            version: CURRENT_VERSION,
            style,
            font_definitions,
            font_data,
        }
    }

    /// Returns the version of this theme.
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Extracts the theme information destructively. This will encode all of the serialized data into an `egui::Context` ready format.
    ///
    /// # Important
    /// The theme will no longer be usable after extraction and will move the `Style` and `FontDefinitions` data for use.
    ///
    /// Style and font data should be managed by your application after extraction.
    pub fn extract(mut self) -> (Style, FontDefinitions) {
        // Allows automatic migrations to be performed.
        if cfg!(feature = "migrate_14_to_15") {
            println!("migrate egui 14 -> 15!");
            self = migrations::migration_from_14_to_15(self);
        } else {
            // This is a workaround since the font_data is not automatically serialized.
            // If the keys are not found in the font data, we need to add them before allowing the data to be extracted
            for (key, value) in self.font_data.iter() {
                if !self.font_definitions.font_data.contains_key(key) && !DEFAULT_FONTS.contains(&key.as_str()) {
                    let data = base64::decode(value).expect("this should work");
                    self.font_definitions
                        .font_data
                        .insert(key.to_owned(), std::borrow::Cow::Owned(data));
                }
            }
        }
        

        (self.style, self.font_definitions)
    }
}

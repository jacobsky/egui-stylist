use egui::{FontDefinitions, Style};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod fonts;
mod style;

type ThemeValue = serde_json::Value;

/// The EguiTheme is the serializable contents of the relevant font information. This is intended to only be used when reading and writing the `Style` and `FontDefinition` information to/from disk.
///
/// The FontData is stored directly into the theme as base64 encoded Strings.
///
/// # Important
/// This should only be used during the Serialization/Deserialization process. Once the loading has completed, this should be extracted directly to an egui::Context as soon as it has been fully loaded.
#[derive(Serialize, Deserialize)]
pub struct EguiTheme {
    /// Version of egui_theme that the theme was created with
    pub(crate) egui_theme_version: String,
    /// Version of egui that the theme was created with
    pub(crate) egui_version: String,
    /// The serialized font information for all desired configuration settings.
    pub(crate) style: HashMap<String, ThemeValue>,
    /// Used for
    pub(crate) fonts: HashMap<String, ThemeValue>,
}

impl EguiTheme {
    /// Create a new style from the required information.
    /// `style` the egui style information
    /// `font_definitions` the current font definitions.
    pub fn new(style: Style, font_definitions: FontDefinitions) -> Self {
        let style = style::from_style(style);
        let fonts = fonts::from_fonts(font_definitions);
        Self {
            egui_theme_version: crate::EGUI_THEME_VERSION.to_owned(),
            egui_version: crate::EGUI_VERSION.to_owned(),
            style,
            fonts,
        }
    }

    /// Consumes the deserialized theme destructively to product the style/font
    pub fn extract(self) -> (Style, FontDefinitions) {
        let EguiTheme { style, fonts, .. } = self;
        let style = style::to_style(style);
        let fonts = fonts::to_fonts(fonts);
        (style, fonts)
    }

    pub fn load_into_context(self, context: &mut egui::Context) {
        let (style, fonts) = self.extract();
        context.set_style(style);
        context.set_fonts(fonts);
    }
}

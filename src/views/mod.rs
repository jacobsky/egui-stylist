//! This contains all the views that are used to construct the core of the application.
use std::collections::BTreeMap;

use eframe::egui::{FontDefinitions, ScrollArea, Style, Ui};
use serde::{Deserialize, Serialize};
mod colors;
mod fonts;
mod preview;
// mod shape;
mod spacing;

const DEFAULT_FONTS: [&str; 4] = [
    "ProggyClean",
    "Ubuntu-Light",
    "NotoEmoji-Regular",
    "emoji-icon-font",
];

pub use colors::colors_view;
pub use fonts::fonts_view;
pub use preview::Preview;
// pub use shape::shape_view;
pub use spacing::spacing_view;

use self::fonts::FontViewState;

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
enum StylerTab {
    Colors,
    Fonts,
    Spacing,
    Preview,
}

/// The EguiTheme contains all the relevant style and font infomration needed to successfully create transferrable themes for EguiApplications.
#[derive(Serialize, Deserialize)]
pub struct EguiTheme {
    style: Style,
    font_definitions: FontDefinitions,
    // Need to hold a reference to the font data as FontDefinitions does not serialize it automatically.
    font_data: BTreeMap<String, String>,
}

impl EguiTheme {
    pub fn new(style: Style, font_definitions: FontDefinitions) -> Self {
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
    pub fn style(&self) -> &Style {
        &self.style
    }
    pub fn font_definitions(&self) -> &FontDefinitions {
        &self.font_definitions
    }
    /// Extracts the file information destructively and consumes `self`
    /// This can be used to avoid borrowing the data when importing a new `EguiTheme`
    pub fn extract(mut self) -> (Style, FontDefinitions) {
        // This is a workaround since the font_data is not automatically serialized.
        // If the keys are not found in the font data, we need to add them before allowing the data to be extracted
        for (key, value) in self.font_data.iter() {
            if !self.font_definitions.font_data.contains_key(key) {
                let data = base64::decode(value).expect("this should work");
                self.font_definitions
                    .font_data
                    .insert(key.to_owned(), std::borrow::Cow::Owned(data));
            }
        }
        (self.style, self.font_definitions)
    }
}

/// This is the framework agnostic application state
#[derive(Serialize, Deserialize)]
pub struct StylerState {
    current_tab: StylerTab,
    style: Style,
    font_definitions: FontDefinitions,
    #[cfg_attr(feature = "persistence", serde(skip))]
    font_view_state: FontViewState,
    preview: Preview,
}

impl StylerState {
    pub fn default() -> Self {
        Self {
            current_tab: StylerTab::Colors,
            style: Style::default(),
            font_definitions: FontDefinitions::default(),
            font_view_state: FontViewState::default(),
            preview: Preview::new(Style::default()),
        }
    }
    fn tab_menu_ui(&mut self, ui: &mut Ui) {
        use eframe::egui::widgets::SelectableLabel;
        // Menu tabs
        ui.horizontal(|ui| {
            if ui
                .add(SelectableLabel::new(
                    self.current_tab == StylerTab::Colors,
                    "Colors",
                ))
                .clicked()
            {
                if self.current_tab == StylerTab::Preview {
                    let ctx = ui.ctx();
                    ctx.set_fonts(FontDefinitions::default());
                }
                self.current_tab = StylerTab::Colors;
            }
            if ui
                .add(SelectableLabel::new(
                    self.current_tab == StylerTab::Fonts,
                    "Fonts",
                ))
                .clicked()
            {
                if self.current_tab == StylerTab::Preview {
                    let ctx = ui.ctx();
                    ctx.set_fonts(FontDefinitions::default());
                }
                self.current_tab = StylerTab::Fonts;
            }
            if ui
                .add(SelectableLabel::new(
                    self.current_tab == StylerTab::Spacing,
                    "Spacing",
                ))
                .clicked()
            {
                if self.current_tab == StylerTab::Preview {
                    let ctx = ui.ctx();
                    ctx.set_fonts(FontDefinitions::default());
                }
                self.current_tab = StylerTab::Spacing;
            }
            if ui
                .add(SelectableLabel::new(
                    self.current_tab == StylerTab::Preview,
                    "Preview",
                ))
                .clicked()
            {
                self.current_tab = StylerTab::Preview;
                let ctx = ui.ctx();
                ctx.set_fonts(self.font_definitions.clone());
            }
        });
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        // Get the tab ui
        self.tab_menu_ui(ui);
        ScrollArea::auto_sized().show(ui, |ui| {
            // Show the content views.
            match self.current_tab {
                StylerTab::Colors => colors_view(&mut self.style, ui),
                StylerTab::Fonts => {
                    fonts_view(&mut self.font_view_state, &mut self.font_definitions, ui)
                }
                StylerTab::Spacing => spacing_view(&mut self.style, ui),
                StylerTab::Preview => {
                    self.preview.set_style(self.style.clone());
                    self.preview.show(ui);
                }
            }
        });
    }
    pub fn export_theme(&self) -> EguiTheme {
        EguiTheme::new(self.style.clone(), self.font_definitions.clone())
    }
    pub fn import_theme(&mut self, theme: EguiTheme) {
        self.style = theme.style;
        self.font_definitions = theme.font_definitions;
        for (key, value) in theme.font_data.iter() {
            let data = base64::decode(value).expect("this should work");
            self.font_definitions
                .font_data
                .insert(key.to_owned(), std::borrow::Cow::Owned(data));
        }
    }
}

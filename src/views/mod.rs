//! This contains all the views that are used to construct the core of the application.
use serde::{Serialize, Deserialize};
use eframe::egui::{FontDefinitions, ScrollArea, Style, Ui};
mod colors;
mod fonts;
mod shape;
mod spacing;

pub use colors::colors_view;
pub use fonts::fonts_view;
pub use shape::shape_view;
pub use spacing::spacing_view;

use self::fonts::FontViewState;

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
enum StylerTab {
    Colors, Fonts, Spacing
}
/// This is the framework agnostic application state
#[derive(Serialize, Deserialize)]
pub struct StylerState {
    current_tab: StylerTab,
    style: Style,
    font_definitions: FontDefinitions,
    #[cfg_attr(feature = "persistence", serde(skip))]
    font_view_state: FontViewState,
}

impl StylerState {
    pub fn default() -> Self {
        Self {
            current_tab: StylerTab::Colors,
            style: Style::default(),
            font_definitions: FontDefinitions::default(),
            font_view_state: FontViewState::default(),
        }
    }
    fn tab_menu_ui(&mut self, ui: &mut Ui) {
        use eframe::egui::widgets::SelectableLabel;
        // Thingy
        ui.heading("Category to style");
        ui.spacing();
        // Menu tabs
        ui.horizontal(|ui|{
            if ui.add(SelectableLabel::new(self.current_tab == StylerTab::Colors, "Colors")).clicked() {
                self.current_tab = StylerTab::Colors;
            }
            if ui.add(SelectableLabel::new(self.current_tab == StylerTab::Fonts, "Fonts")).clicked() {
                self.current_tab = StylerTab::Fonts;
            }
            if ui.add(SelectableLabel::new(self.current_tab == StylerTab::Spacing, "Spacing")).clicked() {
                self.current_tab = StylerTab::Spacing;
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
                StylerTab::Fonts => fonts_view(&mut self.font_view_state, &mut self.font_definitions,  ui),
                StylerTab::Spacing => spacing_view(&mut self.style, ui),
            }
        });
    }
}
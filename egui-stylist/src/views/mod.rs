//! This contains all the views that are used to construct the core of the application.
use eframe::egui::{FontDefinitions, ScrollArea, Style, Ui};
use egui_theme::EguiTheme;
use serde::{Deserialize, Serialize};
mod colors;
mod fonts;
mod preview;
// mod shape;
mod spacing;

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
        let (style, font_definitions) = theme.extract();
        self.style = style;
        self.font_definitions = font_definitions;
        println!("{:?}", self.font_definitions.font_data);
    }
}

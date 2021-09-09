//! This contains all the views that are used to construct the core of the application.
use std::path::PathBuf;

use egui::{FontDefinitions, ScrollArea, Style, Ui};
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

type StylistFileDialogFunction =
    Box<dyn Fn(StylistFileDialog, Option<(&str, &[&str])>) -> Option<PathBuf>>;

/// This is used to allow the function intent to select what kind of File Dialog it wishes to open.
pub enum StylistFileDialog {
    Open,
    Save,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
enum StylerTab {
    Colors,
    Fonts,
    Spacing,
    Preview,
}
/// This is the framework agnostic application state
#[derive(Serialize, Deserialize)]
pub struct StylistState {
    current_tab: StylerTab,
    style: Style,
    font_definitions: FontDefinitions,
    #[serde(skip)]
    font_view_state: FontViewState,
    preview: Preview,
    #[serde(skip)]
    pub file_dialog_function: Option<StylistFileDialogFunction>,
}

impl StylistState {
    pub fn default() -> Self {
        Self {
            current_tab: StylerTab::Colors,
            style: Style::default(),
            font_definitions: FontDefinitions::default(),
            font_view_state: FontViewState::default(),
            preview: Preview::new(Style::default()),
            file_dialog_function: None,
        }
    }
    /// Allow `egui` to get open a filepath from the user's perspective.
    /// This is to allow plumbing in of custom File Dialog
    pub fn set_file_dialog_function(&mut self, f: StylistFileDialogFunction) {
        self.file_dialog_function = Some(f);
    }
    /// Calls the file_dialog function and returns a path if it was found
    pub fn file_dialog(
        &self,
        kind: StylistFileDialog,
        filter: Option<(&str, &[&str])>,
    ) -> Option<PathBuf> {
        self.file_dialog_function
            .as_ref()
            .map(|f| f(kind, filter))
            .flatten()
    }

    fn tab_menu_ui(&mut self, ui: &mut Ui) {
        use egui::widgets::SelectableLabel;
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
                StylerTab::Fonts => fonts_view(
                    &mut self.font_view_state,
                    self.file_dialog_function.as_ref(),
                    &mut self.font_definitions,
                    ui,
                ),
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
    }
}

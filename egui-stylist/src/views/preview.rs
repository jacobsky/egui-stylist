use egui::{Style, Ui};
use serde::{Deserialize, Serialize};
mod widget_gallery;
use widget_gallery::WidgetGallery;

/// Allows previewing the current egui framework settings.
#[derive(Serialize, Deserialize)]
pub struct Preview {
    gallery: WidgetGallery,
    style: Style,
}

impl Preview {
    pub fn new(style: Style) -> Self {
        Self {
            gallery: WidgetGallery::default(),
            style,
        }
    }
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.heading("Preview");
        ui.set_style(self.style.clone());
        self.gallery.ui(ui);
    }
}

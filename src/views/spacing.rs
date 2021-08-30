use eframe::egui::{Style, Ui};

/// Displays the Ui to help modify and calculate the current spacing information.
pub fn spacing_view(style: &mut Style, ui: &mut Ui) {
    ui.heading("Spacing Configuration");
    style.spacing.ui(ui)
}
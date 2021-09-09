use egui::{Style, Ui};

/// Displays the Ui to help modify and calculate the current spacing information.
pub fn spacing_view(style: &mut Style, ui: &mut Ui) {
    // TODO: Make a more ergonic spacing UI
    ui.heading("Spacing Settings");
    style.spacing.ui(ui)
}

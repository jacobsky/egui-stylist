use egui::{CollapsingHeader, Color32, Grid, Style, Ui};

/// Displays and modifies the style information related to color for the application.
pub fn colors_view(style: &mut Style, ui: &mut Ui) {
    ui.heading("Color Settings");
    /// This is a convenience macro for building out this specific table structure without the additional boilerplate.
    macro_rules! color_grid {
        ($ui:expr, $($label_color:expr),*) => {
            Grid::new("_properties").num_columns(3).min_col_width(120.0).max_col_width(120.0).show($ui, |ui|{
                ui.label("Property");
                ui.label("Color Picker");
                ui.label("Html Color Value");
                ui.end_row();
                $(
                    ui.label($label_color.0);
                    ui.color_edit_button_srgba($label_color.1);
                    let mut html_color = color_to_html(&$label_color.1);
                    let response = ui.text_edit_singleline(&mut html_color);
                    if response.changed() || response.lost_focus() {
                        if let Some(color) = html_to_color(html_color.as_str()) {
                            *$label_color.1 = color;
                        }
                    }
                    ui.end_row();
                )*
            });
        }
    }
    Grid::new("color_settings").num_columns(3).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            CollapsingHeader::new("General Color Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        ("Background Faint", &mut style.visuals.faint_bg_color),
                        ("Background Extreme", &mut style.visuals.extreme_bg_color),
                        ("Background Code", &mut style.visuals.code_bg_color),
                        // ("Text Override (defaults to Window Text color)", &mut style.visuals.override_text_color)
                        ("Selection Background", &mut style.visuals.selection.bg_fill),
                        ("Selection Text", &mut style.visuals.selection.stroke.color),
                        ("Hyperlink Color", &mut style.visuals.hyperlink_color)
                    );
                });
            CollapsingHeader::new("Window and non-interactive widget Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        (
                            "Window Fill",
                            &mut style.visuals.widgets.noninteractive.bg_fill
                        ),
                        (
                            "Window Text",
                            &mut style.visuals.widgets.noninteractive.fg_stroke.color
                        ),
                        (
                            "Window Outline",
                            &mut style.visuals.widgets.noninteractive.bg_stroke.color
                        ),
                        ("Window Shadow", &mut style.visuals.window_shadow.color)
                    );
                });

            CollapsingHeader::new("Inactive Interactive Widget Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        ("Fill", &mut style.visuals.widgets.inactive.bg_fill),
                        ("Text", &mut style.visuals.widgets.inactive.fg_stroke.color),
                        (
                            "Outline",
                            &mut style.visuals.widgets.inactive.bg_stroke.color
                        )
                    );
                });
            CollapsingHeader::new("Hovered Interactive Widget Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        ("Fill", &mut style.visuals.widgets.hovered.bg_fill),
                        ("Text", &mut style.visuals.widgets.hovered.fg_stroke.color),
                        (
                            "Outline",
                            &mut style.visuals.widgets.hovered.bg_stroke.color
                        )
                    );
                });
            CollapsingHeader::new("Active Interactive Widget Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        ("Fill", &mut style.visuals.widgets.active.bg_fill),
                        ("Text", &mut style.visuals.widgets.active.fg_stroke.color),
                        ("Outline", &mut style.visuals.widgets.active.bg_stroke.color)
                    );
                });
            CollapsingHeader::new("Open Menu Widget Settings")
                .default_open(true)
                .show(ui, |ui| {
                    color_grid!(
                        ui,
                        ("Fill", &mut style.visuals.widgets.open.bg_fill),
                        ("Text", &mut style.visuals.widgets.open.fg_stroke.color),
                        ("Outline", &mut style.visuals.widgets.open.bg_stroke.color)
                    );
                });
        });
    });
}

fn color_to_html(color: &Color32) -> String {
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        color.r(),
        color.g(),
        color.b(),
        color.a()
    )
}

// Returns the html color if it is able to or None if it cannot convert it.
fn html_to_color(html: &str) -> Option<Color32> {
    use std::convert::TryFrom;
    let mut html = html.chars();
    // HTML Color codes require exactly 9 characters (hash + 8 digits).
    // We will also support 6 where we assume that alpha is 255.
    if let Some(char) = html.next() {
        if char != '#' {
            return None;
        }
    }
    let red = if let (Some(red1), Some(red2)) = (html.next(), html.next()) {
        if let (Some(r1), Some(r2)) = (red1.to_digit(16), red2.to_digit(16)) {
            Some(r1 * 16 + r2)
        } else {
            None
        }
    } else {
        None
    };
    let green = if let (Some(green1), Some(green2)) = (html.next(), html.next()) {
        if let (Some(g1), Some(g2)) = (green1.to_digit(16), green2.to_digit(16)) {
            Some(g1 * 16 + g2)
        } else {
            None
        }
    } else {
        None
    };
    let blue = if let (Some(blue1), Some(blue2)) = (html.next(), html.next()) {
        if let (Some(b1), Some(b2)) = (blue1.to_digit(16), blue2.to_digit(16)) {
            Some(b1 * 16 + b2)
        } else {
            None
        }
    } else {
        None
    };
    let alpha = if let (Some(alpha1), Some(alpha2)) = (html.next(), html.next()) {
        if let (Some(a1), Some(a2)) = (alpha1.to_digit(16), alpha2.to_digit(16)) {
            Some(a1 * 16 + a2)
        } else {
            None
        }
    } else {
        Some(255)
    };
    if let (Some(r), Some(b), Some(g), Some(a)) = (red, green, blue, alpha) {
        if let (Ok(r), Ok(b), Ok(g), Ok(a)) = (
            u8::try_from(r),
            u8::try_from(g),
            u8::try_from(b),
            u8::try_from(a),
        ) {
            Some(Color32::from_rgba_premultiplied(r, g, b, a))
        } else {
            None
        }
    } else {
        None
    }
}

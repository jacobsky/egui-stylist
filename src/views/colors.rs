use eframe::egui::{Style, Ui, Grid, widgets, Color32, Response};

// TODO: Use this to differentiate
// pub enum ColorEditType {
//     RGB, HSV, RGBA, SRGBA
// }

/// Displays and modifies the style information related to color for the application.
pub fn colors_view(style: &mut Style, ui: &mut Ui) {
    ui.heading("Color Settings");
    // macro_rules! color_grid_for_settings
    /// This is a convenience macro for building out this specific table structure without the additional boilerplate.
    macro_rules! add_color_grid_rows {
        ($ui:expr, $($label_color:expr),*) => {
            $ui.label("Property");
            $ui.label("Color Picker");
            $ui.label("Html Color Value");
            $ui.end_row();
            $(
                $ui.label($label_color.0);
                $ui.color_edit_button_srgba($label_color.1);
                let mut html_color = color_to_html(&$label_color.1);
                let response = $ui.text_edit_singleline(&mut html_color);
                if response.changed() || response.lost_focus() {
                    if let Some(color) = html_to_color(html_color.as_str()) {
                        *$label_color.1 = color;
                    } 
                }
                $ui.end_row();
            )*
        }
    }
    Grid::new("color_settings").num_columns(3).show(ui, | ui |{
        ui.heading("General Color Settings");
        
        ui.end_row();
        // ui.label("test color");
        // ui.color_edit_button_srgba(&mut style.visuals.extreme_bg_color);
        // let mut html_color = color_to_html(&style.visuals.extreme_bg_color);
        // let response = ui.text_edit_singleline(&mut html_color);
        // if response.changed() || response.lost_focus() {
        //     if let Some(color) = html_to_color(html_color.as_str()) {
        //         style.visuals.extreme_bg_color = color;
        //     } 
        // }
        // ui.end_row();
        add_color_grid_rows!(ui,
            ("Background Faint", &mut style.visuals.faint_bg_color),
            ("Background Extreme", &mut style.visuals.extreme_bg_color),
            ("Background Code", &mut style.visuals.code_bg_color),
            // ("Text Override (defaults to Window Text color)", &mut style.visuals.override_text_color)
            ("Selection Background", &mut style.visuals.selection.bg_fill),
            ("Selection Text", &mut style.visuals.selection.stroke.color),
            ("Hyperlink Color", &mut style.visuals.hyperlink_color)
        );
        ui.heading("Window and non-interactive widget Settings");
        ui.end_row();
        add_color_grid_rows!(ui,
            ("Window Fill", &mut style.visuals.widgets.noninteractive.bg_fill),
            ("Window Text", &mut style.visuals.widgets.noninteractive.fg_stroke.color),
            ("Window Outline", &mut style.visuals.widgets.noninteractive.bg_stroke.color),
            ("Window Shadow", &mut style.visuals.window_shadow.color)
        );
        
        ui.heading("Inactive Interactive Widget Settings");
        ui.end_row();
        add_color_grid_rows!(ui,
            ("Fill", &mut style.visuals.widgets.inactive.bg_fill),
            ("Text", &mut style.visuals.widgets.inactive.fg_stroke.color),
            ("Outline", &mut style.visuals.widgets.inactive.bg_stroke.color)
        );
        ui.heading("Hovered Interactive Widget Settings");
        ui.end_row();
        add_color_grid_rows!(ui,
            ("Fill", &mut style.visuals.widgets.hovered.bg_fill),
            ("Text", &mut style.visuals.widgets.hovered.fg_stroke.color),
            ("Outline", &mut style.visuals.widgets.hovered.bg_stroke.color)
        );
        ui.heading("Active Interactive Widget Settings");
        ui.end_row();
        add_color_grid_rows!(ui,
            ("Fill", &mut style.visuals.widgets.active.bg_fill),
            ("Text", &mut style.visuals.widgets.active.fg_stroke.color),
            ("Outline", &mut style.visuals.widgets.active.bg_stroke.color)
        );
        ui.heading("Open Menu Widget Settings");
        ui.end_row();
        add_color_grid_rows!(ui,
            ("Fill", &mut style.visuals.widgets.open.bg_fill),
            ("Text", &mut style.visuals.widgets.open.fg_stroke.color),
            ("Outline", &mut style.visuals.widgets.open.bg_stroke.color)
        );



        // TODO: Add more here
    });
}


fn color_to_html(color: &Color32) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", color.r(), color.g(), color.b(), color.a())
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
        } else { None }
    } else { None };
    let green = if let (Some(green1), Some(green2)) = (html.next(), html.next()) {
        if let (Some(g1), Some(g2)) = (green1.to_digit(16), green2.to_digit(16)) {
            Some(g1 * 16 + g2)
        } else { None }
    } else { None };
    let blue = if let (Some(blue1), Some(blue2)) = (html.next(), html.next()) {
        if let (Some(b1), Some(b2)) = (blue1.to_digit(16), blue2.to_digit(16)) {
            Some(b1 * 16 + b2)
        } else { None }
    } else { None };
    let alpha = if let (Some(alpha1), Some(alpha2)) = (html.next(), html.next()) {
        if let (Some(a1), Some(a2)) = (alpha1.to_digit(16), alpha2.to_digit(16)) {
            Some(a1 * 16 + a2)
        } else { None }
    } else { Some(255) };
    if let (Some(r), Some(b), Some(g), Some(a)) = (red, green, blue, alpha) {
        if let (Ok(r), Ok(b), Ok(g), Ok(a)) = (u8::try_from(r), u8::try_from(g), u8::try_from(b), u8::try_from(a)) {
            Some(Color32::from_rgba_premultiplied(r, g, b, a))
        } else {
            None
        }
    } else {
        None
    }
}

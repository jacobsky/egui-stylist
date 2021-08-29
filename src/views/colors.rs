use eframe::egui::{Style, Ui, Grid, widgets, Color32, Response};

// TODO: Use this to differentiate
pub enum ColorEditType {
    RGB, HSV, RGBA, SRGBA
}

/// Displays and modifies the style information related to color for the application.
pub fn colors_view(style: &mut Style, ui: &mut Ui) {
    macro_rules! add_color_grid_rows {
        ($ui:expr, $($label_color:expr),*) => {
            $(
                $ui.label($label_color.0);
                $ui.color_edit_button_srgba($label_color.1);
                $ui.end_row();
            )*
        }
    }
    Grid::new("color_settings").num_columns(2).show(ui, | ui |{
        ui.heading("General");
        ui.end_row();
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

fn edit_color(ui: &mut Ui, srgba: &mut Color32, text: impl Into<widgets::Label>) -> Response {
    // ui.horizontal(|ui| {
        let response = ui.color_edit_button_srgba(srgba);
        ui.label(text);
        response
    // })
    // .response
}

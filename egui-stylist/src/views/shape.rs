use egui::{Checkbox, DragValue, Grid, Style, Ui, Widget};

/// Displays the current font definition from the core app widget and displays the ui to detect any addition changes.
pub fn shape_view(style: &mut Style, ui: &mut Ui) {
    // shape definitions.

    ui.heading("Shape Configuration");
    Grid::new("customization").show(ui, |ui| {
        ui.label("Noninteractive Widget Stroke Width");
        DragValue::new(&mut style.visuals.widgets.noninteractive.bg_stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Inactive Widget Stroke Width");
        DragValue::new(&mut style.visuals.widgets.inactive.bg_stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Active Widget Stroke Width");
        DragValue::new(&mut style.visuals.widgets.active.bg_stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Hovered Widget Stroke Width");
        DragValue::new(&mut style.visuals.widgets.hovered.bg_stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Open Widget Stroke Width");
        DragValue::new(&mut style.visuals.widgets.open.bg_stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Selection Stroke Width");
        DragValue::new(&mut style.visuals.selection.stroke.width)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Resize Grab Radius Side");
        DragValue::new(&mut style.interaction.resize_grab_radius_side)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Resize Grab Radius Corner");
        DragValue::new(&mut style.interaction.resize_grab_radius_corner)
            .clamp_range(0.0f32..=100.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Show tooltips only when still");
        Checkbox::new(&mut style.interaction.show_tooltips_only_when_still, "").ui(ui);
        ui.end_row();

        ui.label("Window Rounding - NW");
        DragValue::new(&mut style.visuals.window_rounding.nw)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();
        ui.label("Window Rounding - NE");
        DragValue::new(&mut style.visuals.window_rounding.ne)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();
        ui.label("Window Rounding - SW");
        DragValue::new(&mut style.visuals.window_rounding.sw)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Window Rounding - SE");
        DragValue::new(&mut style.visuals.window_rounding.se)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Window Shadow Extrusion");
        DragValue::new(&mut style.visuals.window_shadow.extrusion)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Popup Shadow Extrusion");
        DragValue::new(&mut style.visuals.popup_shadow.extrusion)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Resize Corner Radius");
        DragValue::new(&mut style.visuals.resize_corner_size)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Text Cursor Width");
        DragValue::new(&mut style.visuals.text_cursor_width)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Toggle Text Cursor Preview");
        Checkbox::new(&mut style.visuals.text_cursor_preview, "").ui(ui);
        ui.end_row();

        ui.label("Clip Rect Margin");
        DragValue::new(&mut style.visuals.clip_rect_margin)
            .clamp_range(0.0f32..=50.0f32)
            .ui(ui);
        ui.end_row();

        ui.label("Show button frame");
        Checkbox::new(&mut style.visuals.button_frame, "").ui(ui);
        ui.end_row();

        ui.label("Collapsing Header Frame");
        Checkbox::new(&mut style.visuals.collapsing_header_frame, "").ui(ui);
        ui.end_row();
    });
}

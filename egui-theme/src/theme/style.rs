use std::collections::HashMap;
use egui::Style;

macro_rules! serialize_property {
    ($collection:expr, $property:expr) => {
        if let Ok(value) = serde_json::to_value($property.to_owned()) {
           $collection.insert(stringify!($property).to_owned(), value); 
        }
    };
}


macro_rules! serialize_properties {
    ($collection:expr, $( $property:expr), +) => {
        $(
            serialize_property!($collection, $property);
        )+
    };
}

// TODO: Fix this so that it does not assign to property if it cannot extract it.
macro_rules! deserialize_property {
    ($collection:expr, $property:expr) => {
        $collection.get(&stringify!($property).to_owned())
            .map(|value| {
                    if let Ok(deserialized_value) = serde_json::from_value(value.to_owned()) {
                        $property = deserialized_value;
                    }
                }
            );
    };
}

macro_rules! deserialize_properties {
    ($collection:expr, $( $property:expr), +) => {
        $(
            deserialize_property!($collection, $property);
        )+
    };
}

/// TODO: Comment this function
pub fn from_style(style: Style) -> HashMap<String, super::ThemeValue> {
    let mut hash_map = HashMap::new();
    serialize_properties!(
        hash_map,
        style.override_text_style,
        style.override_font_id,
        style.text_styles,
        style.wrap,

        style.animation_time,
        style.explanation_tooltips,

        style.spacing.item_spacing,
        style.spacing.window_margin,
        style.spacing.button_padding,
        style.spacing.indent,
        style.spacing.interact_size,
        style.spacing.slider_width,
        style.spacing.text_edit_width,
        style.spacing.icon_width,
        style.spacing.icon_spacing,
        style.spacing.tooltip_width,
        style.spacing.indent_ends_with_horizontal_line,
        style.spacing.combo_height,
        style.spacing.scroll_bar_width,

        style.interaction.resize_grab_radius_side,
        style.interaction.resize_grab_radius_corner,
        style.interaction.show_tooltips_only_when_still,
        style.visuals.dark_mode,
        style.visuals.override_text_color,
        style.visuals.widgets,
        style.visuals.selection,
        style.visuals.hyperlink_color,
        style.visuals.faint_bg_color,
        style.visuals.extreme_bg_color,
        style.visuals.code_bg_color,
        style.visuals.window_rounding,
        style.visuals.window_shadow,
        style.visuals.popup_shadow,
        style.visuals.resize_corner_size,
        style.visuals.text_cursor_width,
        style.visuals.text_cursor_preview,
        style.visuals.clip_rect_margin,
        style.visuals.button_frame,
        style.visuals.collapsing_header_frame
    );
    
    hash_map
}


/// TODO: Comment this function
pub fn to_style(hash_map: HashMap<String, super::ThemeValue>) -> Style {
    let mut style = Style::default();
    deserialize_properties!(
        hash_map, 
        style.override_text_style,
        style.override_font_id,
        style.text_styles,
        style.wrap,

        style.animation_time,
        style.explanation_tooltips,

        style.spacing.item_spacing,
        style.spacing.window_margin,
        style.spacing.button_padding,
        style.spacing.indent,
        style.spacing.interact_size,
        style.spacing.slider_width,
        style.spacing.text_edit_width,
        style.spacing.icon_width,
        style.spacing.icon_spacing,
        style.spacing.tooltip_width,
        style.spacing.indent_ends_with_horizontal_line,
        style.spacing.combo_height,
        style.spacing.scroll_bar_width,

        style.interaction.resize_grab_radius_side,
        style.interaction.resize_grab_radius_corner,
        style.interaction.show_tooltips_only_when_still,
        style.visuals.dark_mode,
        style.visuals.override_text_color,
        style.visuals.widgets,
        style.visuals.selection,
        style.visuals.hyperlink_color,
        style.visuals.faint_bg_color,
        style.visuals.extreme_bg_color,
        style.visuals.code_bg_color,
        style.visuals.window_rounding,
        style.visuals.window_shadow,
        style.visuals.popup_shadow,
        style.visuals.resize_corner_size,
        style.visuals.text_cursor_width,
        style.visuals.text_cursor_preview,
        style.visuals.clip_rect_margin,
        style.visuals.button_frame,
        style.visuals.collapsing_header_frame
    );
    style
}


// /// BEFORE:
// fn try_parse_rounded_corner(value: JsonValue) -> Result<f32> {
//     value.try_to_f32();
// }

// // AFTER:

// fn try_parse_rounded_corner(value: JsonValue) -> Result<RoundedCorner> {
//     // ...
// }

// fn try_parse_rounded_corner_fallback(value: JsonValue) -> Result<RoundedCorner> {
//     let single = value.try_to_f32()?;
//     RoundedCorner { top_left: single, top_right: single, ... }
// }
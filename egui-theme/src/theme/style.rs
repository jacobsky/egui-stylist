use std::collections::HashMap;
use egui::Style;

macro_rules! ser {
    ($collection:ident, $style:ident, $prop:ident) => {
        if let Ok(value) = serde_json::to_value($style.$prop.to_owned()) {
            $collection.insert(stringify!($prop).to_owned(), value); 
        }
    };
    ($collection:ident, $style:ident, $prop:ident, $sub_prop:ident) => {
        if let Ok(value) = serde_json::to_value($style.$prop.$sub_prop.to_owned()) {
            $collection.insert(stringify!($prop.$sub_prop).to_owned(), value); 
        }
    };
}

macro_rules! de {
    ($collection:ident, $style:ident, $prop:ident) => {
        $collection.get(&stringify!($prop).to_owned())
            .map(|value| {
                if let Ok(deserialized_value) = serde_json::from_value(value.to_owned()) {
                    $style.$prop = deserialized_value;
                }
            }
        );
    };
    ($collection:ident, $style:ident, $prop:ident, $sub_prop:ident) => {
        $collection.get(&stringify!($prop.$sub_prop).to_owned())
            .map(|value| {
                if let Ok(deserialized_value) = serde_json::from_value(value.to_owned()) {
                    $style.$prop.$sub_prop = deserialized_value;
                }
            }
        );
    };
}

/// Helper function to serialize the `egui::Style`
pub fn from_style(style: Style) -> HashMap<String, super::ThemeValue> {
    let mut hash_map = HashMap::new();
    
    ser!(hash_map, style, override_text_style); 
    ser!(hash_map, style, override_font_id); 
    ser!(hash_map, style, text_styles); 
    ser!(hash_map, style, wrap); 

    ser!(hash_map, style, animation_time); 
    ser!(hash_map, style, explanation_tooltips); 

    ser!(hash_map, style, spacing, item_spacing);
    ser!(hash_map, style, spacing, window_margin);
    ser!(hash_map, style, spacing, button_padding);
    ser!(hash_map, style, spacing, indent);
    ser!(hash_map, style, spacing, interact_size);
    ser!(hash_map, style, spacing, slider_width);
    ser!(hash_map, style, spacing, text_edit_width);
    ser!(hash_map, style, spacing, icon_width);
    ser!(hash_map, style, spacing, icon_spacing);
    ser!(hash_map, style, spacing, tooltip_width);
    ser!(hash_map, style, spacing, indent_ends_with_horizontal_line);
    ser!(hash_map, style, spacing, combo_height);
    ser!(hash_map, style, spacing, scroll_bar_width);

    ser!(hash_map, style, interaction, resize_grab_radius_side);
    ser!(hash_map, style, interaction, resize_grab_radius_corner);
    ser!(hash_map, style, interaction, show_tooltips_only_when_still);
    ser!(hash_map, style, visuals, dark_mode);
    ser!(hash_map, style, visuals, override_text_color);
    ser!(hash_map, style, visuals, widgets);
    ser!(hash_map, style, visuals, selection);
    ser!(hash_map, style, visuals, hyperlink_color);
    ser!(hash_map, style, visuals, faint_bg_color);
    ser!(hash_map, style, visuals, extreme_bg_color);
    ser!(hash_map, style, visuals, code_bg_color);
    ser!(hash_map, style, visuals, window_rounding);
    ser!(hash_map, style, visuals, window_shadow);
    ser!(hash_map, style, visuals, popup_shadow);
    ser!(hash_map, style, visuals, resize_corner_size);
    ser!(hash_map, style, visuals, text_cursor_width);
    ser!(hash_map, style, visuals, text_cursor_preview);
    ser!(hash_map, style, visuals, clip_rect_margin);
    ser!(hash_map, style, visuals, button_frame);
    ser!(hash_map, style, visuals, collapsing_header_frame);
    
    hash_map
}


/// Helper function to deserialize the `egui::Style`
pub fn to_style(hash_map: HashMap<String, super::ThemeValue>) -> Style {
    let mut style = Style::default();
 
    de!(hash_map, style, override_text_style); 
    de!(hash_map, style, override_font_id); 
    de!(hash_map, style, text_styles); 
    de!(hash_map, style, wrap); 

    de!(hash_map, style, animation_time); 
    de!(hash_map, style, explanation_tooltips); 

    de!(hash_map, style, spacing, item_spacing);
    de!(hash_map, style, spacing, window_margin);
    de!(hash_map, style, spacing, button_padding);
    de!(hash_map, style, spacing, indent);
    de!(hash_map, style, spacing, interact_size);
    de!(hash_map, style, spacing, slider_width);
    de!(hash_map, style, spacing, text_edit_width);
    de!(hash_map, style, spacing, icon_width);
    de!(hash_map, style, spacing, icon_spacing);
    de!(hash_map, style, spacing, tooltip_width);
    de!(hash_map, style, spacing, indent_ends_with_horizontal_line);
    de!(hash_map, style, spacing, combo_height);
    de!(hash_map, style, spacing, scroll_bar_width);

    de!(hash_map, style, interaction, resize_grab_radius_side);
    de!(hash_map, style, interaction, resize_grab_radius_corner);
    de!(hash_map, style, interaction, show_tooltips_only_when_still);
    de!(hash_map, style, visuals, dark_mode);
    de!(hash_map, style, visuals, override_text_color);
    de!(hash_map, style, visuals, widgets);
    de!(hash_map, style, visuals, selection);
    de!(hash_map, style, visuals, hyperlink_color);
    de!(hash_map, style, visuals, faint_bg_color);
    de!(hash_map, style, visuals, extreme_bg_color);
    de!(hash_map, style, visuals, code_bg_color);
    de!(hash_map, style, visuals, window_rounding);
    de!(hash_map, style, visuals, window_shadow);
    de!(hash_map, style, visuals, popup_shadow);
    de!(hash_map, style, visuals, resize_corner_size);
    de!(hash_map, style, visuals, text_cursor_width);
    de!(hash_map, style, visuals, text_cursor_preview);
    de!(hash_map, style, visuals, clip_rect_margin);
    de!(hash_map, style, visuals, button_frame);
    de!(hash_map, style, visuals, collapsing_header_frame);

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
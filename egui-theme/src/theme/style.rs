use egui::Style;
use std::collections::HashMap;

const TEXT_STYLES_KEY: &str = "text_styles";

// TODO: Change the println! to a proper logging crate.
macro_rules! ser {
    ($collection:ident, $style:ident, $prop:ident) => {
        match serde_json::to_value($style.$prop.to_owned()) {
            Ok(value) => {
                let _ = $collection.insert(stringify!($prop).to_owned(), value);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    };
    ($collection:ident, $style:ident, $prop:ident, $sub_prop:ident) => {
        match serde_json::to_value($style.$prop.$sub_prop.to_owned()) {
            Ok(value) => {
                let _ = $collection.insert(stringify!($prop.$sub_prop).to_owned(), value);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    };
}

macro_rules! de {
    ($collection:ident, $style:ident, $prop:ident) => {
        $collection.get(&stringify!($prop).to_owned()).map(|value| {
            if let Ok(deserialized_value) = serde_json::from_value(value.to_owned()) {
                $style.$prop = deserialized_value;
            }
        });
    };
    ($collection:ident, $style:ident, $prop:ident, $sub_prop:ident) => {
        $collection
            .get(&stringify!($prop.$sub_prop).to_owned())
            .map(|value| {
                if let Ok(deserialized_value) = serde_json::from_value(value.to_owned()) {
                    $style.$prop.$sub_prop = deserialized_value;
                }
            });
    };
}

/// Helper function to serialize the `egui::Style`
pub fn from_style(style: Style) -> HashMap<String, super::ThemeValue> {
    let mut hash_map = HashMap::new();

    {
        let text_styles = style
            .text_styles
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<Vec<_>>();
        if let Ok(value) = serde_json::to_value(text_styles) {
            hash_map.insert(TEXT_STYLES_KEY.to_owned(), value);
        }
    }
    // Text Styles are a special case due to being a map that must serialize to a string.
    // ser!(hash_map, style, text_styles);

    ser!(hash_map, style, override_text_style);
    ser!(hash_map, style, override_font_id);
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
    // Special case due to json requiring String keys
    {
        if let Some(value) = hash_map.get(TEXT_STYLES_KEY) {
            if let Ok(values) =
                serde_json::from_value::<Vec<(egui::TextStyle, egui::FontId)>>(value.to_owned())
            {
                for (key, value) in values {
                    style.text_styles.insert(key, value);
                }
            }
        }
    }

    de!(hash_map, style, override_text_style);
    de!(hash_map, style, override_font_id);
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

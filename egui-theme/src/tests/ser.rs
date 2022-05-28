use crate::EguiTheme;
#[test]
fn test_serialize_theme() {
    let style = egui::Style::default();
    let fonts = egui::FontDefinitions::default();
    let theme = EguiTheme::new(style, fonts);
    let serialized = serde_json::to_string(&theme).expect("serialization failed");
    assert!(serialized.contains(crate::EGUI_THEME_VERSION), "must contain egui_theme version");
    assert!(serialized.contains(crate::EGUI_VERSION), "must contain egui version");
    assert!(serialized.contains("fonts"), "must contain egui version");
    assert!(serialized.contains("style"), "must contain egui version");
}

#[test]
fn test_default_theme_extract() {
    let theme = EguiTheme::new(
        egui::Style::default(), egui::FontDefinitions::default()
    );
    let (style, fonts) = theme.extract();
    let default_style = egui::Style::default();
    let default_fonts = egui::FontDefinitions::default();
    assert_eq!(style.animation_time, default_style.animation_time, "style.animation time is not default");
    assert_eq!(style.interaction.resize_grab_radius_corner, default_style.interaction.resize_grab_radius_corner, "style.animation time is not default");

    for name in crate::DEFAULT_FONTS.iter() {
        assert!(fonts.font_data.contains_key(*name), "font doesn't property have the data for `{name}`.");
        let default_font = default_fonts.font_data.get(*name).expect("{name} does not exist");
        let serialized_font = default_fonts.font_data.get(*name).expect("{name} does not exist");
        assert_eq!(default_font, serialized_font);
    }
}
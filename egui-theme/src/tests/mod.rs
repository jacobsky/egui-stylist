use crate::EguiTheme;

#[test]
fn test_default_theme() {
    let theme = EguiTheme::new(egui::Style::default(), egui::FontDefinitions::default());
    assert_eq!(theme.egui_theme_version, crate::EGUI_THEME_VERSION, "egui_theme version must match");
    assert_eq!(theme.egui_version, crate::EGUI_VERSION, "egui_Version must match");

    assert!(theme.fonts.contains_key("font_data"), "the font_data key should exist");
    assert!(theme.fonts.contains_key("fonts_for_family"), "the font_for_family key should exist");
    assert!(theme.fonts.contains_key("family_and_size"), "the family_and_size key should exist");

    assert!(theme.style.contains_key("style.wrap"), "style.wrap key should exist");
    assert!(theme.style.contains_key("style.explanation_tooltips"), "style.explanation_tooltips key should exist");
    assert!(theme.style.contains_key("style.body_text_style"), "style.body_text_style key should exist");
    assert!(theme.style.contains_key("style.override_text_style"), "style.override_text_style key should exist");
    assert!(theme.style.contains_key("style.animation_time"), "style.animation_time key should exist");
    assert!(theme.style.contains_key("style.wrap"), "style.wrap key should exist");
}
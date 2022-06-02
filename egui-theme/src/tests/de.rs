use egui::{FontDefinitions, FontFamily, Style};

use crate::EguiTheme;

#[test]
fn test_serialize_default() {
    let theme = include_str!("test-themes/default.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    assert_eq!(
        deserialized_theme.egui_theme_version,
        crate::EGUI_THEME_VERSION,
        "egui_theme_version should match"
    );
    assert_eq!(
        deserialized_theme.egui_version,
        crate::EGUI_VERSION,
        "egui_version should match"
    );
    let (style, fonts) = deserialized_theme.extract();
    let default_style = Style::default();
    let default_fonts = FontDefinitions::default();
    assert_eq!(style, default_style, "default should match");
    assert_eq!(fonts, default_fonts, "default should match");
}

#[test]
fn test_deserialize_custom() {
    let theme = include_str!("test-themes/custom-theme.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    assert_eq!(
        deserialized_theme.egui_theme_version,
        crate::EGUI_THEME_VERSION,
        "egui_theme_version should match"
    );
    assert_eq!(
        deserialized_theme.egui_version,
        crate::EGUI_VERSION,
        "egui_version should match"
    );
    let (style, fonts) = deserialized_theme.extract();
    assert_eq!(
        style.spacing.slider_width, 100f32,
        "spacing.slider_width should be 100"
    );
    assert_eq!(
        style.visuals.popup_shadow.color,
        egui::Color32::from_rgba_premultiplied(0, 0, 0, 96),
        "spacing.slider_width should be 100"
    );
    assert_eq!(
        style.spacing.slider_width, 100f32,
        "spacing.slider_width should be 100"
    );
    assert_eq!(
        style.visuals.clip_rect_margin, 3f32,
        "visuals.clip_rect_margin should be 3"
    );
    assert_eq!(style.override_font_id, None, "Override font should be None");
    assert_eq!(
        style.visuals.faint_bg_color,
        egui::Color32::from_rgba_premultiplied(35, 35, 35, 255),
        "color should be (35, 35, 35, 255)"
    );

    assert!(
        fonts.font_data.contains_key("Nacelle-Regul"),
        "font data should contain Nacelle-Regul"
    );
    assert!(
        fonts.font_data.contains_key("Nacelle-Regula"),
        "font data should contain Nacelle-Regula"
    );
    assert!(
        fonts.font_data.contains_key("Nacelle-Regular"),
        "font data should contain Nacelle-Regular"
    );

    let associated_fonts = fonts
        .families
        .get(&FontFamily::Name("asdfasdf".into()))
        .expect("this should have some fonts");
    assert!(
        associated_fonts.contains(&"Ubuntu-Light".to_owned()),
        "needs to have `Ubuntu-Light`"
    );
    assert!(
        associated_fonts.contains(&"NotoEmoji-Regular".to_owned()),
        "needs to have `Nacelle-Regular`"
    );
    assert!(
        associated_fonts.contains(&"Nacelle-Regul".to_owned()),
        "needs to have `Nacelle-Regul`"
    );
    // let default_style = Style::default();
    // let default_fonts = FontDefinitions::default();
}

#[test]
fn test_deserialize_missing_data() {
    let theme = include_str!("test-themes/missing_data.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    let (style, fonts) = deserialized_theme.extract();
    let default_style = Style::default();
    let default_fonts = FontDefinitions::default();
    assert_eq!(style, default_style, "default should match");
    assert_eq!(fonts, default_fonts, "default should match");
}
#[test]
fn test_deserialize_incorrect_parameters() {
    let theme = include_str!("test-themes/incorrect_parameters.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    let (style, fonts) = deserialized_theme.extract();
    let default_style = Style::default();
    let default_fonts = FontDefinitions::default();
    assert_eq!(style, default_style, "default should match");
    assert_eq!(fonts, default_fonts, "default should match");
}

#[test]
fn test_deserialize_incorrect_fields() {
    let theme = include_str!("test-themes/incorrect_fields.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    let (style, fonts) = deserialized_theme.extract();
    let default_style = Style::default();
    let default_fonts = FontDefinitions::default();
    assert_eq!(style, default_style, "default should match");
    assert_eq!(fonts, default_fonts, "default should match");
}

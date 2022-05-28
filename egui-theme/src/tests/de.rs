use egui::{Style, FontDefinitions};

use crate::EguiTheme;

#[test]
fn test_serialize_default() {
    let theme = include_str!("test-themes/default.ron");
    let deserialized_theme = ron::from_str::<EguiTheme>(theme).expect("failed to deserialize");
    let (style, fonts) = deserialized_theme.extract();
    let default_style = Style::default();
    let default_fonts = FontDefinitions::default();
    assert_eq!(style, default_style, "default should match");
    assert_eq!(fonts, default_fonts, "default should match");
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
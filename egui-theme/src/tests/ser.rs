use egui::{FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle};

use crate::EguiTheme;
#[test]
fn test_serialize_theme() {
    let style = egui::Style::default();
    let fonts = egui::FontDefinitions::default();
    let theme = EguiTheme::new(style, fonts);
    let serialized = serde_json::to_string(&theme).expect("serialization failed");
    assert!(
        serialized.contains(crate::EGUI_THEME_VERSION),
        "must contain egui_theme version"
    );
    assert!(
        serialized.contains(crate::EGUI_VERSION),
        "must contain egui version"
    );
    assert!(serialized.contains("fonts"), "must contain egui version");
    assert!(serialized.contains("style"), "must contain egui version");
}

#[test]
fn test_default_theme_extract() {
    let theme = EguiTheme::new(egui::Style::default(), egui::FontDefinitions::default());
    let (style, fonts) = theme.extract();
    let default_style = egui::Style::default();
    let default_fonts = egui::FontDefinitions::default();
    assert_eq!(
        style.animation_time, default_style.animation_time,
        "style.animation time is not default"
    );
    assert_eq!(
        style.interaction.resize_grab_radius_corner,
        default_style.interaction.resize_grab_radius_corner,
        "style.animation time is not default"
    );

    for name in crate::DEFAULT_FONTS.iter() {
        assert!(
            fonts.font_data.contains_key(*name),
            "font doesn't property have the data for `{name}`."
        );
        let default_font = default_fonts
            .font_data
            .get(*name)
            .expect("{name} does not exist");
        let serialized_font = default_fonts
            .font_data
            .get(*name)
            .expect("{name} does not exist");
        assert_eq!(default_font, serialized_font);
    }
}

#[test]
fn test_custom_font_extract() {
    const FONT_NAME: &str = "Nacelle";
    let mut font_definitions = egui::FontDefinitions::default();
    font_definitions.font_data.insert(
        FONT_NAME.to_owned(),
        FontData::from_static(include_bytes!("test-fonts/Nacelle-Regular.otf")),
    );
    let result = font_definitions.families.insert(
        egui::FontFamily::Name(FONT_NAME.into()),
        vec![FONT_NAME.to_owned()],
    );
    assert_eq!(result, None, "None was not returned");

    let theme = EguiTheme::new(egui::Style::default(), font_definitions);

    let (_, fonts) = theme.extract();
    assert!(
        fonts.font_data.contains_key(&FONT_NAME.to_owned()),
        "does not have `Nacelle` key"
    );
    assert_eq!(
        fonts.font_data.get(&FONT_NAME.to_owned()).unwrap(),
        &FontData::from_static(include_bytes!("test-fonts/Nacelle-Regular.otf")),
        "font data does not match"
    );
    assert!(
        fonts.families.contains_key(&egui::FontFamily::Monospace),
        "does not have the `Monospace` family"
    );
    assert!(
        fonts.families.contains_key(&egui::FontFamily::Proportional),
        "does not have the `Proportional` family"
    );
    assert!(
        fonts
            .families
            .contains_key(&egui::FontFamily::Name(FONT_NAME.into())),
        "does not have the `Nacelle` family"
    );
    assert_eq!(
        fonts
            .families
            .get(&egui::FontFamily::Name(FONT_NAME.into()))
            .unwrap(),
        &vec![FONT_NAME.to_owned()],
        "`Nacelle` family does not have the correct data"
    );
}

#[test]
fn test_custom_font_serialization() {
    const FONT_NAME: &str = "Nacelle";
    let mut font_definitions = egui::FontDefinitions::default();
    font_definitions.font_data.insert(
        FONT_NAME.to_owned(),
        FontData::from_static(include_bytes!("test-fonts/Nacelle-Regular.otf")),
    );
    font_definitions.families.insert(
        egui::FontFamily::Name(FONT_NAME.into()),
        vec![FONT_NAME.to_owned()],
    );

    let theme = EguiTheme::new(egui::Style::default(), font_definitions);

    let serialized = serde_json::to_string(&theme).expect("serialization failed");

    assert!(
        serialized.contains("Nacelle"),
        "Nacelle not found in the serialized string"
    );
    let nacelle_font_data = serde_json::to_value(FontData::from_static(include_bytes!(
        "test-fonts/Nacelle-Regular.otf"
    )))
    .unwrap();
    assert!(
        serialized.contains(&nacelle_font_data.to_string()),
        "Nacelle data does not exist in the serialized string"
    );
}

#[test]
fn test_text_style() {
    let mut style = Style::default();
    let mut fonts = FontDefinitions::default();

    const FONT_NAME: &str = "NacelleFontData";
    fonts.font_data.insert(
        FONT_NAME.to_owned(),
        FontData::from_static(include_bytes!("test-fonts/Nacelle-Regular.otf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name(FONT_NAME.into()),
        vec![FONT_NAME.to_owned()],
    );

    fonts.families.insert(
        FontFamily::Name("NacelleFontFamily".into()),
        vec![FONT_NAME.to_owned()],
    );
    style.text_styles.insert(
        TextStyle::Name("NacelleStyle".into()),
        FontId::new(12.0, FontFamily::Name("NacelleFontFamily".into())),
    );

    let theme = EguiTheme::new(style, fonts);
    let serialized = serde_json::to_string(&theme).expect("serialization failed");
    assert!(
        serialized.contains("NacelleFontData"),
        "Nacelle Font Data was not serialized correctly"
    );
    assert!(
        serialized.contains("NacelleFontFamily"),
        "FontFamily was not serialized correctly"
    );
    assert!(
        serialized.contains("NacelleStyle"),
        "TextStyle was not serialized correctly"
    );

    let deserialized =
        serde_json::from_str::<EguiTheme>(serialized.as_str()).expect("deserialization failed");
    let (de_style, _fonts) = deserialized.extract();
    assert!(
        de_style.text_styles().contains(&TextStyle::Body),
        "text style `Body` does not exist"
    );
    assert!(
        de_style.text_styles().contains(&TextStyle::Small),
        "text style `Small` does not exist"
    );
    assert!(
        de_style.text_styles().contains(&TextStyle::Monospace),
        "text style `Monospace` does not exist"
    );
    assert!(
        de_style.text_styles().contains(&TextStyle::Button),
        "text style `Button` does not exist"
    );
    assert!(
        de_style.text_styles().contains(&TextStyle::Heading),
        "text style `Heading` does not exist"
    );
    assert!(
        de_style
            .text_styles()
            .contains(&TextStyle::Name("NacelleStyle".into())),
        "text style `NacelleStyle` does not exist"
    );
    assert!(
        de_style
            .text_styles
            .get(&TextStyle::Name("NacelleStyle".into()))
            .is_some(),
        "could not get the text_style"
    );
    assert_eq!(
        *de_style
            .text_styles
            .get(&TextStyle::Name("NacelleStyle".into()))
            .unwrap(),
        FontId::new(12.0, FontFamily::Name("NacelleFontFamily".into())),
        "FontStyle not deserialized"
    );
}
#[test]
fn test_colors() {
    let mut style = Style::default();
    let fg_stroke = egui::Stroke::new(1f32, egui::Color32::TRANSPARENT);
    style.visuals.widgets.noninteractive.fg_stroke = fg_stroke.clone();
    style.visuals.widgets.inactive.bg_fill = egui::Color32::LIGHT_RED;

    let theme = EguiTheme::new(style, FontDefinitions::default());
    let serialized = serde_json::to_string(&theme).expect("serialization failed");
    let deserialized =
        serde_json::from_str::<EguiTheme>(serialized.as_str()).expect("deserialization failed");
    let (de_style, _fonts) = deserialized.extract();

    assert_eq!(
        de_style.visuals.widgets.noninteractive.fg_stroke,
        fg_stroke.clone(),
        "stroke doesn't match"
    );
    assert_eq!(
        de_style.visuals.widgets.inactive.bg_fill,
        egui::Color32::LIGHT_RED,
        "Color doesn't match"
    );
}

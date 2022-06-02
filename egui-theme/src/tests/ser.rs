use egui::FontData;

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

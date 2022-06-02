use crate::EguiTheme;
mod de;
mod ser;
#[test]
fn test_default_theme() {
    let theme = EguiTheme::new(egui::Style::default(), egui::FontDefinitions::default());
    assert_eq!(
        theme.egui_theme_version,
        crate::EGUI_THEME_VERSION,
        "egui_theme version must match"
    );
    assert_eq!(
        theme.egui_version,
        crate::EGUI_VERSION,
        "egui_Version must match"
    );

    assert!(
        theme.fonts.contains_key("font_data"),
        "the font_data key should exist"
    );
    assert!(
        theme.fonts.contains_key("families"),
        "the families key should exist"
    );

    assert!(theme.style.contains_key("wrap"), "`wrap` key should exist");
    assert!(
        theme.style.contains_key("explanation_tooltips"),
        "`explanation_tooltips` key should exist"
    );
    assert!(
        theme.style.contains_key("override_text_style"),
        "`override_text_style` key should exist"
    );
    assert!(
        theme.style.contains_key("animation_time"),
        "`animation_time` key should exist"
    );
    assert!(
        theme.style.contains_key("explanation_tooltips"),
        "`explanation_tooltips` key should exist"
    );
    assert!(
        theme.style.contains_key("text_styles"),
        "`text_styles` key should exist"
    );
    assert!(
        theme.style.contains_key("visuals.dark_mode"),
        "`visuals.dark_mode` key should exist"
    );
    assert!(
        theme.style.contains_key("visuals.selection"),
        "`visuals.selection` key should exist"
    );
    assert!(
        theme.style.contains_key("spacing.combo_height"),
        "`spacing.combo_height` key should exist"
    );
    assert!(
        theme
            .style
            .contains_key("interaction.show_tooltips_only_when_still"),
        "`interaction.show_tooltips_only_when_still` key should exist"
    );
}

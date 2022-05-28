//! This is modified from the widget gallery code available at the [egui repository](https://github.com/emilk/egui/blob/master/egui_demo_lib/src/apps/demo/widget_gallery.rs)
use egui::{TextStyle, Widget, RichText, Label, Ui};

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum Enum {
    First,
    Second,
    Third,
}

/// Shows off one example of each major type of widget.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WidgetGallery {
    enabled: bool,
    visible: bool,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,
}

impl Default for WidgetGallery {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            string: Default::default(),
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: false,
        }
    }
}

impl WidgetGallery {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.set_visible(self.visible);
            ui.set_enabled(self.enabled);

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.visible, "Visible")
                .on_hover_text("Uncheck to hide all the widgets.");
            if self.visible {
                ui.checkbox(&mut self.enabled, "Interactive")
                    .on_hover_text("Uncheck to inspect how the widgets look when disabled.");
            }
        });
    }

    // TODO: Add this back in
    fn font_selector(ui: &mut egui::Ui) {            
        // let mut proportional_fonts = ui
        //     .ctx()
        //     .fonts()
        //     .definitions()
        //     .fonts_for_family
        //     .get(&egui::FontFamily::Proportional)
        //     .expect("this should exist")
        //     .clone();
        // let mut current_font = 0;
        // ui.add(egui::Label::new("Proportional Font"));
        // egui::ComboBox::from_id_source("_proportional_select")
        //     .selected_text(&proportional_fonts[0])
        //     .show_ui(ui, |ui| {
        //         for (i, name) in proportional_fonts.iter().enumerate() {
        //             ui.selectable_value(&mut current_font, i, name);
        //         }
        //     });

        // if current_font > 0 {
        //     let font = proportional_fonts.remove(current_font);
        //     proportional_fonts.insert(0, font);
        //     let mut fonts = ui.ctx().fonts().definitions().clone();
        //     fonts
        //         .fonts_for_family
        //         .insert(egui::FontFamily::Proportional, proportional_fonts);
        //     ui.ctx().set_fonts(fonts);
        // }
        // ui.end_row();

        // let mut monospace_fonts = ui
        //     .ctx()
        //     .fonts()
        //     .definitions()
        //     .fonts_for_family
        //     .get(&egui::FontFamily::Monospace)
        //     .expect("this should exist")
        //     .clone();
        // let mut current_font = 0;
        // ui.add(egui::Label::new("Monospace Font"));
        // egui::ComboBox::from_id_source("_monospace_select")
        //     .selected_text(&monospace_fonts[0])
        //     .show_ui(ui, |ui| {
        //         for (i, name) in monospace_fonts.iter().enumerate() {
        //             ui.selectable_value(&mut current_font, i, name);
        //         }
        //     });
        // if current_font > 0 {
        //     let font = monospace_fonts.remove(current_font);
        //     monospace_fonts.insert(0, font);
        //     let mut fonts = ui.ctx().fonts().definitions().clone();
        //     fonts
        //         .fonts_for_family
        //         .insert(egui::FontFamily::Monospace, monospace_fonts);
        //     ui.ctx().set_fonts(fonts);
        // }
        // ui.end_row();
    }
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            enabled: _,
            visible: _,
            boolean,
            radio,
            scalar,
            string,
            color,
            animate_progress_bar,
        } = self;

        ui.label("Label");
        ui.label("Welcome to the widget gallery!");
        ui.end_row();
        WidgetGallery::font_selector(ui);
        Label::new("Monospace Label").ui(ui);
        Label::new(
            RichText::new("This is using Monospace TextStyle").monospace()
        ).ui(ui);
        ui.end_row();

        Label::new("Small Label").ui(ui);
        Label::new(RichText::new("This is using the Small TextStyle").small()).ui(ui);
        ui.end_row();

        Label::new("Body Label").ui(ui);
        Label::new(RichText::new("This is using the Body TextStyle").text_style(TextStyle::Body)).ui(ui);
        ui.end_row();

        Label::new("Heading Label").ui(ui);
        Label::new(RichText::new("This is using the Heading TextStyle").heading()).ui(ui);
        ui.end_row();

        Label::new("Button Label").ui(ui);
        Label::new(
            RichText::new("This is using the Button TextStyle").text_style(TextStyle::Button)
        ).ui(ui);
        ui.end_row();

        ui.label("Hyperlink");
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
            format!("{} egui home page", GITHUB),
            "https://github.com/emilk/egui",
        );
        ui.end_row();

        ui.label("TextEdit");
        ui.add(egui::TextEdit::singleline(string).hint_text("Write something here"));
        ui.end_row();

        ui.label("Button");
        if ui.button("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.label("Checkbox");
        ui.checkbox(boolean, "Checkbox");
        ui.end_row();

        ui.label("RadioButton");
        ui.horizontal(|ui| {
            ui.radio_value(radio, Enum::First, "First");
            ui.radio_value(radio, Enum::Second, "Second");
            ui.radio_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.label("SelectableLabel");
        ui.horizontal(|ui| {
            ui.selectable_value(radio, Enum::First, "First");
            ui.selectable_value(radio, Enum::Second, "Second");
            ui.selectable_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.label("ComboBox");

        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{:?}", radio))
            .show_ui(ui, |ui| {
                ui.selectable_value(radio, Enum::First, "First");
                ui.selectable_value(radio, Enum::Second, "Second");
                ui.selectable_value(radio, Enum::Third, "Third");
            });
        ui.end_row();

        ui.label("Slider");
        ui.add(egui::Slider::new(scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.label("DragValue");
        ui.add(egui::DragValue::new(scalar).speed(1.0));
        ui.end_row();

        ui.label("ProgressBar");
        let progress = *scalar / 360.0;
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(*animate_progress_bar);
        *animate_progress_bar = ui
            .add(progress_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.label("Color");
        ui.color_edit_button_srgba(color);
        ui.end_row();

        ui.label("Image");
        ui.image(egui::TextureId::Managed(0), [24.0, 16.0])
            .on_hover_text("The egui font texture was the most convenient choice to show here.");
        ui.end_row();

        ui.label("ImageButton");
        if ui
            .add(egui::ImageButton::new(egui::TextureId::Managed(0), [24.0, 16.0]))
            .on_hover_text("The egui font texture was the most convenient choice to show here.")
            .clicked()
        {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.label("Separator");
        ui.separator();
        ui.end_row();

        ui.label("CollapsingHeader");
        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    "Not much, as it turns out - but here is a gold star for you for checking:",
                );
                ui.colored_label(egui::Color32::GOLD, "☆");
            });
        });
        ui.end_row();

        ui.label("Plot");
        example_plot(ui);
        ui.end_row();
    }
}

fn example_plot(ui: &mut egui::Ui) -> egui::Response {
    use egui::plot::{Line, Plot, Value, Values};
    let n = 128;
    let line = Line::new(Values::from_values_iter((0..=n).map(|i| {
        use std::f64::consts::TAU;
        let x = egui::remap(i as f64, 0.0..=(n as f64), -TAU..=TAU);
        Value::new(x, x.sin())
    })));
    Plot::new("example_plot")
        .height(32.0)
        .data_aspect(1.0).show(ui, |plot_ui| {plot_ui.line(line)})
        .response
}

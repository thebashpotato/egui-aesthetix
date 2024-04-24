//! Tokyo Night Storm theme.
//! A port of the popular visual studio code theme: `https://github.com/enkia/tokyo-night-vscode-theme/blob/master/themes/tokyo-night-storm-color-theme.json`

use crate::Aesthetix;

/// Tokyo Night Storm.
pub struct TokyoNightStorm;

impl Aesthetix for TokyoNightStorm {
    fn name(&self) -> &str {
        "Tokyo Night Storm"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(138, 171, 244, 255)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(97, 175, 239, 255)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(23, 24, 38, 255)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(31, 31, 51, 255)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(33, 35, 53, 255)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(27, 29, 45, 255)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(42, 42, 68, 255)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgba_premultiplied(204, 204, 204, 255))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(86, 209, 123, 255)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(255, 161, 90, 255)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(255, 121, 121, 255)
    }

    fn dark_mode_visuals(&self) -> bool {
        true
    }

    fn margin_style(&self) -> f32 {
        12.0
    }

    fn button_padding(&self) -> egui::Vec2 {
        egui::Vec2 { x: 12.0, y: 10.0 }
    }

    fn item_spacing_style(&self) -> f32 {
        18.0
    }

    fn scroll_bar_width_style(&self) -> f32 {
        14.0
    }

    fn rounding_visuals(&self) -> f32 {
        6.0
    }
}

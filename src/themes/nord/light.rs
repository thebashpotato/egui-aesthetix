//! Nord Light theme
//! <https://www.nordtheme.com/>

use crate::Aesthetix;

/// Light theme, with rounded buttons, and ample margin. Adapted from the Nord light
/// color scheme.
pub struct NordLight;

impl Aesthetix for NordLight {
    fn name(&self) -> &str {
        "Nord Light"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(104, 161, 210)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(129, 161, 193)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(216, 222, 233)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(229, 233, 240)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 255, 255)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(206, 212, 224)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(180, 186, 189)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::BLACK)
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(82, 163, 137)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 179, 71)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(228, 97, 107)
    }

    fn dark_mode_visuals(&self) -> bool {
        false
    }

    fn margin_style(&self) -> i8 {
        12
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

    fn rounding_visuals(&self) -> u8 {
        6
    }
}

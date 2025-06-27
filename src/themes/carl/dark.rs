//! A port of the Carl dark theme from Kde plasma.
//! <https://store.kde.org/p/1338881/>

use crate::Aesthetix;

/// A very dark theme with blueish accents
pub struct CarlDark;

impl Aesthetix for CarlDark {
    fn name(&self) -> &str {
        "Carl Dark"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(135, 169, 241)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(56, 114, 238)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(12, 12, 15)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(17, 18, 22)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(25, 27, 33)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(72, 72, 72)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(109, 109, 109)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(207, 216, 220))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(42, 172, 170)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(191, 54, 198)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 55, 102)
    }

    fn dark_mode_visuals(&self) -> bool {
        true
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

//! Gnome Adwaita light theme (roughly)
//! <https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/named-colors.html/>

use crate::Aesthetix;

/// Standard light theme, with rounded buttons, ample margin adapted from the Gnome light color
/// scheme.
pub struct StandardLight;

impl Aesthetix for StandardLight {
    fn name(&self) -> &str {
        "Standard Light"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(98, 160, 234)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(53, 132, 228)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 255, 255)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(246, 246, 246)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(222, 221, 221)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(192, 191, 188)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(154, 153, 150)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(16, 16, 16))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(46, 194, 126)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(229, 165, 10)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(224, 27, 36)
    }

    fn dark_mode_visuals(&self) -> bool {
        false
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

//! Gnome Adwaita Dark theme (roughly)
//! <https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/named-colors.html/>

use crate::Aesthetix;

/// A Standard dark theme, with rounded buttons, and ample margin. Adapted from the Gnome dark
/// color scheme.
pub struct StandardDark;

impl Aesthetix for StandardDark {
    fn name(&self) -> &str {
        "Standard Dark"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(98, 160, 234)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(53, 132, 228)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(30, 30, 30)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(48, 48, 48)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(40, 40, 40)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(72, 72, 72)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(109, 109, 109)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(255, 255, 255))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(38, 162, 105)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(205, 147, 9)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(192, 28, 40)
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

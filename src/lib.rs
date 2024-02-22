//! A programmatic and Uniform approach to theming egui Applications.
//!
//! This library supplies one simple theme trait which attempts to expose all the key colors, margins, paddings, spacings
//! and other style elements that one would need to manipulate to implement a theme.
//!
//! It also gives defaults for more niche elements of the style that a user might not want to customize
//! but can if they want to.

#[cfg(feature = "default")]
pub mod themes;

use egui::style::ScrollStyle;

/// Every custom egui theme that wishes to use the egui aesthetix crate must implement this trait.
/// Aesthetix is structured in such a way that it is easy to customize the theme to your liking.
///
/// The trait is split into two parts:
/// - The first part are the methods that have no implementation, these should just return self-explanatory values.
///
/// - The second part are the methods that have a default implementation, they are more complex and use all the user defined methods.
///   the fields in these traits that don't use trait methods as values are niche and can be ignored if you don't want to customize them.
///   If the user really wants to customize these fields, they can override the method easily enough, just copy the method you wish to override
///   and do so. All of eguis style fields can be found here.
pub trait Aesthetix {
    /// The name of the theme for debugging and comparison purposes.
    fn name(&self) -> &str;

    /// The primary accent color of the theme.
    fn primary_accent_color_visuals(&self) -> egui::Color32;

    /// The secondary accent color of the theme.
    fn secondary_accent_color_visuals(&self) -> egui::Color32;

    /// Used for the main background color of the app.
    ///
    /// - This value is used for eguis `panel_fill` and `window_fill` fields
    fn bg_primary_color_visuals(&self) -> egui::Color32;

    /// Something just barely different from the background color.
    ///
    /// - This value is used for eguis `faint_bg_color` field
    fn bg_secondary_color_visuals(&self) -> egui::Color32;

    /// Very dark or light color (for corresponding theme). Used as the background of text edits,
    /// scroll bars and others things that needs to look different from other interactive stuff.
    ///
    /// - This value is used for eguis `extreme_bg_color` field
    fn bg_triage_color_visuals(&self) -> egui::Color32;

    /// Background color behind code-styled monospaced labels.
    /// Back up lighter than the background primary, secondary and triage colors.
    ///
    /// - This value is used for eguis `code_bg_color` field
    fn bg_auxiliary_color_visuals(&self) -> egui::Color32;

    /// The color for hyperlinks, and border contrasts.
    fn bg_contrast_color_visuals(&self) -> egui::Color32;

    /// This is great for setting the color of text for any widget.
    ///
    /// If text color is None (default), then the text color will be the same as the foreground stroke color
    /// and will depend on whether the widget is being interacted with.
    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32>;

    /// Success color for text.
    fn fg_success_text_color_visuals(&self) -> egui::Color32;

    /// Warning text color.
    fn fg_warn_text_color_visuals(&self) -> egui::Color32;

    /// Error text color.
    fn fg_error_text_color_visuals(&self) -> egui::Color32;

    /// Visual dark mode.
    /// True specifies a dark mode, false specifies a light mode.
    fn dark_mode_visuals(&self) -> bool;

    /// Horizontal and vertical margins within a menu frame.
    /// This value is used for all margins, in windows, panes, frames etc.
    /// Using the same value will yield a more consistent look.
    ///
    /// - Egui default is 6.0
    fn margin_style(&self) -> f32;

    /// Button size is text size plus this on each side.
    ///
    /// - Egui default is { x: 6.0, y: 4.0 }
    fn button_padding(&self) -> egui::Vec2;

    /// Horizontal and vertical spacing between widgets.
    /// If you want to override this for special cases use the `add_space` method.
    /// This single value is added for the x and y coordinates to yield a more consistent look.
    ///
    /// - Egui default is 4.0
    fn item_spacing_style(&self) -> f32;

    /// Scroll bar width.
    ///
    /// - Egui default is 6.0
    fn scroll_bar_width_style(&self) -> f32;

    /// Custom rounding value for all buttons and frames.
    ///
    /// - Egui default is 4.0
    fn rounding_visuals(&self) -> f32;

    /// Controls the sizes and distances between widgets.
    /// The following types of spacing are implemented.
    ///
    /// - Spacing
    /// - Margin
    /// - Button Padding
    /// - Scroll Bar width
    fn spacing_style(&self) -> egui::style::Spacing {
        egui::style::Spacing {
            item_spacing: egui::Vec2 {
                x: self.item_spacing_style(),
                y: self.item_spacing_style(),
            },
            window_margin: egui::Margin {
                left: self.margin_style(),
                right: self.margin_style(),
                top: self.margin_style(),
                bottom: self.margin_style(),
            },
            button_padding: self.button_padding(),
            menu_margin: egui::Margin {
                left: self.margin_style(),
                right: self.margin_style(),
                top: self.margin_style(),
                bottom: self.margin_style(),
            },
            indent: 18.0,
            interact_size: egui::Vec2 { x: 40.0, y: 20.0 },
            slider_width: 100.0,
            combo_width: 100.0,
            text_edit_width: 280.0,
            icon_width: 14.0,
            icon_width_inner: 8.0,
            icon_spacing: 6.0,
            tooltip_width: 600.0,
            indent_ends_with_horizontal_line: false,
            combo_height: 200.0,
            scroll: ScrollStyle {
                bar_width: self.scroll_bar_width_style(),
                handle_min_length: 12.0,
                bar_inner_margin: 4.0,
                bar_outer_margin: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// How and when interaction happens.
    fn interaction_style(&self) -> egui::style::Interaction {
        egui::style::Interaction {
            resize_grab_radius_side: 5.0,
            resize_grab_radius_corner: 10.0,
            show_tooltips_only_when_still: true,
            ..Default::default()
        }
    }

    /// The style of a widget that you cannot interact with.
    ///
    /// `noninteractive.bg_stroke` is the outline of windows.
    /// `noninteractive.bg_fill` is the background color of windows.
    /// `noninteractive.fg_stroke` is the normal text color.
    fn custom_noninteractive_widget_visuals(&self) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: self.bg_auxiliary_color_visuals(),
            weak_bg_fill: self.bg_auxiliary_color_visuals(),
            bg_stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_auxiliary_color_visuals(),
            },
            rounding: egui::Rounding {
                nw: self.rounding_visuals(),
                ne: self.rounding_visuals(),
                sw: self.rounding_visuals(),
                se: self.rounding_visuals(),
            },
            fg_stroke: egui::Stroke {
                width: 1.0,
                color: self.fg_primary_text_color_visuals().unwrap_or_default(),
            },
            expansion: 0.0,
        }
    }

    /// The style of an interactive widget, such as a button, at rest.
    fn widget_inactive_visual(&self) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: self.bg_auxiliary_color_visuals(),
            weak_bg_fill: self.bg_auxiliary_color_visuals(),
            bg_stroke: egui::Stroke {
                width: 0.0,
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
            },
            rounding: egui::Rounding {
                nw: self.rounding_visuals(),
                ne: self.rounding_visuals(),
                sw: self.rounding_visuals(),
                se: self.rounding_visuals(),
            },
            fg_stroke: egui::Stroke {
                width: 1.0,
                color: self.fg_primary_text_color_visuals().unwrap_or_default(),
            },
            expansion: 0.0,
        }
    }

    /// The style of an interactive widget while you hover it, or when it is highlighted
    fn widget_hovered_visual(&self) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: self.bg_auxiliary_color_visuals(),
            weak_bg_fill: self.bg_auxiliary_color_visuals(),
            bg_stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_triage_color_visuals(),
            },
            rounding: egui::Rounding {
                nw: self.rounding_visuals(),
                ne: self.rounding_visuals(),
                sw: self.rounding_visuals(),
                se: self.rounding_visuals(),
            },
            fg_stroke: egui::Stroke {
                width: 1.5,
                color: self.fg_primary_text_color_visuals().unwrap_or_default(),
            },
            expansion: 2.0,
        }
    }

    /// The style of an interactive widget as you are clicking or dragging it.
    fn custom_active_widget_visual(&self) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: self.bg_primary_color_visuals(),
            weak_bg_fill: self.primary_accent_color_visuals(),
            bg_stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_primary_color_visuals(),
            },
            rounding: egui::Rounding {
                nw: self.rounding_visuals(),
                ne: self.rounding_visuals(),
                sw: self.rounding_visuals(),
                se: self.rounding_visuals(),
            },
            fg_stroke: egui::Stroke {
                width: 2.0,
                color: self.fg_primary_text_color_visuals().unwrap_or_default(),
            },
            expansion: 1.0,
        }
    }

    /// The style of a button that has an open menu beneath it (e.g. a combo-box)
    fn custom_open_widget_visual(&self) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: self.bg_secondary_color_visuals(),
            weak_bg_fill: self.bg_secondary_color_visuals(),
            bg_stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_triage_color_visuals(),
            },
            rounding: egui::Rounding {
                nw: self.rounding_visuals(),
                ne: self.rounding_visuals(),
                sw: self.rounding_visuals(),
                se: self.rounding_visuals(),
            },
            fg_stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_contrast_color_visuals(),
            },
            expansion: 0.0,
        }
    }

    /// Uses the primary and secondary accent colors to build a custom selection style.
    fn custom_selection_visual(&self) -> egui::style::Selection {
        egui::style::Selection {
            bg_fill: self.primary_accent_color_visuals(),
            stroke: egui::Stroke {
                width: 1.0,
                color: self.bg_primary_color_visuals(),
            },
        }
    }

    /// Edit text styles.
    /// This is literally just a copy and pasted version of eguis `default_text_styles` function.
    fn custom_text_styles(&self) -> std::collections::BTreeMap<egui::TextStyle, egui::FontId> {
        use egui::FontFamily::{Monospace, Proportional};
        [
            (
                egui::TextStyle::Small,
                egui::FontId::new(10.0, Proportional),
            ),
            (egui::TextStyle::Body, egui::FontId::new(14.0, Proportional)),
            (
                egui::TextStyle::Button,
                egui::FontId::new(14.00, Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(18.0, Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(12.0, Monospace),
            ),
        ]
        .into()
    }

    /// Sets the custom style for the given original [`Style`](egui::Style).
    /// Relies on all above trait methods to build the complete style.
    ///
    /// Specifies the look and feel of egui.
    fn custom_style(&self) -> egui::Style {
        egui::style::Style {
            // override the text styles here: Option<egui::TextStyle>
            override_text_style: None,

            // override the font id here: Option<egui::FontId>
            override_font_id: None,

            // set your text styles here:
            text_styles: self.custom_text_styles(),

            // set your drag value text style:
            // drag_value_text_style: egui::TextStyle,
            spacing: self.spacing_style(),
            interaction: self.interaction_style(),

            visuals: egui::Visuals {
                dark_mode: self.dark_mode_visuals(),
                //override_text_color: self.fg_primary_text_color_visuals(),
                widgets: egui::style::Widgets {
                    noninteractive: self.custom_noninteractive_widget_visuals(),
                    inactive: self.widget_inactive_visual(),
                    hovered: self.widget_hovered_visual(),
                    active: self.custom_active_widget_visual(),
                    open: self.custom_open_widget_visual(),
                },
                selection: self.custom_selection_visual(),
                hyperlink_color: self.bg_contrast_color_visuals(),
                panel_fill: self.bg_primary_color_visuals(),
                faint_bg_color: self.bg_secondary_color_visuals(),
                extreme_bg_color: self.bg_triage_color_visuals(),
                code_bg_color: self.bg_auxiliary_color_visuals(),
                warn_fg_color: self.fg_warn_text_color_visuals(),
                error_fg_color: self.fg_error_text_color_visuals(),
                window_rounding: egui::Rounding {
                    nw: self.rounding_visuals(),
                    ne: self.rounding_visuals(),
                    sw: self.rounding_visuals(),
                    se: self.rounding_visuals(),
                },
                window_shadow: egui::epaint::Shadow {
                    extrusion: 32.0,
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 96),
                },
                window_fill: self.bg_primary_color_visuals(),
                window_stroke: egui::Stroke {
                    width: 1.0,
                    color: self.bg_contrast_color_visuals(),
                },
                menu_rounding: egui::Rounding {
                    nw: self.rounding_visuals(),
                    ne: self.rounding_visuals(),
                    sw: self.rounding_visuals(),
                    se: self.rounding_visuals(),
                },
                popup_shadow: egui::epaint::Shadow {
                    extrusion: 16.0,
                    color: egui::Color32::from_rgba_premultiplied(19, 18, 18, 96),
                },
                resize_corner_size: 12.0,
                text_cursor_preview: false,
                clip_rect_margin: 3.0,
                button_frame: true,
                collapsing_header_frame: true,
                indent_has_left_vline: true,
                striped: true,
                slider_trailing_fill: true,
                ..Default::default()
            },
            animation_time: 0.083_333_336,
            explanation_tooltips: true,
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for dyn Aesthetix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for dyn Aesthetix {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

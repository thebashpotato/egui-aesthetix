//! Nord Dark and Light themes
//!
//! The [`egui_aesthetix::themes::NordDark`] and [`egui_aesthetix::themes::NordLight`]
//! structs are good examples on how to implement a theme you want using the
//! [`egui_aesthetix::Aesthetix`] trait.

mod dark;
mod light;

pub use dark::NordDark;
pub use light::NordLight;

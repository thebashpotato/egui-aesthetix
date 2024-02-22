//! All themes that the Aesthetix ships without of the box will
//! be located in this folder

mod carl;
mod standard;

#[cfg(feature = "carl")]
pub use carl::CarlDark;

#[cfg(feature = "standard")]
pub use standard::{StandardDark, StandardLight};

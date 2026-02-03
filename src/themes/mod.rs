//! All themes that the Aesthetix ships with out of the box
//! be located in this folder

#[cfg(feature = "carl")]
mod carl;
#[cfg(feature = "carl")]
pub use carl::CarlDark;

#[cfg(feature = "nord")]
mod nord;
#[cfg(feature = "nord")]
pub use nord::{NordDark, NordLight};

#[cfg(feature = "standard")]
mod standard;
#[cfg(feature = "standard")]
pub use standard::{StandardDark, StandardLight};

#[cfg(feature = "tokyo_night")]
mod tokyo_night;
#[cfg(feature = "tokyo_night")]
pub use tokyo_night::{TokyoNight, TokyoNightStorm};

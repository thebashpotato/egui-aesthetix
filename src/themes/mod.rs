//! All themes that the Aesthetix ships without of the box will
//! be located in this folder

mod carl;
mod nord;
mod standard;
mod tokyo_night;

#[cfg(feature = "carl")]
pub use carl::dark::CarlDark;

#[cfg(feature = "nord")]
pub use nord::dark::NordDark;
pub use nord::light::NordLight;

#[cfg(feature = "tokyo_night")]
pub use tokyo_night::dark::TokyoNight;
pub use tokyo_night::storm::TokyoNightStorm;

#[cfg(feature = "standard")]
pub use standard::dark::StandardDark;
pub use standard::light::StandardLight;

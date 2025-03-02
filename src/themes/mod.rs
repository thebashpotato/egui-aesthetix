//! All themes that the Aesthetix ships out of the box will
//! be located in this folder

#[cfg(feature = "carl")]
mod carl;
#[cfg(feature = "carl")]
pub use carl::*;

#[cfg(feature = "nord")]
mod nord;
#[cfg(feature = "nord")]
pub use nord::*;

#[cfg(feature = "tokyo_night")]
mod tokyo_night;
#[cfg(feature = "tokyo_night")]
pub use tokyo_night::*;

#[cfg(feature = "standard")]
mod standard;
#[cfg(feature = "standard")]
pub use standard::*;

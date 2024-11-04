//! All themes that the Aesthetix ships with out of the box
//! be located in this folder

macro_rules! conditional_mod_use {
    ($feature:literal, $mod_name:ident, $($item:ident),*) => {
        #[cfg(feature = $feature)]
        mod $mod_name;

        #[cfg(feature = $feature)]
        pub use $mod_name::{ $($item),* };
    };
}

// Apply the macro for each module with their corresponding items
conditional_mod_use!("carl", carl, CarlDark);
conditional_mod_use!("nord", nord, NordDark, NordLight);
conditional_mod_use!("standard", standard, StandardDark, StandardLight);
conditional_mod_use!("tokyo_night", tokyo_night, TokyoNight, TokyoNightStorm);

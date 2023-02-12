//! # Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondrayColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondray colors according to the OGP color model.
    pub enum SecondrayColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary tolor.
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondrayColor {
        todo!();
    }
}

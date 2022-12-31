//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combine two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Red => SecondaryColor::Orange,
                PrimaryColor::Blue => SecondaryColor::Purple,
                PrimaryColor::Yellow => SecondaryColor::Orange,
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => SecondaryColor::Purple,
                PrimaryColor::Blue => SecondaryColor::Purple,
                PrimaryColor::Yellow => SecondaryColor::Green,
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Red => SecondaryColor::Orange,
                PrimaryColor::Blue => SecondaryColor::Green,
                PrimaryColor::Yellow => SecondaryColor::Orange,
            },
        }
    }
}

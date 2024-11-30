use bevy::{prelude::*, utils::HashMap};
use bitflags::bitflags;

bitflags! {
    /// Represents the different colors of mana.
    ///
    /// Each color is represented as a bitflag, allowing for combinations of colors.
    /// The available colors are:
    /// - `WHITE`
    /// - `BLUE`
    /// - `BLACK`
    /// - `RED`
    /// - `GREEN`
    /// - `COLORLESS`
    /// - `ALL` (combination of all colors)
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Color: u8 {
        const COLORLESS = 0b00000;
        const WHITE = 0b00001;
        const BLUE = 0b00010;
        const BLACK = 0b00100;
        const RED = 0b01000;
        const GREEN = 0b10000;
        const ALL = Self::WHITE.bits() & Self::BLUE.bits() & Self::BLACK.bits() & Self::RED.bits() & Self::GREEN.bits();
    }
}

/// Represents a quantity of mana of a specific color.
///
/// The `Mana` struct contains:
/// - `color`: The color of the mana, represented by the `Color` bitflags.
/// - `amount`: The amount of mana of the specified color.
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Mana {
    color: Color,
    amount: u64,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManaPool {
    mana: HashMap<Color, Mana>,
}

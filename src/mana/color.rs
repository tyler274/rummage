use bevy::prelude::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// ManaColor represents the five colors of Magic and colorless as bit flags, allowing combinations
    /// of colors to be represented efficiently.
    ///
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ManaColor: u32 {
        const NONE = 0;
        const WHITE = 1 << 0;
        const BLUE = 1 << 1;
        const BLACK = 1 << 2;
        const RED = 1 << 3;
        const GREEN = 1 << 4;
        const COLORLESS = 1 << 5;
    }
}

/// Wrapper around ManaColor for reflection support
#[derive(
    Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect,
)]
#[reflect(Serialize, Deserialize)]
pub struct ReflectableColor {
    bits: u32,
}

impl ReflectableColor {
    pub fn new(color: ManaColor) -> Self {
        Self { bits: color.bits() }
    }

    pub fn color(&self) -> ManaColor {
        ManaColor::from_bits_truncate(self.bits)
    }
}

impl From<ManaColor> for ReflectableColor {
    fn from(color: ManaColor) -> Self {
        Self::new(color)
    }
}

impl From<ReflectableColor> for ManaColor {
    fn from(reflectable: ReflectableColor) -> Self {
        reflectable.color()
    }
}

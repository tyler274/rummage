use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::color::*;
use super::pool::ManaPool;

/// Represents mana costs with specific amounts for each color.
///
/// This struct tracks both the colors present in the mana cost and
/// the specific amounts of each type of mana required.
///
#[derive(
    Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect,
)]
#[reflect(Serialize, Deserialize)]
pub struct Mana {
    /// The colors present in this mana cost
    #[reflect(ignore)]
    pub color: ManaColor,
    /// Reflectable version of the color
    pub reflectable_color: ReflectableColor,
    /// Amount of white mana
    pub white: u64,
    /// Amount of blue mana
    pub blue: u64,
    /// Amount of black mana
    pub black: u64,
    /// Amount of red mana
    pub red: u64,
    /// Amount of green mana
    pub green: u64,
    /// Amount of colorless mana
    pub colorless: u64,
}

impl Mana {
    /// Creates a new empty mana cost.
    ///
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the converted mana cost (total mana of all colors and colorless)
    #[allow(dead_code)]
    pub fn converted_mana_cost(&self) -> u64 {
        self.total()
    }

    /// Returns the amount of mana of a specific color
    #[allow(dead_code)]
    pub fn colored_mana_cost(&self, color: ManaColor) -> u64 {
        match color {
            ManaColor::WHITE => self.white,
            ManaColor::BLUE => self.blue,
            ManaColor::BLACK => self.black,
            ManaColor::RED => self.red,
            ManaColor::GREEN => self.green,
            ManaColor::COLORLESS => self.colorless,
            _ => 0,
        }
    }

    /// Returns the total amount of mana in this cost.
    ///
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green + self.colorless
    }

    /// Returns true if this mana cost is empty (has no mana of any color or colorless).
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Returns true if this mana cost contains any colored mana.
    #[allow(dead_code)]
    pub fn has_color(&self) -> bool {
        self.white > 0 || self.blue > 0 || self.black > 0 || self.red > 0 || self.green > 0
    }

    /// Returns true if this mana cost can be paid with the given mana pool.
    pub fn can_pay(&self, pool: &ManaPool) -> bool {
        // Check if the mana cost is empty
        if self.is_empty() {
            return true;
        }

        // Check if we can pay each colored mana cost
        if self.white > 0 && pool.mana.values().map(|m| m.white).sum::<u64>() < self.white {
            return false;
        }
        if self.blue > 0 && pool.mana.values().map(|m| m.blue).sum::<u64>() < self.blue {
            return false;
        }
        if self.black > 0 && pool.mana.values().map(|m| m.black).sum::<u64>() < self.black {
            return false;
        }
        if self.red > 0 && pool.mana.values().map(|m| m.red).sum::<u64>() < self.red {
            return false;
        }
        if self.green > 0 && pool.mana.values().map(|m| m.green).sum::<u64>() < self.green {
            return false;
        }
        if self.colorless > 0 {
            let total_available = pool.mana.values().map(|m| m.total()).sum::<u64>();
            let colored_needed = self.white + self.blue + self.black + self.red + self.green;
            if total_available < colored_needed + self.colorless {
                return false;
            }
        }

        true
    }

    /// Creates a new mana cost with the given amounts of each color.
    pub fn new_with_colors(
        colorless: u64,
        white: u64,
        blue: u64,
        black: u64,
        red: u64,
        green: u64,
    ) -> Self {
        let mut color = ManaColor::NONE;
        if white > 0 {
            color |= ManaColor::WHITE;
        }
        if blue > 0 {
            color |= ManaColor::BLUE;
        }
        if black > 0 {
            color |= ManaColor::BLACK;
        }
        if red > 0 {
            color |= ManaColor::RED;
        }
        if green > 0 {
            color |= ManaColor::GREEN;
        }
        if colorless > 0 {
            color |= ManaColor::COLORLESS;
        }

        Self {
            color,
            reflectable_color: ReflectableColor::new(color),
            white,
            blue,
            black,
            red,
            green,
            colorless,
        }
    }

    /// Returns the total amount of colored mana (excluding colorless)
    #[allow(dead_code)]
    pub fn colored_total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns a string representation of all colored mana symbols and a count
    #[allow(dead_code)]
    fn colored_symbols(&self) -> (String, u64) {
        let mut symbols = String::new();
        let mut count = 0;

        for _ in 0..self.white {
            symbols.push_str("{W}");
            count += 1;
        }
        for _ in 0..self.blue {
            symbols.push_str("{U}");
            count += 1;
        }
        for _ in 0..self.black {
            symbols.push_str("{B}");
            count += 1;
        }
        for _ in 0..self.red {
            symbols.push_str("{R}");
            count += 1;
        }
        for _ in 0..self.green {
            symbols.push_str("{G}");
            count += 1;
        }

        (symbols, count)
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cost = String::new();

        // Add generic mana first (colorless mana that can be paid with any color)
        if self.colorless > 0 {
            // For generic mana, we use the number directly with braces
            cost.push_str(&format!("{{{}}}", self.colorless));
        }

        // Add colored mana in WUBRG order
        for _ in 0..self.white {
            cost.push_str("{W}");
        }
        for _ in 0..self.blue {
            cost.push_str("{U}");
        }
        for _ in 0..self.black {
            cost.push_str("{B}");
        }
        for _ in 0..self.red {
            cost.push_str("{R}");
        }
        for _ in 0..self.green {
            cost.push_str("{G}");
        }

        if cost.is_empty() {
            cost.push_str("{0}");
        }

        write!(f, "{}", cost)
    }
}

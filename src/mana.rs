/// Mana system for Magic: The Gathering.
///
/// This module provides:
/// - Mana cost representation and parsing
/// - Mana pool management
/// - Mana payment validation
/// - Color identity calculations
use bevy::{prelude::*, utils::HashMap};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    /// Represents the colors of mana in Magic: The Gathering.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Color: u32 {
        const NONE = 0;
        const WHITE = 1 << 0;
        const BLUE = 1 << 1;
        const BLACK = 1 << 2;
        const RED = 1 << 3;
        const GREEN = 1 << 4;
        const COLORLESS = 1 << 5;
    }
}

/// Represents mana costs with specific amounts for each color
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Mana {
    /// The colors present in this mana cost
    pub color: Color,
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

/// A pool of mana that can be used to cast spells.
///
/// This tracks both the amount and type of mana available to a player.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    /// Map of colors to mana amounts
    pub mana: HashMap<Color, Mana>,
}

impl Mana {
    /// Creates a new empty mana cost.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total amount of mana in this cost.
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green + self.colorless
    }

    /// Returns true if this mana cost contains any colored mana.
    pub fn has_color(&self) -> bool {
        self.white > 0 || self.blue > 0 || self.black > 0 || self.red > 0 || self.green > 0
    }

    /// Returns true if this mana cost can be paid with the given mana pool.
    #[allow(dead_code)]
    pub fn can_pay(&self, pool: &ManaPool) -> bool {
        // Check colored mana requirements
        if self.white > pool.mana.get(&self.color).map_or(0, |m| m.white)
            || self.blue > pool.mana.get(&self.color).map_or(0, |m| m.blue)
            || self.black > pool.mana.get(&self.color).map_or(0, |m| m.black)
            || self.red > pool.mana.get(&self.color).map_or(0, |m| m.red)
            || self.green > pool.mana.get(&self.color).map_or(0, |m| m.green)
            || self.colorless > pool.mana.get(&self.color).map_or(0, |m| m.colorless)
        {
            return false;
        }

        // Check if we have enough total mana for generic costs
        let remaining_mana = pool.mana.iter().map(|(_, m)| m.total()).sum::<u64>() - self.total();

        remaining_mana >= self.total()
    }

    /// Creates a new Mana instance with the specified amounts for each color.
    #[allow(dead_code)]
    pub fn new_with_colors(
        colorless: u64,
        white: u64,
        blue: u64,
        black: u64,
        red: u64,
        green: u64,
    ) -> Self {
        let mut color = Color::COLORLESS;

        // Set color flags based on presence of colored mana
        if white > 0 {
            color |= Color::WHITE;
        }
        if blue > 0 {
            color |= Color::BLUE;
        }
        if black > 0 {
            color |= Color::BLACK;
        }
        if red > 0 {
            color |= Color::RED;
        }
        if green > 0 {
            color |= Color::GREEN;
        }

        Self {
            color,
            white,
            blue,
            black,
            red,
            green,
            colorless,
        }
    }

    /// Returns the total amount of colored mana
    #[allow(dead_code)]
    pub fn colored_total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns the number of colored mana symbols in the cost
    #[allow(dead_code)]
    fn colored_symbols(&self) -> (String, u64) {
        let mut symbols = String::new();
        let mut count = 0;

        // Add colored mana symbols in WUBRG order
        if self.color.contains(Color::WHITE) {
            symbols.push_str(&"W".repeat(self.white as usize));
            count += self.white;
        }
        if self.color.contains(Color::BLUE) {
            symbols.push_str(&"U".repeat(self.blue as usize));
            count += self.blue;
        }
        if self.color.contains(Color::BLACK) {
            symbols.push_str(&"B".repeat(self.black as usize));
            count += self.black;
        }
        if self.color.contains(Color::RED) {
            symbols.push_str(&"R".repeat(self.red as usize));
            count += self.red;
        }
        if self.color.contains(Color::GREEN) {
            symbols.push_str(&"G".repeat(self.green as usize));
            count += self.green;
        }

        (symbols, count)
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cost = String::new();

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

        // Add colorless mana
        for _ in 0..self.colorless {
            cost.push_str("{C}");
        }

        if cost.is_empty() {
            cost.push_str("{0}");
        }

        write!(f, "{}", cost)
    }
}

impl ManaPool {
    /// Creates a new empty mana pool.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            mana: HashMap::new(),
        }
    }

    /// Adds mana to the pool.
    #[allow(dead_code)]
    pub fn add(&mut self, mana: Mana) {
        self.mana
            .entry(mana.color)
            .and_modify(|e| {
                e.white += mana.white;
                e.blue += mana.blue;
                e.black += mana.black;
                e.red += mana.red;
                e.green += mana.green;
                e.colorless += mana.colorless;
            })
            .or_insert(mana);
    }

    /// Removes mana from the pool if possible.
    ///
    /// Returns true if the mana was successfully removed,
    /// false if there wasn't enough mana of the right types.
    #[allow(dead_code)]
    pub fn remove(&mut self, mana: Mana) -> bool {
        if let Some(existing) = self.mana.get_mut(&mana.color) {
            if existing.white >= mana.white
                && existing.blue >= mana.blue
                && existing.black >= mana.black
                && existing.red >= mana.red
                && existing.green >= mana.green
                && existing.colorless >= mana.colorless
            {
                existing.white -= mana.white;
                existing.blue -= mana.blue;
                existing.black -= mana.black;
                existing.red -= mana.red;
                existing.green -= mana.green;
                existing.colorless -= mana.colorless;

                if existing.total() == 0 {
                    self.mana.remove(&mana.color);
                }
                return true;
            }
        }
        false
    }

    /// Empties the mana pool.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.mana.clear();
    }
}

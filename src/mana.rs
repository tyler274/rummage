use bevy::{prelude::*, utils::HashMap};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

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
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct Color: u8 {
        const COLORLESS = 0b00000;
        const WHITE = 0b00001;
        const BLUE = 0b00010;
        const BLACK = 0b00100;
        const RED = 0b01000;
        const GREEN = 0b10000;
        const ALL = Self::WHITE.bits() | Self::BLUE.bits() | Self::BLACK.bits() | Self::RED.bits() | Self::GREEN.bits();
    }
}

/// Represents mana costs with specific amounts for each color
#[derive(
    Component,
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct Mana {
    pub color: Color, // Used to quickly check which colors are present
    pub white: u64,
    pub blue: u64,
    pub black: u64,
    pub red: u64,
    pub green: u64,
    pub colorless: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    pub mana: HashMap<Color, Mana>,
}

#[allow(dead_code)]
impl ManaPool {
    pub fn new() -> Self {
        Self {
            mana: HashMap::new(),
        }
    }

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
}

impl Mana {
    /// Creates a new Mana instance with the specified amounts for each color.
    #[allow(dead_code)]
    pub fn new(colorless: u64, white: u64, blue: u64, black: u64, red: u64, green: u64) -> Self {
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

    /// Returns the total amount of mana
    pub fn total(&self) -> u64 {
        self.colorless + self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns the total amount of colored mana
    pub fn _colored_total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns the number of colored mana symbols in the cost
    fn _colored_symbols(&self) -> (String, u64) {
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

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut mana_text = String::new();

        // Add colorless mana first if any
        if self.colorless > 0 {
            write!(f, "{}", self.colorless)?;
        }

        // Add colored mana symbols in WUBRG order
        for _ in 0..self.white {
            mana_text.push_str("⚪"); // White circle emoji
        }
        for _ in 0..self.blue {
            mana_text.push_str("🔵"); // Blue circle emoji
        }
        for _ in 0..self.black {
            mana_text.push_str("⚫"); // Black circle emoji
        }
        for _ in 0..self.red {
            mana_text.push_str("🔴"); // Red circle emoji
        }
        for _ in 0..self.green {
            mana_text.push_str("🟢"); // Green circle emoji
        }

        write!(f, "{}", mana_text)
    }
}

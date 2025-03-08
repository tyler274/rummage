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

/// Represents a quantity of mana of a specific color.
///
/// The `Mana` struct contains:
/// - `color`: The color of the mana, represented by the `Color` bitflags.
/// - `amount`: The amount of mana of the specified color.
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mana {
    pub color: Color,
    pub amount: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    pub mana: HashMap<Color, Mana>,
}

impl ManaPool {
    pub fn new() -> Self {
        Self {
            mana: HashMap::new(),
        }
    }

    pub fn add(&mut self, mana: Mana) {
        self.mana.entry(mana.color).and_modify(|e| e.amount += mana.amount).or_insert(mana);
    }

    pub fn remove(&mut self, mana: Mana) -> bool {
        if let Some(existing) = self.mana.get_mut(&mana.color) {
            if existing.amount >= mana.amount {
                existing.amount -= mana.amount;
                if existing.amount == 0 {
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
    /// 
    /// # Arguments
    /// * `colorless` - Amount of colorless mana
    /// * `white` - Amount of white mana
    /// * `red` - Amount of red mana
    /// * `blue` - Amount of blue mana
    /// * `black` - Amount of black mana
    /// * `green` - Amount of green mana
    pub fn new(colorless: u64, white: u64, red: u64, blue: u64, black: u64, green: u64) -> Self {
        let mut color = Color::COLORLESS;
        let mut total = colorless;

        // Add colored mana
        if white > 0 {
            color |= Color::WHITE;
            total += white;
        }
        if red > 0 {
            color |= Color::RED;
            total += red;
        }
        if blue > 0 {
            color |= Color::BLUE;
            total += blue;
        }
        if black > 0 {
            color |= Color::BLACK;
            total += black;
        }
        if green > 0 {
            color |= Color::GREEN;
            total += green;
        }

        Self {
            color,
            amount: total,
        }
    }

    /// Returns the number of colored mana symbols in the cost
    fn colored_symbols(&self) -> (String, u64) {
        let mut symbols = String::new();
        let mut count = 0;

        // Add colored mana symbols in WUBRG order
        if self.color.contains(Color::WHITE) {
            symbols.push_str(&"W".repeat(self.amount as usize));
            count += self.amount;
        }
        if self.color.contains(Color::BLUE) {
            symbols.push_str(&"U".repeat(self.amount as usize));
            count += self.amount;
        }
        if self.color.contains(Color::BLACK) {
            symbols.push_str(&"B".repeat(self.amount as usize));
            count += self.amount;
        }
        if self.color.contains(Color::RED) {
            symbols.push_str(&"R".repeat(self.amount as usize));
            count += self.amount;
        }
        if self.color.contains(Color::GREEN) {
            symbols.push_str(&"G".repeat(self.amount as usize));
            count += self.amount;
        }

        (symbols, count)
    }
}

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // For colorless mana
        if self.color == Color::COLORLESS {
            return write!(f, "{}", self.amount);
        }

        let mut mana_text = String::new();

        // Add colored mana symbols in WUBRG order
        // Using circle emojis: ⚪️🔵⚫️🔴🟢
        let mut remaining = self.amount;

        // Add colored mana symbols
        if self.color.contains(Color::WHITE) {
            for _ in 0..2 {
                mana_text.push('⚪');
                remaining -= 1;
            }
        }
        if self.color.contains(Color::BLUE) {
            for _ in 0..2 {
                mana_text.push('🔵');
                remaining -= 1;
            }
        }
        if self.color.contains(Color::BLACK) {
            for _ in 0..2 {
                mana_text.push('⚫');
                remaining -= 1;
            }
        }
        if self.color.contains(Color::RED) {
            for _ in 0..2 {
                mana_text.push('🔴');
                remaining -= 1;
            }
        }
        if self.color.contains(Color::GREEN) {
            for _ in 0..2 {
                mana_text.push('🟢');
                remaining -= 1;
            }
        }

        // Add colorless mana first if any
        if remaining > 0 {
            let mut result = remaining.to_string();
            result.push_str(&mana_text);
            write!(f, "{}", result)
        } else {
            write!(f, "{}", mana_text)
        }
    }
}

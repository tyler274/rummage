/// Mana system for Magic: The Gathering.
///
/// This module provides functionality for:
/// - Mana cost representation and parsing
/// - Mana pool management
/// - Mana payment validation
/// - Color identity calculations
///
use bevy::{prelude::*, utils::HashMap};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;

/// MTG mana symbol mappings using the Mana font
/// See: https://mana.andrewgioia.com/icons.html
///
/// These mappings define how mana symbols in braces (e.g., `{W}`, `{2}`) are rendered
/// using the Mana font. The font expects the braced symbols as input and renders them
/// as the appropriate mana symbols.
///
pub const MANA_SYMBOLS: &[(&str, char)] = &[
    ("{W}", '\u{e600}'),  // White mana
    ("{U}", '\u{e601}'),  // Blue mana
    ("{B}", '\u{e602}'),  // Black mana
    ("{R}", '\u{e603}'),  // Red mana
    ("{G}", '\u{e604}'),  // Green mana
    ("{C}", '\u{e905}'),  // Colorless mana
    ("{0}", '\u{e605}'),  // Zero mana
    ("{1}", '\u{e606}'),  // One generic mana
    ("{2}", '\u{e607}'),  // Two generic mana
    ("{3}", '\u{e608}'),  // Three generic mana
    ("{4}", '\u{e609}'),  // Four generic mana
    ("{5}", '\u{e60a}'),  // Five generic mana
    ("{6}", '\u{e60b}'),  // Six generic mana
    ("{7}", '\u{e60c}'),  // Seven generic mana
    ("{8}", '\u{e60d}'),  // Eight generic mana
    ("{9}", '\u{e60e}'),  // Nine generic mana
    ("{10}", '\u{e60f}'), // Ten generic mana
    ("{11}", '\u{e610}'), // Eleven generic mana
    ("{12}", '\u{e611}'), // Twelve generic mana
    ("{13}", '\u{e612}'), // Thirteen generic mana
    ("{14}", '\u{e613}'), // Fourteen generic mana
    ("{15}", '\u{e614}'), // Fifteen generic mana
    ("{16}", '\u{e615}'), // Sixteen generic mana
    ("{20}", '\u{e616}'), // Twenty generic mana
    ("{X}", '\u{e617}'),  // Variable mana
    ("{Y}", '\u{e618}'),  // Variable mana Y
    ("{Z}", '\u{e619}'),  // Variable mana Z
    ("{T}", '\u{e61a}'),  // Tap symbol
    ("{Q}", '\u{e61b}'),  // Untap symbol
];

/// Converts a mana symbol string to its font character representation.
///
/// This function converts braced symbols like "{B}" to the appropriate
/// character that the Mana font uses to render the symbol.
///
/// # Arguments
/// * `symbol` - The mana symbol to convert (e.g., "{W}", "{2}")
///
pub fn mana_symbol_to_char(symbol: &str) -> String {
    let cleaned = symbol.trim();

    // Check if we have a direct mapping
    for (key, val) in MANA_SYMBOLS {
        if key == &cleaned {
            return val.to_string();
        }
    }

    // If no direct mapping, check if it's a symbol with braces
    if cleaned.starts_with('{') && cleaned.ends_with('}') {
        let inner = &cleaned[1..cleaned.len() - 1];

        // Handle special cases
        match inner {
            "W" => return '\u{e600}'.to_string(), // White mana
            "U" => return '\u{e601}'.to_string(), // Blue mana
            "B" => return '\u{e602}'.to_string(), // Black mana (skull)
            "R" => return '\u{e603}'.to_string(), // Red mana
            "G" => return '\u{e604}'.to_string(), // Green mana
            "T" => return '\u{e61a}'.to_string(), // Tap symbol
            "X" => return 'x'.to_string(),
            _ => {
                // Try to parse as a number for generic mana
                if let Ok(num) = inner.parse::<u32>() {
                    if num <= 9 {
                        return char::from_digit(num, 10).unwrap_or('0').to_string();
                    } else if num == 10 {
                        return 'a'.to_string();
                    } else if num == 11 {
                        return 'b'.to_string();
                    } else if num == 12 {
                        return 'c'.to_string();
                    } else if num == 13 {
                        return 'd'.to_string();
                    } else if num == 14 {
                        return 'e'.to_string();
                    } else if num == 15 {
                        return 'f'.to_string();
                    } else if num == 16 {
                        return 'g'.to_string();
                    } else if num == 20 {
                        return 'h'.to_string();
                    }
                }
            }
        }
    }

    // If no match found, just return the original
    symbol.to_string()
}

bitflags! {
    /// Color represents the five colors of Magic and colorless as bit flags, allowing combinations
    /// of colors to be represented efficiently.
    ///
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

/// Wrapper around Color for reflection support
#[derive(
    Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect,
)]
#[reflect(Serialize, Deserialize)]
pub struct ReflectableColor {
    bits: u32,
}

impl ReflectableColor {
    pub fn new(color: Color) -> Self {
        Self { bits: color.bits() }
    }

    pub fn color(&self) -> Color {
        Color::from_bits_truncate(self.bits)
    }
}

impl From<Color> for ReflectableColor {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

impl From<ReflectableColor> for Color {
    fn from(reflectable: ReflectableColor) -> Self {
        reflectable.color()
    }
}

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
    pub color: Color,
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

/// A pool of mana that can be used to cast spells.
///
/// This tracks both the amount and type of mana available to a player.
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ManaPool {
    /// Map of colors to mana amounts - we need to use a wrapper here since Color can't be a HashMap key with Reflect
    #[reflect(ignore)]
    pub mana: HashMap<Color, Mana>,
    /// Reflectable version of the mana map using u32 keys (the bits of Color)
    pub reflectable_mana: Vec<(ReflectableColor, Mana)>,
}

impl Mana {
    /// Creates a new empty mana cost.
    ///
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the converted mana cost (total mana of all colors and colorless)
    pub fn converted_mana_cost(&self) -> u64 {
        self.total()
    }

    /// Returns the amount of mana of a specific color
    pub fn colored_mana_cost(&self, color: Color) -> u64 {
        match color {
            Color::WHITE => self.white,
            Color::BLUE => self.blue,
            Color::BLACK => self.black,
            Color::RED => self.red,
            Color::GREEN => self.green,
            Color::COLORLESS => self.colorless,
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
    pub fn has_color(&self) -> bool {
        self.white > 0 || self.blue > 0 || self.black > 0 || self.red > 0 || self.green > 0
    }

    /// Returns true if this mana cost can be paid with the given mana pool.
    ///
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
            reflectable_color: ReflectableColor::from(color),
            white,
            blue,
            black,
            red,
            green,
            colorless,
        }
    }

    /// Returns the total amount of colored mana (excluding colorless).

    #[allow(dead_code)]
    pub fn colored_total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns the number of colored mana symbols in the cost and their string representation.

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

/// Formats mana costs using font characters.
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

impl ManaPool {
    /// Creates a new empty mana pool.
    ///
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds mana to the pool.
    ///
    pub fn add(&mut self, mana: Mana) {
        // Add to the HashMap for efficient access
        let entry = self.mana.entry(mana.color).or_default();
        entry.white += mana.white;
        entry.blue += mana.blue;
        entry.black += mana.black;
        entry.red += mana.red;
        entry.green += mana.green;
        entry.colorless += mana.colorless;
        entry.color |= mana.color;

        // Update the reflectable version
        self.sync_reflectable_mana();
    }

    /// Removes mana from the pool. Returns true if successful.
    ///
    pub fn remove(&mut self, mana: Mana) -> bool {
        if let Some(entry) = self.mana.get_mut(&mana.color) {
            if entry.white >= mana.white
                && entry.blue >= mana.blue
                && entry.black >= mana.black
                && entry.red >= mana.red
                && entry.green >= mana.green
                && entry.colorless >= mana.colorless
            {
                entry.white -= mana.white;
                entry.blue -= mana.blue;
                entry.black -= mana.black;
                entry.red -= mana.red;
                entry.green -= mana.green;
                entry.colorless -= mana.colorless;

                // Update the reflectable version
                self.sync_reflectable_mana();

                return true;
            }
        }
        false
    }

    /// Clears all mana from the pool.
    ///
    pub fn clear(&mut self) {
        self.mana.clear();
        self.reflectable_mana.clear();
    }

    /// Sync the reflectable mana with the actual mana HashMap
    fn sync_reflectable_mana(&mut self) {
        self.reflectable_mana.clear();
        for (&color, &mana) in self.mana.iter() {
            self.reflectable_mana
                .push((ReflectableColor::from(color), mana));
        }
    }

    /// Rebuild the mana HashMap from reflectable_mana (used after deserialization)
    pub fn rebuild_from_reflectable(&mut self) {
        self.mana.clear();
        for (reflectable_color, mana) in &self.reflectable_mana {
            self.mana.insert(reflectable_color.color(), *mana);
        }
    }
}

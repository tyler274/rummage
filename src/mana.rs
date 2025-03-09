/// Mana system for Magic: The Gathering.
///
/// This module provides functionality for:
/// - Mana cost representation and parsing
/// - Mana pool management
/// - Mana payment validation
/// - Color identity calculations
///
/// # Examples
///
/// ```
/// use rummage::mana::{Mana, Color};
///
/// // Create a mana cost of {2}{W}{W}
/// let cost = Mana::new_with_colors(2, 2, 0, 0, 0, 0);
/// assert_eq!(cost.white, 2);
/// assert_eq!(cost.colorless, 2);
/// assert!(cost.color.contains(Color::WHITE));
/// ```
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
/// # Examples
/// ```
/// use rummage::mana::mana_symbol_to_char;
/// assert_eq!(mana_symbol_to_char("{W}"), "{W}"); // Will render as white mana symbol
/// assert_eq!(mana_symbol_to_char("{2}"), "{2}"); // Will render as generic mana 2
/// ```
pub const _MANA_SYMBOLS: &[(&str, char)] = &[
    ("{W}", 'w'),  // White mana
    ("{U}", 'u'),  // Blue mana
    ("{B}", 'b'),  // Black mana
    ("{R}", 'r'),  // Red mana
    ("{G}", 'g'),  // Green mana
    ("{C}", 'c'),  // Colorless mana
    ("{0}", '0'),  // Zero mana
    ("{1}", '1'),  // One generic mana
    ("{2}", '2'),  // Two generic mana
    ("{3}", '3'),  // Three generic mana
    ("{4}", '4'),  // Four generic mana
    ("{5}", '5'),  // Five generic mana
    ("{6}", '6'),  // Six generic mana
    ("{7}", '7'),  // Seven generic mana
    ("{8}", '8'),  // Eight generic mana
    ("{9}", '9'),  // Nine generic mana
    ("{10}", 'a'), // Ten generic mana
    ("{X}", 'x'),  // Variable mana
    ("{T}", 't'),  // Tap symbol
];

/// Converts a mana symbol string to its font character representation.
///
/// This function preserves the braces around mana symbols, as the Mana font
/// expects braced symbols (e.g., `{W}`, `{2}`) to render them correctly.
///
/// # Arguments
/// * `symbol` - The mana symbol to convert (e.g., "{W}", "{2}")
///
/// # Returns
/// The symbol string unchanged, as the Mana font handles the conversion internally.
///
/// # Examples
/// ```
/// use rummage::mana::mana_symbol_to_char;
/// assert_eq!(mana_symbol_to_char("{W}"), "{W}");
/// ```
pub fn _mana_symbol_to_char(symbol: &str) -> String {
    // Keep the braces around the symbol for proper font rendering
    symbol.to_string()
}

/// Converts rules text containing mana symbols to font character representation.
///
/// This function preserves mana symbols in braces within the text, as the Mana font
/// expects braced symbols to render them correctly.
///
/// # Arguments
/// * `text` - The rules text containing mana symbols (e.g., "Add {W} to your mana pool")
///
/// # Returns
/// The text unchanged, as the Mana font handles the symbol conversion internally.
///
/// # Examples
/// ```
/// use rummage::mana::convert_rules_text_to_symbols;
/// assert_eq!(convert_rules_text_to_symbols("Add {W}"), "Add {W}");
/// ```
pub fn convert_rules_text_to_symbols(text: &str) -> String {
    // Keep the braces around the symbols for proper font rendering
    text.to_string()
}

bitflags! {
    /// Represents the colors of mana in Magic: The Gathering.
    ///
    /// Each color is represented by a bit flag, allowing for combinations
    /// of colors to be represented efficiently.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Color;
    ///
    /// let azorius = Color::WHITE | Color::BLUE;
    /// assert!(azorius.contains(Color::WHITE));
    /// assert!(azorius.contains(Color::BLUE));
    /// assert!(!azorius.contains(Color::BLACK));
    /// ```
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

/// Represents mana costs with specific amounts for each color.
///
/// This struct tracks both the colors present in the mana cost and
/// the specific amounts of each type of mana required.
///
/// # Examples
///
/// ```
/// use rummage::mana::Mana;
///
/// // Create a mana cost of {1}{U}{U}
/// let cost = Mana::new_with_colors(1, 0, 2, 0, 0, 0);
/// assert_eq!(cost.colorless, 1);
/// assert_eq!(cost.blue, 2);
/// assert_eq!(cost.to_string(), "{U}{U}{C}");
/// ```
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
///
/// # Examples
///
/// ```
/// use rummage::mana::{Mana, ManaPool};
///
/// let mut pool = ManaPool::new();
/// let blue_mana = Mana::new_with_colors(0, 0, 1, 0, 0, 0);
/// pool.add(blue_mana);
///
/// // Try to use the blue mana
/// assert!(pool.remove(blue_mana));
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    /// Map of colors to mana amounts
    pub mana: HashMap<Color, Mana>,
}

impl Mana {
    /// Creates a new empty mana cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Mana;
    ///
    /// let cost = Mana::new();
    /// assert_eq!(cost.total(), 0);
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total amount of mana in this cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Mana;
    ///
    /// let cost = Mana::new_with_colors(1, 2, 0, 0, 0, 0);
    /// assert_eq!(cost.total(), 3); // 1 colorless + 2 white
    /// ```
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green + self.colorless
    }

    /// Returns true if this mana cost contains any colored mana.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Mana;
    ///
    /// let colorless = Mana::new_with_colors(2, 0, 0, 0, 0, 0);
    /// assert!(!colorless.has_color());
    ///
    /// let white = Mana::new_with_colors(0, 1, 0, 0, 0, 0);
    /// assert!(white.has_color());
    /// ```
    pub fn has_color(&self) -> bool {
        self.white > 0 || self.blue > 0 || self.black > 0 || self.red > 0 || self.green > 0
    }

    /// Returns true if this mana cost can be paid with the given mana pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::{Mana, ManaPool};
    ///
    /// let mut pool = ManaPool::new();
    /// pool.add(Mana::new_with_colors(0, 2, 0, 0, 0, 0)); // Add {W}{W}
    ///
    /// let cost = Mana::new_with_colors(0, 1, 0, 0, 0, 0); // Cost is {W}
    /// assert!(cost.can_pay(&pool));
    ///
    /// let expensive = Mana::new_with_colors(0, 3, 0, 0, 0, 0); // Cost is {W}{W}{W}
    /// assert!(!expensive.can_pay(&pool));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::{Mana, Color};
    ///
    /// // Create {2}{W}{U}
    /// let cost = Mana::new_with_colors(2, 1, 1, 0, 0, 0);
    /// assert_eq!(cost.colorless, 2);
    /// assert_eq!(cost.white, 1);
    /// assert_eq!(cost.blue, 1);
    /// assert!(cost.color.contains(Color::WHITE | Color::BLUE));
    /// ```
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

    /// Returns the total amount of colored mana (excluding colorless).
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Mana;
    ///
    /// let cost = Mana::new_with_colors(2, 1, 1, 0, 0, 0); // {2}{W}{U}
    /// assert_eq!(cost.colored_total(), 2); // Only counts {W}{U}, not the {2}
    /// ```
    #[allow(dead_code)]
    pub fn colored_total(&self) -> u64 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Returns the number of colored mana symbols in the cost and their string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::Mana;
    ///
    /// let cost = Mana::new_with_colors(0, 1, 1, 0, 0, 0); // {W}{U}
    /// let (symbols, count) = cost.colored_symbols();
    /// assert_eq!(symbols, "WU");
    /// assert_eq!(count, 2);
    /// ```
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
///
/// # Examples
///
/// ```
/// use rummage::mana::Mana;
///
/// let cost = Mana::new_with_colors(2, 1, 1, 0, 0, 0);
/// // Will display as "wucc" (white, blue, and two colorless)
/// println!("{}", cost);
/// ```
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
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::ManaPool;
    ///
    /// let pool = ManaPool::new();
    /// assert!(pool.mana.is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            mana: HashMap::new(),
        }
    }

    /// Adds mana to the pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::{Mana, ManaPool};
    ///
    /// let mut pool = ManaPool::new();
    /// let blue_mana = Mana::new_with_colors(0, 0, 1, 0, 0, 0);
    /// pool.add(blue_mana);
    ///
    /// assert_eq!(pool.mana.len(), 1);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::{Mana, ManaPool};
    ///
    /// let mut pool = ManaPool::new();
    /// let blue_mana = Mana::new_with_colors(0, 0, 1, 0, 0, 0);
    /// pool.add(blue_mana);
    ///
    /// // Can remove the blue mana we added
    /// assert!(pool.remove(blue_mana));
    ///
    /// // Can't remove more than we have
    /// assert!(!pool.remove(blue_mana));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rummage::mana::{Mana, ManaPool};
    ///
    /// let mut pool = ManaPool::new();
    /// pool.add(Mana::new_with_colors(0, 0, 1, 0, 0, 0));
    /// assert!(!pool.mana.is_empty());
    ///
    /// pool.clear();
    /// assert!(pool.mana.is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.mana.clear();
    }
}

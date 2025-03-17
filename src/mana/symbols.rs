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

/// Checks if a string is a valid mana symbol
pub fn is_valid_mana_symbol(symbol: &str) -> bool {
    if symbol.len() < 3 || !symbol.starts_with('{') || !symbol.ends_with('}') {
        return false;
    }

    // Use our constant mapping to validate symbols
    for (key, _) in MANA_SYMBOLS {
        if *key == symbol {
            return true;
        }
    }

    // Generic check for numbers that may not be in our map
    let inner = &symbol[1..symbol.len() - 1];
    if inner.parse::<u32>().is_ok() {
        return true;
    }

    false
}

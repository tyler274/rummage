use super::components::Commander;
use crate::card::{Card, CardTypes};
use crate::mana::Color;
use bevy::prelude::Entity;
use std::collections::HashSet;

/// Commander-specific game rules and constants
#[allow(dead_code)]
pub struct CommanderRules;

impl CommanderRules {
    /// The amount of Commander damage needed to eliminate a player
    pub const COMMANDER_DAMAGE_THRESHOLD: u32 = 21;

    /// The number of players in a standard Commander game
    #[allow(dead_code)]
    pub const STANDARD_PLAYER_COUNT: usize = 4;

    /// The starting life total in Commander
    #[allow(dead_code)]
    pub const STARTING_LIFE: i32 = 40;

    /// Calculate the Commander tax (additional mana cost) for casting a Commander
    ///
    /// The commander tax increases by {2} each time the commander has been cast from the command zone
    #[allow(dead_code)]
    pub fn calculate_tax(cast_count: u32) -> u64 {
        // Convert to u64 to match the Mana.colorless type
        2u64 * cast_count as u64
    }

    /// Check if a player has been eliminated by Commander damage
    ///
    /// In Commander, a player loses the game if they've taken 21 or more combat damage
    /// from a single commander.
    #[allow(dead_code)]
    pub fn check_commander_damage_elimination(commander: &Commander, player: Entity) -> bool {
        commander
            .damage_dealt
            .iter()
            .any(|(p, damage)| *p == player && *damage >= Self::COMMANDER_DAMAGE_THRESHOLD)
    }

    /// Check if a card can be a Commander
    ///
    /// In standard Commander, a card can be a commander if it's a legendary creature
    /// or specifically states that it "can be your commander".
    #[allow(dead_code)]
    pub fn can_be_commander(card: &Card) -> bool {
        // Legendary creatures can be commanders
        if card.types.contains(CardTypes::LEGENDARY) && card.types.contains(CardTypes::CREATURE) {
            return true;
        }

        // Cards with "can be your commander" text can also be commanders
        // For simplicity, we're just checking if the text contains the phrase
        card.rules_text
            .to_lowercase()
            .contains("can be your commander")
    }

    /// Extract the color identity of a card
    ///
    /// A card's color identity consists of all colors in its mana cost,
    /// color indicator, and rules text.
    #[allow(dead_code)]
    pub fn extract_color_identity(card: &Card) -> HashSet<Color> {
        let mut colors = HashSet::new();

        // Add colors from mana cost
        if card.cost.white > 0 {
            colors.insert(Color::WHITE);
        }
        if card.cost.blue > 0 {
            colors.insert(Color::BLUE);
        }
        if card.cost.black > 0 {
            colors.insert(Color::BLACK);
        }
        if card.cost.red > 0 {
            colors.insert(Color::RED);
        }
        if card.cost.green > 0 {
            colors.insert(Color::GREEN);
        }

        // In a full implementation, we would also:
        // - Check mana symbols in rules text
        // - Check color indicators
        // - Check for land types that implicitly add colors

        colors
    }
}

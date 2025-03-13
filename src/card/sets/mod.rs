use std::collections::HashMap;

use bevy::prelude::*;

use crate::card::{Card, CardSet, Rarity};
use crate::mana::Color;

// Export set modules
pub mod alliances;
pub mod alpha;
pub mod innistrad_midnight_hunt;
pub mod legends;
pub mod scourge;

/// Registry for all available card sets
#[derive(Resource)]
pub struct CardRegistry {
    /// Cards organized by set code
    sets: HashMap<String, CardSetRegistry>,
    /// All cards ordered by release date (newest first)
    all_cards: Vec<Entity>,
}

/// Registry for a specific card set
#[derive(Clone)]
pub struct CardSetRegistry {
    /// Set information
    #[allow(dead_code)]
    pub set_info: CardSet,
    /// All card entities in this set
    pub cards: Vec<Entity>,
    /// Cards organized by color
    pub by_color: HashMap<Color, Vec<Entity>>,
    /// Cards organized by rarity
    pub by_rarity: HashMap<Rarity, Vec<Entity>>,
    /// Cards organized by type
    pub by_type: HashMap<String, Vec<Entity>>,
}

impl Default for CardRegistry {
    fn default() -> Self {
        Self {
            sets: HashMap::new(),
            all_cards: Vec::new(),
        }
    }
}

impl CardRegistry {
    /// Register a new card with the registry
    pub fn register_card(&mut self, entity: Entity, card: &Card, set: &CardSet, rarity: Rarity) {
        // Create the set registry if it doesn't exist
        if !self.sets.contains_key(&set.code) {
            self.sets.insert(
                set.code.clone(),
                CardSetRegistry {
                    set_info: set.clone(),
                    cards: Vec::new(),
                    by_color: HashMap::new(),
                    by_rarity: HashMap::new(),
                    by_type: HashMap::new(),
                },
            );
        }

        // Add the card to the set registry
        if let Some(set_registry) = self.sets.get_mut(&set.code) {
            // Add to general cards list
            set_registry.cards.push(entity);

            // Add to color-specific list based on the card's color identity
            let color = card.cost.color;
            set_registry
                .by_color
                .entry(color)
                .or_insert_with(Vec::new)
                .push(entity);

            // Add to rarity-specific list
            set_registry
                .by_rarity
                .entry(rarity)
                .or_insert_with(Vec::new)
                .push(entity);

            // Add to type-specific lists
            for type_name in card.type_line().split_whitespace() {
                if !["—", "-"].contains(&type_name) {
                    set_registry
                        .by_type
                        .entry(type_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(entity);
                }
            }
        }

        // Add to the global cards list
        self.all_cards.push(entity);
    }

    /// Get all cards in a specific set
    #[allow(dead_code)]
    pub fn get_set_cards(&self, set_code: &str) -> Option<&Vec<Entity>> {
        self.sets.get(set_code).map(|set| &set.cards)
    }

    /// Get cards of a specific color in a set
    #[allow(dead_code)]
    pub fn get_set_cards_by_color(&self, set_code: &str, color: Color) -> Option<&Vec<Entity>> {
        self.sets
            .get(set_code)
            .and_then(|set| set.by_color.get(&color))
    }

    /// Get cards of a specific rarity in a set
    #[allow(dead_code)]
    pub fn get_set_cards_by_rarity(&self, set_code: &str, rarity: Rarity) -> Option<&Vec<Entity>> {
        self.sets
            .get(set_code)
            .and_then(|set| set.by_rarity.get(&rarity))
    }

    /// Get cards of a specific type in a set
    #[allow(dead_code)]
    pub fn get_set_cards_by_type(&self, set_code: &str, type_name: &str) -> Option<&Vec<Entity>> {
        self.sets
            .get(set_code)
            .and_then(|set| set.by_type.get(type_name))
    }

    /// Get all cards of a specific color across all sets
    #[allow(dead_code)]
    pub fn get_cards_by_color(&self, color: Color) -> Vec<Entity> {
        let mut result = Vec::new();
        for set in self.sets.values() {
            if let Some(cards) = set.by_color.get(&color) {
                result.extend(cards);
            }
        }
        result
    }

    /// Get all cards of a specific rarity across all sets
    #[allow(dead_code)]
    pub fn get_cards_by_rarity(&self, rarity: Rarity) -> Vec<Entity> {
        let mut result = Vec::new();
        for set in self.sets.values() {
            if let Some(cards) = set.by_rarity.get(&rarity) {
                result.extend(cards);
            }
        }
        result
    }

    /// Get all cards of a specific type across all sets
    #[allow(dead_code)]
    pub fn get_cards_by_type(&self, type_name: &str) -> Vec<Entity> {
        let mut result = Vec::new();
        for set in self.sets.values() {
            if let Some(cards) = set.by_type.get(type_name) {
                result.extend(cards);
            }
        }
        result
    }

    /// Get all sets ordered by release date (newest first)
    #[allow(dead_code)]
    pub fn get_sets(&self) -> Vec<&CardSet> {
        self.sets.values().map(|set| &set.set_info).collect()
    }

    /// Get all cards
    #[allow(dead_code)]
    pub fn get_all_cards(&self) -> &Vec<Entity> {
        &self.all_cards
    }
}

/// Systems for card registry
pub mod systems {
    use bevy::prelude::*;

    use crate::card::{Card, CardSet, Rarity, sets::CardRegistry};

    /// System that initializes the card registry
    pub fn init_card_registry(mut commands: Commands) {
        commands.insert_resource(CardRegistry::default());
    }

    /// System that adds a card to the registry when it's created
    pub fn register_card(
        mut registry: ResMut<CardRegistry>,
        query: Query<(Entity, &Card, &CardSet, &Rarity), Added<Card>>,
    ) {
        for (entity, card, set, rarity) in query.iter() {
            registry.register_card(entity, card, set, *rarity);
        }
    }
}

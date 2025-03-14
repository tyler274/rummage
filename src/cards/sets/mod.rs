use std::collections::HashMap;

use bevy::prelude::*;

use crate::cards::{Card, CardSet, Rarity};
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
    #[allow(dead_code)]
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

        // Always add to all cards list
        self.all_cards.push(entity);

        // Add to set-specific registries
        if let Some(set_registry) = self.sets.get_mut(&set.code) {
            // Add to general cards list
            set_registry.cards.push(entity);

            // Add to color-specific list based on the card's color identity
            let color = card.cost.cost.color;
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
            let type_line =
                crate::cards::format_type_line(&card.type_info.types, &card.details.details);
            for type_name in type_line.split_whitespace() {
                if !["—", "-"].contains(&type_name) {
                    set_registry
                        .by_type
                        .entry(type_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(entity);
                }
            }
        }
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

    use crate::cards::{Card, CardSet, Rarity, sets::CardRegistry};

    /// System that initializes the card registry
    #[allow(dead_code)]
    pub fn init_card_registry(mut commands: Commands) {
        commands.insert_resource(CardRegistry::default());
    }

    /// Register new cards as they are added to the world
    #[allow(dead_code)]
    pub fn register_card(
        mut registry: ResMut<CardRegistry>,
        query: Query<(Entity, &Card, &CardSet, &Rarity), Added<Card>>,
    ) {
        for (entity, card, set, rarity) in query.iter() {
            registry.register_card(entity, card, set, *rarity);
        }
    }
}

/// Helper function to spawn a card and add set info + rarity
#[allow(dead_code)]
pub fn spawn_card_with_set_info(
    commands: &mut Commands,
    card: crate::cards::Card,
    set_info: CardSet,
    rarity: Rarity,
) -> Entity {
    // Store the name before moving card
    let card_name = card.name.name.clone();

    commands
        .spawn(card)
        .insert(set_info)
        .insert(rarity)
        .insert(Name::new(card_name))
        .id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, CardDetails, CardTypes};
    use crate::mana::{Color, Mana};
    use bevy::prelude::*;

    /// Test function demonstrating how to use the registry functions
    #[test]
    fn test_card_registry() {
        // Create a new registry
        let mut registry = CardRegistry::default();

        // Create a set
        let alpha_set = CardSet {
            code: "LEA".to_string(),
            name: "Limited Edition Alpha".to_string(),
            release_date: "1993-08-05".to_string(),
        };

        // Create an app to spawn entities
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn some test cards with default Mana
        let lightning_bolt = app
            .world
            .spawn(
                Card::builder("Lightning Bolt")
                    .cost(Mana::default())
                    .types(CardTypes::INSTANT)
                    .rules_text("Lightning Bolt deals 3 damage to any target.")
                    .build_or_panic(),
            )
            .id();

        let serra_angel = app
            .world
            .spawn(
                Card::builder("Serra Angel")
                    .cost(Mana::default())
                    .types(CardTypes::CREATURE)
                    .details(CardDetails::Creature(super::super::details::CreatureCard {
                        power: 4,
                        toughness: 4,
                        creature_type: crate::cards::types::CreatureType::ANGEL,
                    }))
                    .rules_text("Flying, vigilance")
                    .build_or_panic(),
            )
            .id();

        let black_lotus = app
            .world
            .spawn(
                Card::builder("Black Lotus")
                    .cost(Mana::default())
                    .types(CardTypes::ARTIFACT)
                    .rules_text("{T}, Sacrifice Black Lotus: Add three mana of any one color.")
                    .build_or_panic(),
            )
            .id();

        // Get the card components
        let lightning_bolt_card = app
            .world
            .entity(lightning_bolt)
            .get::<Card>()
            .unwrap()
            .clone();
        let serra_angel_card = app.world.entity(serra_angel).get::<Card>().unwrap().clone();
        let black_lotus_card = app.world.entity(black_lotus).get::<Card>().unwrap().clone();

        // Register the cards
        registry.register_card(
            lightning_bolt,
            &lightning_bolt_card,
            &alpha_set,
            Rarity::Common,
        );
        registry.register_card(serra_angel, &serra_angel_card, &alpha_set, Rarity::Uncommon);
        registry.register_card(black_lotus, &black_lotus_card, &alpha_set, Rarity::Rare);

        // Test querying by set
        let alpha_cards = registry.get_set_cards("LEA").unwrap();
        assert_eq!(alpha_cards.len(), 3);

        // Test querying by color
        let white_cards = registry
            .get_set_cards_by_color("LEA", Color::WHITE)
            .unwrap();
        assert_eq!(white_cards.len(), 1);
        assert_eq!(white_cards[0], serra_angel);

        // Test querying by rarity
        let rare_cards = registry
            .get_set_cards_by_rarity("LEA", Rarity::Rare)
            .unwrap();
        assert_eq!(rare_cards.len(), 1);
        assert_eq!(rare_cards[0], black_lotus);

        // Test getting all cards
        let all_cards = registry.get_all_cards();
        assert_eq!(all_cards.len(), 3);

        // Test getting all sets
        let sets = registry.get_sets();
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0].code, "LEA");
    }

    /// Test demonstrating how to use the init_card_registry and register_card systems
    #[test]
    fn test_registry_systems() {
        // Create a new app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add the systems
        app.add_systems(Startup, systems::init_card_registry);
        app.add_systems(Update, systems::register_card);

        // Run the startup system to initialize the registry
        app.update();

        // Check that the registry was created
        assert!(app.world.contains_resource::<CardRegistry>());

        // Create a set
        let alpha_set = CardSet {
            code: "LEA".to_string(),
            name: "Limited Edition Alpha".to_string(),
            release_date: "1993-08-05".to_string(),
        };

        // Create a command buffer to spawn entities
        let mut commands = Commands::new(&mut app.world, &mut app.schedule);

        // Spawn a card with set info and get the entity
        let card_entity = spawn_card_with_set_info(
            &mut commands,
            Card::builder("Test Card")
                .cost(Mana::default())
                .types(CardTypes::INSTANT)
                .build_or_panic(),
            alpha_set.clone(),
            Rarity::Rare,
        );

        // Apply commands and run update to register the card
        commands.apply(&mut app.world);
        app.update();

        // Check that the card was registered
        let registry = app.world.resource::<CardRegistry>();
        let alpha_cards = registry.get_set_cards("LEA").unwrap();
        assert_eq!(alpha_cards.len(), 1);
        assert_eq!(alpha_cards[0], card_entity);
    }
}

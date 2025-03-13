mod builder;
mod types;

pub use types::{Deck, DeckType, PlayerDeck};

// Re-export any other types or functions that should be public

// Plugin for deck-related functionality
use bevy::prelude::*;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        // Register any systems related to decks
        app.init_resource::<DeckRegistry>()
            .add_systems(Startup, register_default_decks)
            .add_systems(Startup, shuffle_all_player_decks);
    }
}

/// System to ensure all player decks are properly shuffled independently
/// This is run during startup to ensure each player starts with a randomized deck
fn shuffle_all_player_decks(mut player_decks: Query<&mut PlayerDeck>) {
    info!("Shuffling all player decks...");

    for (index, mut player_deck) in player_decks.iter_mut().enumerate() {
        player_deck.deck.shuffle();
        info!("Shuffled deck for player {}", index);
    }

    info!("All player decks have been independently shuffled");
}

// Registry for storing predefined decks
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct DeckRegistry {
    decks: std::collections::HashMap<String, Deck>,
}

impl DeckRegistry {
    #[allow(dead_code)]
    pub fn register_deck(&mut self, name: &str, deck: Deck) {
        self.decks.insert(name.to_string(), deck);
    }

    #[allow(dead_code)]
    pub fn get_deck(&self, name: &str) -> Option<&Deck> {
        self.decks.get(name)
    }

    #[allow(dead_code)]
    pub fn get_all_decks(&self) -> Vec<(&String, &Deck)> {
        self.decks.iter().collect()
    }
}

// Register default decks for testing/examples
fn register_default_decks(_deck_registry: ResMut<DeckRegistry>) {
    // Register any predefined decks
    // Example:
    // let mono_red = DeckBuilder::new()
    //     .with_name("Mono Red Aggro")
    //     .with_type(DeckType::Commander)
    //     .build();
    // deck_registry.register_deck("mono_red", mono_red);
}

// Get a collection of example cards that can be used to create a deck
// This moves the functionality from src/card/mod.rs to here
#[allow(dead_code)]
pub fn get_example_cards(_owner: Entity) -> Vec<crate::card::Card> {
    let mut cards = Vec::new();
    cards.extend(crate::card::artifacts::get_artifact_cards());
    cards.extend(crate::card::black::get_black_cards());
    cards.extend(crate::card::blue::get_blue_cards());
    cards.extend(crate::card::green::get_green_cards());
    cards.extend(crate::card::red::get_red_cards());
    cards.extend(crate::card::white::get_white_cards());
    cards
}

// Get unique cards for a player based on their player index
// This ensures different players get different cards
pub fn get_player_specific_cards(_owner: Entity, player_index: usize) -> Vec<crate::card::Card> {
    let mut cards = Vec::new();

    // Use different card subsets based on player index to ensure variety
    if player_index % 2 == 0 {
        // Even-indexed players (like Player 1) get artifact, black, and red cards
        cards.extend(crate::card::artifacts::get_artifact_cards());
        cards.extend(crate::card::black::get_black_cards());
        cards.extend(crate::card::red::get_red_cards());
    } else {
        // Odd-indexed players (like Player 2) get blue, green, and white cards
        cards.extend(crate::card::blue::get_blue_cards());
        cards.extend(crate::card::green::get_green_cards());
        cards.extend(crate::card::white::get_white_cards());
    }

    cards
}

// Return a shuffled deck of cards
#[allow(dead_code)]
pub fn get_shuffled_deck(owner: Entity) -> Deck {
    let cards = get_example_cards(owner);

    let mut deck = Deck::new("Example Deck".to_string(), DeckType::Standard, cards);

    deck.shuffle();
    deck
}

// Return a player-specific shuffled deck of cards
pub fn get_player_shuffled_deck(
    owner: Entity,
    player_index: usize,
    deck_name: Option<&str>,
) -> Deck {
    let cards = get_player_specific_cards(owner, player_index);

    let name = deck_name
        .unwrap_or(&format!("Player {} Deck", player_index + 1))
        .to_string();
    let mut deck = Deck::new(name, DeckType::Standard, cards);

    deck.shuffle();
    deck
}

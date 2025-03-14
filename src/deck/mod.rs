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
#[allow(dead_code)]
pub fn get_example_cards(_owner: Entity) -> Vec<crate::card::Card> {
    let mut cards = Vec::new();

    // Alpha set cards
    let (card, _, _, _, _, _, _) = crate::card::sets::alpha::ancestral_recall::get_card();
    cards.push(card);

    cards.push(crate::card::sets::alpha::counterspell::get_card());
    cards.push(crate::card::sets::alpha::fireball::get_card());
    cards.push(crate::card::sets::alpha::lightning_bolt::get_card());
    cards.push(crate::card::sets::alpha::time_walk::get_card());
    cards.push(crate::card::sets::alpha::wheel_of_fortune::get_card());

    // Alliances set cards
    cards.push(crate::card::sets::alliances::force_of_will::get_card());

    // Legends set cards
    cards.push(crate::card::sets::legends::mana_drain::get_card());

    // Scourge set cards
    cards.push(crate::card::sets::scourge::dragon_mage::get_card());

    cards
}

// Create a deck containing all implemented cards for a player
// This gives all players access to the full collection of cards
#[allow(dead_code)]
pub fn get_player_specific_cards() -> Vec<crate::card::Card> {
    let mut cards = Vec::new();

    // Alpha set cards
    let (card, _, _, _, _, _, _) = crate::card::sets::alpha::ancestral_recall::get_card();
    cards.push(card);

    cards.push(crate::card::sets::alpha::counterspell::get_card());
    cards.push(crate::card::sets::alpha::fireball::get_card());
    cards.push(crate::card::sets::alpha::lightning_bolt::get_card());
    cards.push(crate::card::sets::alpha::shivan_dragon::get_card());
    cards.push(crate::card::sets::alpha::time_walk::get_card());
    cards.push(crate::card::sets::alpha::wheel_of_fortune::get_card());

    // Alliances set cards
    cards.push(crate::card::sets::alliances::force_of_will::get_card());

    // Legends set cards
    cards.push(crate::card::sets::legends::mana_drain::get_card());

    // Scourge set cards
    cards.push(crate::card::sets::scourge::dragon_mage::get_card());

    // Innistrad Midnight Hunt
    cards.push(crate::card::sets::innistrad_midnight_hunt::briarbridge_tracker::get_card());
    cards.push(crate::card::sets::innistrad_midnight_hunt::brutal_cathar::get_card());
    cards.push(crate::card::sets::innistrad_midnight_hunt::cathars_call::get_card());
    cards.push(crate::card::sets::innistrad_midnight_hunt::champion_of_the_perished::get_card());
    cards.push(crate::card::sets::innistrad_midnight_hunt::delver_of_secrets::get_card());
    cards.push(crate::card::sets::innistrad_midnight_hunt::moonveil_regent::get_card());

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
    _owner: Entity,
    player_index: usize,
    deck_name: Option<&str>,
) -> Deck {
    let cards = get_player_specific_cards();

    let name = deck_name
        .unwrap_or(&format!("Player {} Deck", player_index + 1))
        .to_string();
    let mut deck = Deck::new(name, DeckType::Standard, cards);

    deck.shuffle();
    deck
}

mod artifacts;
mod black;
mod blue;
mod green;
mod penacony;
mod red;
mod white;

use crate::card::Card;
use crate::player::Player;
use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        // Get a random seed from system time
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        app.insert_resource(CardRng(StdRng::seed_from_u64(seed)))
            .add_systems(Startup, init_deck)
            .add_systems(Update, shuffle_player_cards);
    }
}

#[derive(Resource)]
struct CardRng(StdRng);

/// Returns a list of example cards for testing and development.
/// Currently returns a subset of 7 cards from the full deck.
///
/// # Arguments
/// * `player_entity` - The entity ID of the player who will own these cards
pub fn get_example_cards(_player_entity: Entity) -> Vec<Card> {
    let mut all_cards = [
        artifacts::get_artifact_cards(),
        blue::get_blue_cards(),
        red::get_red_cards(),
        white::get_white_cards(),
        black::get_black_cards(),
        green::get_green_cards(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    // Randomly select 7 cards for a proper starting hand
    all_cards.shuffle(&mut rand::rng());
    all_cards.truncate(7);
    all_cards
}

/// Initialize the deck with a random shuffle
fn init_deck(mut rng: ResMut<CardRng>) {
    // Ensure RNG is properly seeded by drawing a few numbers
    for _ in 0..10 {
        rng.0.random::<u64>();
    }
}

/// System that shuffles cards for all players.
/// Uses a seeded RNG for deterministic shuffling in multiplayer.
fn shuffle_player_cards(mut players: Query<&mut Player>, mut rng: ResMut<CardRng>) {
    for mut player in players.iter_mut() {
        player.cards.shuffle(&mut rng.0);
    }
}

pub mod artifacts;
pub mod black;
pub mod blue;
pub mod green;
pub mod mtgjson;
pub mod red;
pub mod white;

pub use mtgjson::MTGService;

use crate::card::{Card, debug_render_text_positions, handle_card_dragging};
use bevy::prelude::*;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_card_dragging, debug_render_text_positions));
    }
}

pub fn get_example_cards(_owner: Entity) -> Vec<Card> {
    let mut cards = Vec::new();
    cards.extend(artifacts::get_artifact_cards());
    cards.extend(black::get_black_cards());
    cards.extend(blue::get_blue_cards());
    cards.extend(green::get_green_cards());
    cards.extend(red::get_red_cards());
    cards.extend(white::get_white_cards());
    cards
}

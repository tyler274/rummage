use super::table::TableLayout;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

/// Calculate the appropriate position for a player based on their index
#[allow(dead_code)]
pub fn get_player_position(player_index: usize, config: &PlayerConfig) -> Transform {
    // Create a table layout calculator
    let table = TableLayout::new(config.player_count, config.player_card_distance);

    // Use the table layout to calculate the position
    table.get_player_position(player_index)
}

/// Calculate a card offset based on player position
#[allow(dead_code)]
pub fn get_card_offset(player_index: usize, config: &PlayerConfig) -> Vec3 {
    // Create a table layout calculator
    let table = TableLayout::new(config.player_count, config.player_card_distance);

    // Use the table layout to calculate the card offset
    table.get_card_offset(player_index)
}

/// Calculate appropriate transform for a player's cards
#[allow(dead_code)]
pub fn get_card_transform(
    player_transform: &Transform,
    player_index: usize,
    config: &PlayerConfig,
) -> Transform {
    let offset = get_card_offset(player_index, config);

    // Add the offset to the player's position
    let mut transform = *player_transform;
    transform.translation += offset;

    transform
}

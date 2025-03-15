//! Player spawn systems for positioning players and their cards around a table
//! This module handles spawning any number of players in a circular arrangement

mod cards;
mod position;
mod table;

use crate::camera::components::{AppLayer, GameCamera};
use crate::deck::{PlayerDeck, get_player_shuffled_deck};
use crate::player::components::Player;
use crate::player::playmat::spawn_player_playmat; // Import the new playmat function
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

/// Spawns players according to the PlayerConfig resource
///
/// This system:
/// 1. Reads the PlayerConfig to determine how many players to spawn
/// 2. Creates player entities with appropriate positioning
/// 3. Only spawns cards for player 1 by default (or all if configured)
/// 4. Creates a playmat for each player using the game engine Zone structure
/// 5. Creates independent deck components for each player
pub fn spawn_players(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    player_config: Option<Res<PlayerConfig>>,
) {
    // Use default config if none exists
    let config = player_config.map(|c| c.clone()).unwrap_or_default();

    info!("Spawning {} players...", config.player_count);

    // Create a table layout calculator for the players with appropriate playmat size
    let playmat_size = Vec2::new(330.0, 250.0); // Adjusted playmat size for corner-to-corner touching
    let table = table::TableLayout::new(config.player_count, config.player_card_distance)
        .with_playmat_size(playmat_size);

    // Spawn each player
    for player_index in 0..config.player_count {
        // Get position name for logging
        let position_name = table.get_position_name(player_index);

        // Create a new player using the builder pattern
        let player = Player::new(&format!("Player {} ({})", player_index + 1, position_name))
            .with_life(config.starting_life)
            .with_player_index(player_index);

        info!(
            "Creating player with index {} and name '{}'",
            player_index, player.name
        );

        // Get player position based on their index and table layout
        let player_transform = table.get_player_position(player_index);

        // Spawn the player entity
        let player_entity = commands
            .spawn((
                player.clone(),
                player_transform,
                GlobalTransform::default(),
                AppLayer::game_layers(), // Add to all game layers
            ))
            .id();

        info!(
            "Spawned player entity {:?} with index {} and name '{}' at position {:?}",
            player_entity, player_index, player.name, player_transform.translation
        );

        // Spawn the player's playmat
        spawn_player_playmat(
            commands,
            &asset_server,
            player_entity,
            &player,
            &config,
            player_transform.translation,
        );

        // Create a player-specific deck for ALL players
        let deck = get_player_shuffled_deck(
            player_entity,
            player_index,
            Some(&format!("Player {} Deck", player_index + 1)),
        );

        // Add the PlayerDeck component to the player entity
        commands
            .entity(player_entity)
            .insert(PlayerDeck::new(deck.clone()));

        info!(
            "Added independent deck component with {} cards to player {}",
            deck.cards.len(),
            player_index
        );

        // Only spawn visual cards for player 1 or if spawn_all_cards is true
        if player_index == 0 || config.spawn_all_cards {
            // Instead of getting new cards, draw from the player's own deck
            // Make a copy of the deck to draw from without modifying the original
            let mut temp_deck = deck.clone();

            // Draw 7 cards from the player's own deck as a starting hand
            let display_cards = temp_deck.draw_multiple(7);

            info!(
                "Drew {} cards from player {}'s own deck for display",
                display_cards.len(),
                player_index
            );

            // Update the player's cards while preserving other fields
            commands.entity(player_entity).insert(
                Player::new(&player.name)
                    .with_life(player.life)
                    .with_player_index(player.player_index),
            );

            // Spawn visual cards for all players that have cards
            info!(
                "Spawning visual cards for player {} ({})",
                player_index, position_name
            );

            // Get the base position for the player's cards
            let card_position = player_transform.translation;

            // Create visual representations of the cards
            cards::spawn_visual_cards(
                commands,
                display_cards,
                &game_cameras,
                &config.card_size,
                config.card_spacing_multiplier,
                card_position,
                player_index,
                player_entity,
                &table,
                Some(&asset_server),
            );
        } else {
            info!(
                "Skipping card spawning for player {} (index {})",
                player.name, player_index
            );
        }
    }

    info!("Player spawning complete!");
}

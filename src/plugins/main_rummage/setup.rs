use bevy::prelude::*;

use crate::camera::components::{AppLayer, GameCamera};
use crate::deck::{PlayerDeck, get_player_shuffled_deck};
use crate::player::components::Player;
use crate::player::playmat::spawn_player_playmat;
use crate::player::resources::PlayerConfig;
use crate::player::systems::spawn::table::TableLayout;

use super::visual_hand::SpawnVisualHand;

pub(super) fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _game_cameras: Query<Entity, With<GameCamera>>,
    player_config: Res<PlayerConfig>,
) {
    info!("Setting up game...");

    // Use default config if none exists
    let config = player_config.clone();

    info!("Spawning {} players...", config.player_count);

    // Create a table layout calculator for the players with appropriate playmat size
    let playmat_size = Vec2::new(430.0, 330.0); // Increased playmat size for larger cards
    let table = TableLayout::new(config.player_count, config.player_card_distance)
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
            &mut commands,
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

        // If cards should be spawned, add marker component instead of spawning directly
        if player_index == 0 || config.spawn_all_cards {
            commands.spawn(SpawnVisualHand {
                player_entity,
                deck: PlayerDeck::new(deck), // Store the deck copy needed
                position: player_transform.translation, // Store position
            });
        }
    }

    info!("Player spawning complete!");
}

use crate::camera::components::{AppLayer, GameCamera};
use crate::camera::{
    CameraPanState,
    config::CameraConfig,
    systems::{camera_movement, handle_window_resize, set_initial_zoom},
};
use crate::deck::{PlayerDeck, get_player_shuffled_deck};
use crate::game_engine::config::GameConfig;
use crate::menu::GameMenuState;
use crate::menu::state::StateTransitionContext;
use crate::player::components::Player;
use crate::player::playmat::spawn_player_playmat;
use crate::player::systems::spawn::cards;
use crate::player::systems::spawn::table::TableLayout;
use crate::player::{PlayerPlugin, resources::PlayerConfig, spawn_players};
use crate::replicon::RepliconPluginSettings;
use crate::snapshot::GameSnapshotPlugin;
#[cfg(feature = "snapshot")]
use crate::snapshot::systems::take_snapshot_after_setup;
use crate::state::GameState;
use crate::text::DebugConfig;
use bevy::prelude::*;

// Plugin for the actual game systems
pub struct RummagePlugin;

impl Plugin for RummagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::cards::drag::DragPlugin)
            .add_plugins(crate::cards::CardPlugin)
            .add_plugins(crate::deck::DeckPlugin)
            .add_plugins(crate::game_engine::GameEnginePlugin)
            .add_plugins(crate::text::TextPlugin::default())
            .add_plugins(PlayerPlugin)
            .insert_resource(DebugConfig {
                show_text_positions: true,
            })
            .insert_resource(CameraConfig::default())
            .insert_resource(CameraPanState::default())
            .insert_resource(
                PlayerConfig::new()
                    .with_player_count(4)
                    .with_spawn_all_cards(true)
                    .with_starting_life(40)
                    .with_player_card_distance(400.0)
                    .with_player_card_offset(0, -1200.0) // Bottom player
                    .with_player_card_offset(1, 0.0) // Right player
                    .with_player_card_offset(2, 1200.0) // Top player
                    .with_player_card_offset(3, 0.0), // Left player
            )
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (
                    setup_game,
                    // Only set initial zoom when not coming from pause menu
                    set_initial_zoom
                        .run_if(|context: Res<crate::menu::state::StateTransitionContext>| {
                            !context.from_pause_menu
                        })
                        .after(setup_game),
                    // Snapshot system is controlled by feature flag
                    #[cfg(feature = "snapshot")]
                    take_snapshot_after_setup
                        .run_if(|context: Res<crate::menu::state::StateTransitionContext>| {
                            !context.from_pause_menu
                        })
                        .after(setup_game),
                ),
            )
            .add_systems(
                Update,
                (handle_window_resize, camera_movement).run_if(in_state(GameMenuState::InGame)),
            );
    }
}

// System to set up the game state
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    player_config: Res<PlayerConfig>,
    game_config: Res<GameConfig>,
) {
    info!("Setting up game state...");

    // Use player config
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

            // Create the context for spawning cards
            let mut context = cards::CardSpawnContext {
                commands: &mut commands,
                game_cameras: &game_cameras,
                card_size: &config.card_size,
                spacing_multiplier: config.card_spacing_multiplier,
                player_position: card_position,
                player_index,
                player_entity,
                table: &table,
                asset_server_option: Some(&asset_server),
            };

            // Create visual representations of the cards
            cards::spawn_visual_cards(&mut context, display_cards);
        } else {
            info!(
                "Skipping card spawning for player {} (index {})",
                player.name, player_index
            );
        }
    }

    info!("Player spawning complete!");

    // Other setup logic using game_config
    info!("Game starting turn: {}", game_config.starting_turn);
    info!("Player life total: {}", game_config.player_life);
}

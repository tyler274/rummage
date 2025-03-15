mod game_plugin;

use bevy::prelude::*;

// Import what we need
use crate::camera::components::GameCamera;
use crate::game_engine::save::SaveLoadPlugin;
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, resources::PlayerConfig, spawn_players};

pub struct MainRummagePlugin;

impl Plugin for MainRummagePlugin {
    fn build(&self, app: &mut App) {
        // Add Player Plugin
        app.add_plugins(PlayerPlugin)
            // Add Save/Load system
            .add_plugins(SaveLoadPlugin)
            // Setup game configuration
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
            // Add game setup system
            .add_systems(OnEnter(GameMenuState::InGame), setup_game);
    }
}

// Game setup system that spawns players
fn setup_game(
    commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    context: Res<crate::menu::state::StateTransitionContext>,
    player_config: Res<PlayerConfig>,
) {
    // Skip setup if we're coming from pause menu
    if context.from_pause_menu {
        info!("Resuming from pause menu, skipping game setup");
        return;
    }

    // Normal game setup - this is a fresh game
    info!("Spawning players...");

    // Spawn the players
    spawn_players(commands, asset_server, game_cameras, Some(player_config));

    info!("Game setup complete!");
}

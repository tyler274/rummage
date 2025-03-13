use crate::camera::{
    CameraPanState,
    components::{AppLayer, GameCamera},
    config::CameraConfig,
    systems::{camera_movement, handle_window_resize, set_initial_zoom},
};
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, resources::PlayerConfig, spawn_players};
#[cfg(feature = "snapshot")]
use crate::snapshot::systems::take_snapshot_after_setup;
use crate::text::DebugConfig;
use bevy::prelude::*;

// Plugin for the actual game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::drag::DragPlugin)
            .add_plugins(crate::card::CardPlugin)
            .add_plugins(crate::deck::DeckPlugin)
            .add_plugins(crate::game_engine::GameEnginePlugin)
            .add_plugins(crate::text::TextPlugin)
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

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    context: Res<crate::menu::state::StateTransitionContext>,
    player_config: Res<PlayerConfig>,
) {
    info!("Setting up game environment...");
    info!("Game engine initializing game engine resources...");

    // Skip camera setup if we're coming from the pause menu and already have a camera
    if context.from_pause_menu {
        info!("Resuming from pause menu, skipping game setup");
        // Only set up camera if needed here, but don't create cards
        if game_cameras.is_empty() {
            info!("No game camera found despite coming from pause menu, setting up camera anyway");
            // Spawn a new camera directly here
            commands.spawn((
                Camera2d::default(),
                Camera {
                    order: 0, // Explicitly set order to 0 for game camera
                    ..default()
                },
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Transform::default(), // Use default transform position (0,0,0)
                GlobalTransform::default(),
                GameCamera,
                AppLayer::game_layers(), // Use combined game layers to see all game elements including cards
            ));

            // Initialize camera pan state
            commands.insert_resource(CameraPanState::default());
        }
    } else {
        // Normal game setup - this is a fresh game
        info!("Setting up game camera...");
        // Spawn a new camera directly here
        commands.spawn((
            Camera2d::default(),
            Camera {
                order: 0, // Explicitly set order to 0 for game camera
                ..default()
            },
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            Transform::default(), // Use default transform position (0,0,0)
            GlobalTransform::default(),
            GameCamera,
            AppLayer::game_layers(), // Use combined game layers to see all game elements including cards
        ));

        // Initialize camera pan state
        commands.insert_resource(CameraPanState::default());

        // Spawn the players using the new system
        info!("Spawning initial hand...");
        spawn_players(commands, asset_server, game_cameras, Some(player_config));
    }

    info!("Game setup complete!");
}

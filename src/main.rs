#![feature(trivial_bounds)]

mod camera;
mod card;
mod deck;
mod docs;
mod drag;
mod game_engine;
mod mana;
mod menu;
mod player;
mod text;

use bevy::DefaultPlugins;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use camera::{
    CameraPanState,
    components::{AppLayer, GameCamera},
    config::CameraConfig,
    snapshot::resources::SnapshotEvent,
    snapshot::systems::take_snapshot,
    systems::{camera_movement, handle_window_resize, set_initial_zoom},
};
use card::CardPlugin;
use deck::DeckPlugin;
use docs::Docs;
use drag::DragPlugin;
use game_engine::GameEnginePlugin;
use menu::{GameMenuState, MenuPlugin, state::StateTransitionContext};
use player::{PlayerPlugin, resources::PlayerConfig, spawn_players};
use text::DebugConfig;

// Plugin for the actual game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DragPlugin)
            .add_plugins(CardPlugin)
            .add_plugins(DeckPlugin)
            .add_plugins(GameEnginePlugin)
            .add_plugins(text::TextPlugin)
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
                        .run_if(|context: Res<StateTransitionContext>| !context.from_pause_menu)
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
    context: Res<StateTransitionContext>,
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

fn handle_exit(mut exit_events: EventReader<AppExit>) {
    for _exit_event in exit_events.read() {
        info!("Received exit event, cleaning up...");
    }
}

fn main() {
    // Check if we're being called with documentation commands
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "docs-build" {
            if let Err(e) = Docs::build() {
                eprintln!("Documentation build error: {}", e);
                std::process::exit(1);
            }
            std::process::exit(0);
        } else if args[1] == "docs-serve" {
            if let Err(e) = Docs::serve() {
                eprintln!("Documentation server error: {}", e);
                std::process::exit(1);
            }
            std::process::exit(0);
        } else if args[1] == "docs-check" {
            if let Err(e) = Docs::check() {
                eprintln!("Documentation check error: {}", e);
                std::process::exit(1);
            }
            std::process::exit(0);
        }
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rummage".to_string(),
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        resizable: true,
                        resize_constraints: bevy::window::WindowResizeConstraints {
                            min_width: 960.0,  // Half of 1920
                            min_height: 540.0, // Half of 1080
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::render::RenderPlugin {
                    // Configure rendering to be more resilient in WSL2 environments
                    render_creation: bevy::render::settings::RenderCreation::Automatic(
                        bevy::render::settings::WgpuSettings {
                            // Try multiple backends if needed for WSL2 compatibility
                            backends: Some(bevy::render::settings::Backends::all()),
                            // Use low power preference for better WSL2 compatibility
                            power_preference: bevy::render::settings::PowerPreference::LowPower,
                            // Don't require all features, adapt to what's available in WSL2
                            features: bevy::render::settings::WgpuFeatures::empty(),
                            ..default()
                        },
                    ),
                    // Don't wait for pipelines to compile, which can hang under certain conditions
                    synchronous_pipeline_compilation: false,
                    ..default()
                }),
        )
        .add_event::<SnapshotEvent>()
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Update, handle_exit)
        .add_systems(PostStartup, take_snapshot_after_setup)
        .run();
}

/// System to take a snapshot after the game setup is complete
fn take_snapshot_after_setup(
    mut commands: Commands,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_cameras: Query<Entity, With<camera::components::GameCamera>>,
) {
    // Get the first game camera
    if let Some(camera) = game_cameras.iter().next() {
        info!("Taking initial card layout snapshot");
        take_snapshot(
            &mut commands,
            &mut snapshot_events,
            Some(camera),
            Some("initial_card_layout".to_string()),
        );
    } else {
        error!("No game camera found for initial snapshot");
    }
}

mod camera;
mod card;
mod cards;
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
use bevy_rand::prelude::*;
use camera::{
    CameraConfig, CameraPanState,
    components::GameCamera,
    systems::{camera_movement, handle_window_resize, set_initial_zoom, setup_camera},
};
use card::{DebugConfig, debug_render_text_positions, handle_card_dragging};
use cards::CardsPlugin;
use drag::DragPlugin;
use game_engine::GameEnginePlugin;
use menu::{GameMenuState, MenuPlugin, state::StateTransitionContext};
use player::spawn_hand;
use text::spawn_card_text;

// Plugin for the actual game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DragPlugin)
            .add_plugins(EntropyPlugin::<WyRand>::default())
            .add_plugins(CardsPlugin)
            .add_plugins(GameEnginePlugin)
            .insert_resource(DebugConfig {
                show_text_positions: false,
            })
            .insert_resource(CameraConfig::default())
            .insert_resource(CameraPanState::default())
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (setup_game, set_initial_zoom.after(setup_game)),
            )
            .add_systems(
                Update,
                (
                    handle_card_dragging,
                    handle_window_resize,
                    debug_render_text_positions,
                    camera_movement,
                    spawn_card_text,
                )
                    .run_if(in_state(GameMenuState::InGame)),
            );
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_cameras: Query<Entity, With<GameCamera>>,
    context: Res<StateTransitionContext>,
) {
    info!("Setting up game environment...");

    // Skip camera setup if we're coming from the pause menu and already have a camera
    if context.from_pause_menu {
        info!("Resuming from pause menu, skipping game setup");
        // Only set up camera if needed here, but don't create cards
        if game_cameras.is_empty() {
            info!("No game camera found despite coming from pause menu, setting up camera anyway");
            setup_camera(&mut commands);
        }
    } else {
        // Normal game setup - this is a fresh game
        info!("Setting up game camera...");
        setup_camera(&mut commands);

        // Spawn the player's hand only for a new game
        info!("Spawning initial hand...");
        spawn_hand(commands, asset_server, game_cameras);
    }

    info!("Game setup complete!");
}

fn handle_exit(mut exit_events: EventReader<AppExit>) {
    for _exit_event in exit_events.read() {
        info!("Received exit event, cleaning up...");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
        }))
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Update, handle_exit)
        .run();
}

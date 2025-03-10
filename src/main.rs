mod camera;
mod card;
mod cards;
mod drag;
mod game_engine;
mod mana;
mod menu;
mod player;
mod text;

use bevy::{
    app::AppExit,
    log::info,
    prelude::*,
    window::{
        PresentMode, WindowPlugin, WindowPosition, WindowResizeConstraints, WindowResolution,
    },
};
use bevy_rand::prelude::*;
use camera::{
    components::GameCamera,
    systems::{camera_movement, handle_window_resize, safe_wsl2_resize_handler, setup_camera},
};
use card::hdr::HDRCardsPlugin;
use cards::CardsPlugin;
use drag::DragPlugin;
use game_engine::GameEnginePlugin;
use menu::{GameMenuState, MenuPlugin, StateTransitionContext};
use player::spawn_hand;
use text::spawn_card_text;

// Plugin for the actual game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources first
            .init_state::<GameMenuState>()
            .init_resource::<StateTransitionContext>()
            .insert_resource(camera::CameraConfig::default())
            .insert_resource(camera::CameraPanState::default())
            // Use Bevy's internal plugins
            .add_plugins(DragPlugin)
            .add_plugins(EntropyPlugin::<WyRand>::default())
            // Add game-specific plugins
            .add_plugins(CardsPlugin)
            .add_plugins(GameEnginePlugin)
            .add_plugins(HDRCardsPlugin)
            // Add menu plugin last to control state transitions properly
            .add_plugins(MenuPlugin)
            // Set initial game state
            .add_systems(
                Startup,
                |mut next_state: ResMut<NextState<GameMenuState>>| {
                    next_state.set(GameMenuState::MainMenu);
                },
            )
            // Add game systems
            .add_systems(OnEnter(GameMenuState::InGame), setup_game)
            .add_systems(
                Update,
                (handle_window_resize, camera_movement, spawn_card_text)
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
        info!("Exit event received. Shutting down.");
    }
}

// Initialize a panic hook to prevent application crashes
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        error!("PANIC OCCURRED: {:?}", panic_info);
    }));
}

/// Detect if we're running in WSL2 and apply special handling if needed
fn detect_wsl2() -> bool {
    if cfg!(target_os = "linux") {
        // Check for WSL-specific environment markers
        if let Ok(output) = std::process::Command::new("uname").arg("-r").output() {
            let kernel = String::from_utf8_lossy(&output.stdout);
            return kernel.contains("microsoft") || kernel.contains("WSL");
        }
    }
    false
}

fn main() {
    // Set up custom panic handling
    setup_panic_hook();

    // Detect if running under WSL2
    let is_wsl2 = detect_wsl2();
    if is_wsl2 {
        info!("WSL2 environment detected - applying WSL2-specific optimizations");
    }

    App::new()
        // Add DefaultPlugins with optimized window settings for WSL2
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rummage".into(),
                // Use a standard resolution with scale factor override to prevent WSL2 scaling issues
                resolution: WindowResolution::new(1024.0, 768.0).with_scale_factor_override(1.0),
                // Use Fifo if in WSL2, otherwise AutoNoVsync for better performance
                present_mode: if is_wsl2 {
                    // More stable in WSL2, but can be slower
                    PresentMode::Fifo
                } else {
                    // Better performance outside WSL2
                    PresentMode::AutoNoVsync
                },
                // WSL2 needs more conservative window handling
                decorations: true,
                transparent: false,
                resizable: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 640.0,
                    min_height: 480.0,
                    ..default()
                },
                position: WindowPosition::At(IVec2::new(50, 50)),
                ..default()
            }),
            ..default()
        }))
        // Process WSL2 resize handler first to prevent window issues
        .add_systems(First, safe_wsl2_resize_handler)
        // Add game plugins
        .add_plugins(GamePlugin)
        // Add exit handler
        .add_systems(Update, handle_exit)
        .run();
}

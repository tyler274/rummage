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
        // Initialize resources first to ensure they're available early
        app
            // Add basic resources before any plugins
            .init_resource::<StateTransitionContext>()
            .insert_resource(camera::CameraConfig::default())
            .insert_resource(camera::CameraPanState::default())
            // Initialize state after resources but before plugins
            .init_state::<GameMenuState>()
            // Core Bevy plugins
            .add_plugins(DragPlugin)
            .add_plugins(EntropyPlugin::<WyRand>::default())
            // Game core plugins - add these in order of dependency
            .add_plugins(CardsPlugin)
            .add_plugins(GameEnginePlugin)
            .add_plugins(HDRCardsPlugin)
            // Menu plugin added last to ensure all dependencies are available
            .add_plugins(MenuPlugin)
            // Set initial state after all plugins are loaded
            .add_systems(
                Startup,
                |mut next_state: ResMut<NextState<GameMenuState>>| {
                    next_state.set(GameMenuState::MainMenu);
                },
            )
            // Game-specific systems
            .add_systems(OnEnter(GameMenuState::InGame), setup_game)
            .add_systems(
                Update,
                (handle_window_resize, camera_movement, spawn_card_text)
                    // Only run these systems when in the game state
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

/// Plugin that adds WSL2-specific compatibility features to the application
struct WSL2CompatibilityPlugin;

impl Plugin for WSL2CompatibilityPlugin {
    fn build(&self, app: &mut App) {
        if !detect_wsl2() {
            return; // Only apply these settings in WSL2
        }

        info!("Applying WSL2 compatibility plugin");

        // Add WSL2-specific systems in the correct order
        app
            // Handle window resizing safely first
            .add_systems(First, safe_wsl2_resize_handler)
            // Ensure the app doesn't hang when the window loses focus
            .add_systems(Update, handle_exit)
            // Prevent frame freezing by using appropriate update modes
            .insert_resource(bevy::winit::WinitSettings {
                focused_mode: bevy::winit::UpdateMode::Continuous,
                unfocused_mode: bevy::winit::UpdateMode::Continuous,
            });
    }
}

fn main() {
    // Set up custom panic handling
    setup_panic_hook();

    // Detect if running under WSL2
    let is_wsl2 = detect_wsl2();
    if is_wsl2 {
        info!("WSL2 environment detected - applying WSL2-specific optimizations");
    }

    // Create the app with an explicit window configuration for WSL2 compatibility
    App::new()
        // Add DefaultPlugins with optimized window settings for WSL2
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rummage".into(),
                // Use a standard resolution with scale factor override to prevent WSL2 scaling issues
                resolution: WindowResolution::new(800.0, 600.0).with_scale_factor_override(1.0),
                // For WSL2, force Fifo (VSync) to prevent timing issues
                present_mode: PresentMode::Fifo,
                // CRITICAL: Disable decorations completely in WSL2 to avoid window border issues
                decorations: !is_wsl2,
                // Avoid alpha compositing which can cause issues in WSL2
                transparent: false,
                // Allow resizing but with constraints
                resizable: true,
                // Set reasonable constraints to prevent degenerate window sizes
                resize_constraints: WindowResizeConstraints {
                    min_width: 640.0,
                    min_height: 480.0,
                    ..default()
                },
                // Force a position to avoid window placement issues
                position: WindowPosition::At(IVec2::new(50, 50)),
                // Use reasonable defaults for everything else
                ..default()
            }),
            // Use default plugin settings
            ..default()
        }))
        // Add WSL2 compatibility plugin
        .add_plugins(WSL2CompatibilityPlugin)
        // Add game plugins
        .add_plugins(GamePlugin)
        // Add exit handler
        .add_systems(Update, handle_exit)
        .run();
}

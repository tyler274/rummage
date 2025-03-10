mod camera;
mod card;
mod cards;
mod drag;
mod game_engine;
mod mana;
mod menu;
mod player;
mod text;
mod wsl2;

use bevy::{
    app::AppExit,
    log::info,
    prelude::*,
    window::{PresentMode, WindowPlugin, WindowResolution},
};

use camera::{
    CameraConfig, CameraPanState, CameraPlugin,
    components::GameCamera,
    systems::{camera_movement, handle_window_resize, setup_camera},
};
use card::CardPlugin;
use drag::DragPlugin;
use game_engine::GameEnginePlugin;
use menu::{
    MenuPlugin,
    state::{GameMenuState, StateTransitionContext},
};
use player::spawn_hand;
use text::spawn_card_text;
use wsl2::{WSL2CompatibilityPlugin, detect_wsl2, get_wsl2_window_settings};

/// Main game plugin that sets up the game environment
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources first to ensure they're available
        app
            // Add basic resources
            .insert_resource(StateTransitionContext::default())
            .insert_resource(CameraConfig::default())
            .insert_resource(CameraPanState::default())
            // Initialize game state
            .init_state::<GameMenuState>()
            // Add core plugins
            .add_plugins(CameraPlugin)
            .add_plugins(CardPlugin)
            .add_plugins(DragPlugin)
            // Add game engine plugins
            .add_plugins(GameEnginePlugin)
            // Add menu plugin last to ensure all dependencies are available
            .add_plugins(MenuPlugin)
            // Set initial game state
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

/// Handle exit events from the application
fn handle_exit(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.read() {
        info!("Exiting application");
    }
}

/// Set up a custom panic hook to handle crashes gracefully
fn setup_panic_hook() {
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Call the default handler first
        old_hook(panic_info);
        // Then log additional information
        error!("Application panicked: {}", panic_info);
    }));
}

fn main() {
    // Set up custom panic handling
    setup_panic_hook();

    // Detect if running under WSL2
    let is_wsl2 = detect_wsl2();
    if is_wsl2 {
        info!("WSL2 environment detected - applying WSL2-specific optimizations");
    }

    // Create the app with appropriate window configuration
    let mut app = App::new();

    // Configure window settings based on environment
    if is_wsl2 {
        // Use WSL2-specific window settings
        app.add_plugins(DefaultPlugins.set(get_wsl2_window_settings()));
    } else {
        // Use standard window settings for non-WSL2 environments
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rummage".into(),
                resolution: WindowResolution::new(1024.0, 768.0),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }));
    }

    // Add WSL2 compatibility plugin
    app.add_plugins(WSL2CompatibilityPlugin)
        // Add game plugins
        .add_plugins(GamePlugin)
        // Add exit handler
        .add_systems(Update, handle_exit)
        .run();
}

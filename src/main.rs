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
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use camera::{
    CameraPanState,
    components::{AppLayer, GameCamera},
    config::CameraConfig,
    snapshot::resources::SnapshotEvent,
    systems::{camera_movement, handle_window_resize, set_initial_zoom},
};
use card::CardPlugin;
use deck::DeckPlugin;
use docs::Docs;
use drag::DragPlugin;
use game_engine::GameEnginePlugin;
use menu::{GameMenuState, MenuPlugin, state::StateTransitionContext};
use player::{PlayerPlugin, resources::PlayerConfig, spawn_players};
use std::panic;
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

// Resource to track system execution and detect potential issues
#[derive(Resource, Default)]
pub struct SystemExecutionTracker {
    pub currently_running: Vec<String>,
    pub completed_systems: Vec<String>,
    pub failed_systems: Vec<String>,
    pub startup_complete: bool,
    pub frame_count: u64,
    pub last_panic_info: Option<String>,
}

// Plugin for tracing systems execution
pub struct SystemsTracePlugin;

impl Plugin for SystemsTracePlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing Systems Trace Plugin");

        // Register panic hook to capture system panics
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // Call the previous hook
            previous_hook(panic_info);

            // Format and log the panic information
            let panic_message = format!("{}", panic_info);
            error!("PANIC DETECTED: {}", panic_message);

            // Store in a global static or similar to retrieve later
            // This is a simple approach; in a real app you might use a channel or atomic
            LAST_PANIC_MESSAGE.with(|cell| {
                *cell.borrow_mut() = Some(panic_message);
            });
        }));

        // Add systems tracking resources
        app.insert_resource(SystemTraceConfig {
            log_system_init: true,
            log_system_start: true,
            log_system_finish: true,
            log_system_panic: true,
            trace_level: Level::DEBUG,
        })
        .init_resource::<SystemExecutionTracker>()
        .add_systems(Startup, log_startup_systems)
        .add_systems(PreUpdate, log_system_initialization)
        .add_systems(PreUpdate, log_frame_start.after(log_system_initialization))
        .add_systems(Update, detect_panics)
        .add_systems(Last, log_frame_end)
        .add_systems(Last, check_for_system_panics.after(log_frame_end))
        .add_systems(First, track_main_schedule_begin)
        .add_systems(Last, track_main_schedule_end);

        info!("Systems Trace Plugin initialized");
    }
}

// Thread-local to store the last panic message
thread_local! {
    static LAST_PANIC_MESSAGE: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

// System to track the beginning of the Main schedule
fn track_main_schedule_begin() {
    debug!("Main schedule beginning");
}

// System to track the end of the Main schedule
fn track_main_schedule_end() {
    debug!("Main schedule ending");
}

// System to detect and handle panics
fn detect_panics(mut tracker: ResMut<SystemExecutionTracker>) {
    // Check if we have a new panic to process
    LAST_PANIC_MESSAGE.with(|cell| {
        if let Some(panic_msg) = cell.borrow_mut().take() {
            let frame_count = tracker.frame_count;
            error!("Processing panic from previous frame: {}", panic_msg);
            tracker.last_panic_info = Some(panic_msg);
            tracker
                .failed_systems
                .push(format!("unknown_system_panic_{}", frame_count));
        }
    });
}

// Configuration for the system tracing
#[derive(Resource)]
pub struct SystemTraceConfig {
    pub log_system_init: bool,
    pub log_system_start: bool,
    pub log_system_finish: bool,
    pub log_system_panic: bool,
    pub trace_level: Level,
}

// Log systems during startup phase
fn log_startup_systems(mut tracker: ResMut<SystemExecutionTracker>) {
    info!("=== STARTUP SYSTEMS RUNNING ===");
    debug!("Tracking startup system initialization");
    tracker.currently_running.push("startup".to_string());
}

// System to log system initialization during startup
fn log_system_initialization(world: &World, mut tracker: ResMut<SystemExecutionTracker>) {
    if !tracker.startup_complete {
        info!("=== STARTUP COMPLETE ===");
        tracker.startup_complete = true;

        // Log information about app state after startup
        let schedule_names = world
            .resource::<bevy::ecs::schedule::Schedules>()
            .iter()
            .map(|(id, _)| format!("{:?}", id))
            .collect::<Vec<_>>();

        info!("Registered schedules: {}", schedule_names.join(", "));
        debug!("System initialization check complete");
    }
}

// Log at the start of each frame
fn log_frame_start(mut tracker: ResMut<SystemExecutionTracker>) {
    tracker.frame_count += 1;
    tracker.currently_running.clear();
    tracker.completed_systems.clear();

    debug!("=== FRAME {} START ===", tracker.frame_count);
}

// Log at the end of each frame
fn log_frame_end(tracker: Res<SystemExecutionTracker>) {
    debug!("=== FRAME {} END ===", tracker.frame_count);
    if !tracker.failed_systems.is_empty() {
        warn!(
            "Failed systems in this frame: {}",
            tracker.failed_systems.join(", ")
        );
    }
}

// System to check for panics that have occurred
fn check_for_system_panics(mut tracker: ResMut<SystemExecutionTracker>) {
    if !tracker.failed_systems.is_empty() {
        error!(
            "Detected {} system failures this frame",
            tracker.failed_systems.len()
        );
        // Clear failures for next frame
        tracker.failed_systems.clear();
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
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,bevy_render=info,bevy_app=debug,rummage=debug".to_string(),
                    ..default()
                }),
        )
        .add_plugins(SystemsTracePlugin) // Add our tracing plugin
        .add_event::<SnapshotEvent>()
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Update, handle_exit)
        .add_systems(PostStartup, take_snapshot_after_setup)
        .run();
}

/// System to take a snapshot after the game setup is complete
fn take_snapshot_after_setup(
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_cameras: Query<Entity, With<camera::components::GameCamera>>,
) {
    // Get the first game camera
    if let Some(camera) = game_cameras.iter().next() {
        info!("Taking initial card layout snapshot");

        // Use the safe version of take_snapshot
        take_snapshot_safe(&mut snapshot_events, camera, "initial_card_layout");
    } else {
        error!("No game camera found for initial snapshot");
    }
}

// More robust version of take_snapshot with better error handling
fn take_snapshot_safe(
    event_writer: &mut EventWriter<SnapshotEvent>,
    camera_entity: Entity,
    description: &str,
) {
    info!(
        "Taking snapshot with camera {:?}, description: {}",
        camera_entity, description
    );

    // Send snapshot event with proper error handling
    match event_writer.send(SnapshotEvent {
        camera_entity: Some(camera_entity),
        filename: None,
        description: Some(description.to_string()),
        include_debug: Some(true),
    }) {
        _ => {
            debug!("Successfully sent snapshot event");
        }
    }

    // We don't actually need to use the returned unit value from send()
    // but we're wrapping it in match for future error handling expansion
}

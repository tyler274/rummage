#![feature(trivial_bounds)]

mod camera;
mod cards;
mod deck;
mod drag;
mod game_engine;
mod mana;
mod menu;
mod player;
mod plugins;
mod snapshot;
mod text;
mod tracing;
mod utils;

use bevy::DefaultPlugins;
use bevy::audio::{AudioPlayer, AudioPlugin, PlaybackMode, PlaybackSettings, Volume};
use bevy::log::Level;
use bevy::prelude::*;
use bevy::time::Fixed;
use bevy::window::WindowResolution;
use camera::CameraPlugin;
use menu::MenuPlugin;
use plugins::MainRummagePlugin;
#[cfg(feature = "snapshot")]
use snapshot::SnapshotDisabled;
use tracing::DiagnosticsPlugin;

// A simple component to mark the test sound
#[derive(Component)]
struct TestSound;

fn main() {
    println!("Starting Rummage application...");

    let mut app = App::new();

    // Configure the fixed timestep update rate (20 Hz)
    app.insert_resource(Time::<Fixed>::from_seconds(0.05));

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rummage".to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0)
                        .with_scale_factor_override(1.0),
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
                        // Prefer Vulkan backend for better WSL2 compatibility
                        backends: Some(bevy::render::settings::Backends::VULKAN),
                        // Use low power preference for better WSL2 compatibility
                        power_preference: bevy::render::settings::PowerPreference::LowPower,
                        // Don't require all features, adapt to what's available in WSL2
                        features: bevy::render::settings::WgpuFeatures::empty(),
                        // Add more conservative options for WSL2 compatibility
                        dx12_shader_compiler: bevy::render::settings::Dx12Compiler::Fxc,
                        ..default()
                    },
                ),
                // Don't wait for pipelines to compile, which can hang under certain conditions
                synchronous_pipeline_compilation: false,
                ..default()
            })
            .set(bevy::log::LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,bevy_render=info,bevy_app=debug,rummage=debug,khronos_egl=warn"
                    .to_string(),
                ..default()
            })
            // Explicitly configure the AudioPlugin
            .set(AudioPlugin {
                global_volume: bevy::prelude::GlobalVolume::new(1.0),
                ..default()
            }),
    )
    .add_plugins(DiagnosticsPlugin) // Add our diagnostics plugin
    .add_plugins(CameraPlugin) // Add the camera plugin which manages SnapshotEvent
    .add_plugins(MenuPlugin)
    .add_plugins(MainRummagePlugin)
    // Add a startup system to test audio
    .add_systems(Startup, test_audio)
    // Add a system to clean up the test sound after a delay
    .add_systems(Update, cleanup_test_sound);

    // Add debug logging for audio system
    info!("Audio system initialized with DefaultPlugins");

    // Add the SnapshotDisabled resource if the snapshot feature is enabled
    #[cfg(feature = "snapshot")]
    app.insert_resource(SnapshotDisabled::enabled()); // Enable snapshots

    app.add_systems(Update, utils::handle_exit).run();
}

// System to test if audio works at all
fn test_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Attempting to play test audio");

    // Use an absolute path for testing
    let file_path = "music/Negev sings Hava Nagila [XwZwz0iCuF0].ogg";
    info!("Loading audio file from path: {}", file_path);

    let test_sound = asset_server.load(file_path);

    // Spawn two audio entities with different configurations to maximize chances of working
    let entity1 = commands
        .spawn((
            AudioPlayer::new(test_sound.clone()),
            PlaybackSettings {
                mode: PlaybackMode::Loop, // Change to loop to keep playing
                volume: Volume::new(1.0),
                speed: 1.0,
                paused: false,
                ..default()
            },
            TestSound,
            Name::new("Test Sound 1 - Loop"),
        ))
        .id();

    info!("Spawned test sound entity 1: {:?}", entity1);

    // Spawn a second test sound with slightly different settings
    let entity2 = commands
        .spawn((
            AudioPlayer::new(test_sound),
            PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(1.0),
                speed: 1.0,
                paused: false,
                ..default()
            },
            TestSound,
            Name::new("Test Sound 2 - Once"),
        ))
        .id();

    info!("Spawned test sound entity 2: {:?}", entity2);
}

// System to clean up the test sound after a delay
fn cleanup_test_sound(
    mut commands: Commands,
    test_sounds: Query<(Entity, &TestSound)>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    // Initialize the timer if it's None
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(5.0, TimerMode::Once));
    }

    // Tick the timer
    if let Some(timer_ref) = timer.as_mut() {
        timer_ref.tick(time.delta());

        // If the timer finished, despawn the test sounds
        if timer_ref.just_finished() {
            info!("Cleaning up test sounds");
            for (entity, _) in test_sounds.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

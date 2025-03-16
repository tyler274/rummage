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
use bevy::audio::AudioPlugin;
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
                        // power_preference: bevy::render::settings::PowerPreference::LowPower,
                        // Don't require all features, adapt to what's available in WSL2
                        // features: bevy::render::settings::WgpuFeatures::empty(),
                        // Add more conservative options for WSL2 compatibility
                        // dx12_shader_compiler: bevy::render::settings::Dx12Compiler::Fxc,
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
    .add_plugins(MainRummagePlugin);
    // Add debug logging for audio system
    info!("Audio system initialized with DefaultPlugins");

    // Add the SnapshotDisabled resource if the snapshot feature is enabled
    #[cfg(feature = "snapshot")]
    app.insert_resource(SnapshotDisabled::enabled()); // Enable snapshots

    app.add_systems(Update, utils::handle_exit).run();
}

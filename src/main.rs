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
mod plugins;
mod text;
mod tracing;
mod utils;

use bevy::DefaultPlugins;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use camera::snapshot::resources::SnapshotEvent;
use docs::Docs;
use menu::MenuPlugin;
use plugins::GamePlugin;
use tracing::SystemsTracePlugin;
use utils::snapshot::take_snapshot_after_setup;

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
        .add_systems(Update, utils::handle_exit)
        .add_systems(PostStartup, take_snapshot_after_setup)
        .run();
}

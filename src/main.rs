mod camera;
mod card;
mod cards;
mod drag;
mod mana;
mod player;
mod text;

use bevy::DefaultPlugins;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rand::prelude::*;
use camera::{CameraConfig, camera_movement, handle_window_resize, setup_camera};
use card::{DebugConfig, debug_render_text_positions, handle_card_dragging};
use cards::CardsPlugin;
use drag::DragPlugin;
use player::spawn_hand;
use text::spawn_card_text;

fn hello_world() {
    println!("hello world!");
    println!("Mana default color is: {:?}", mana::Mana::default());
}

fn handle_exit(mut exit_events: EventReader<AppExit>) {
    for _exit_event in exit_events.read() {
        println!("Received exit event, cleaning up...");
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
        .add_plugins(DragPlugin)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(CardsPlugin)
        .insert_resource(DebugConfig {
            show_text_positions: false, // Set to false to disable debug rendering
        })
        .insert_resource(CameraConfig::default())
        .add_systems(
            Startup,
            (
                hello_world,
                setup_camera,
                spawn_hand,
                spawn_card_text.after(spawn_hand),
            ),
        )
        .add_systems(
            Update,
            (
                handle_card_dragging,
                handle_window_resize,
                handle_exit,
                debug_render_text_positions,
                camera_movement,
            ),
        )
        .run();
}

mod camera;
mod card;
mod cards;
mod drag;
mod mana;
mod player;
mod text;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_turborand::prelude::*;
use camera::{handle_window_resize, setup_camera};
use card::{debug_render_text_positions, handle_card_dragging, DebugConfig};
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

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Transform::default()));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rummage".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DragPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(CardsPlugin)
        .insert_resource(DebugConfig {
            show_text_positions: false, // Set to false to disable debug rendering
        })
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
            ),
        )
        .run();
}

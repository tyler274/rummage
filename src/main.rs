mod card;
mod cards;
mod mana;
mod player;

use bevy::prelude::*;
use bevy::render as bevy_render;
use mana::Mana;

fn hello_world() {
    println!("hello world!");
    println!("Mana default color is: {:?}", Mana::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::render::RenderPlugin {
            render_creation: bevy_render::settings::RenderCreation::Automatic(
                bevy::render::settings::WgpuSettings {
                    backends: Some(bevy::render::settings::Backends::VULKAN),
                    ..default()
                },
            ),
            ..default()
        }))
        .add_systems(Update, hello_world)
        .run();
}

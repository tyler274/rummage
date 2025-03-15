mod game_plugin;

use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::game_engine::save::SaveLoadPlugin;
use crate::player::PlayerPlugin;

pub struct MainRummagePlugin;

impl Plugin for MainRummagePlugin {
    fn build(&self, app: &mut App) {
        // Add Camera Plugin
        app.add_plugins(CameraPlugin);

        // Add Player Plugin
        app.add_plugins(PlayerPlugin);

        // Save/Load system
        app.add_plugins(SaveLoadPlugin);

        // Add your additional plugins here
    }
}

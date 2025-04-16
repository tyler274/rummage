use bevy::prelude::*;

use crate::camera::components::GameCamera;
use crate::camera::systems::setup_camera;

pub(super) fn setup_game_camera(commands: Commands, game_cameras: Query<Entity, With<GameCamera>>) {
    // Check if a game camera already exists
    if !game_cameras.is_empty() {
        info!("Game camera already exists, not creating a new one");
        return;
    }

    info!("No game camera found, creating a new one for the game state");

    // Call the camera module's setup system directly
    setup_camera(commands);
}

pub(super) fn ensure_game_camera_visible(
    mut game_camera_query: Query<&mut Visibility, With<GameCamera>>,
) {
    if game_camera_query.is_empty() {
        error!("No game camera found when entering game state!");
        return;
    }

    for mut visibility in game_camera_query.iter_mut() {
        *visibility = Visibility::Visible;
        info!("Ensuring game camera is visible for card rendering");
    }
}

use crate::camera::components::{AppLayer, MenuCamera};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub fn setup_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 2,
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        MenuCamera,
        AppLayer::Menu.with_shared(), // Menu camera can see Menu and Shared layers
    ));
}

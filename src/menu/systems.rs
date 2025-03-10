use crate::camera::components::MenuCamera;
use bevy::prelude::*;

pub fn setup_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera::default(),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        MenuCamera,
    ));
}

use bevy::prelude::*;
use bevy::ui::{PositionType, Val};

use crate::menu::components::{MenuBackground, MenuCamera, MenuItem, MenuRoot};

/// Sets up the main menu background
pub fn setup_menu_background(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    window: Query<&Window>,
) {
    // Get window dimensions to set appropriate background size
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    info!(
        "Setting up menu background with window dimensions: {}x{}",
        width, height
    );

    // Find the menu camera to attach the background to
    if let Some(camera) = menu_cameras.iter().next() {
        info!("Found menu camera for background: {:?}", camera);

        // Create and attach background to camera
        commands.entity(camera).with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(width),
                    height: Val::Px(height),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
                Name::new("Menu Background"),
                MenuBackground,
                MenuItem,
                GlobalZIndex(-10),
            ));
        });

        info!("Menu background attached to camera entity");
    } else {
        warn!("No menu camera found, creating standalone background");

        // Create a standalone background
        commands.spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            Name::new("Menu Background"),
            MenuBackground,
            MenuRoot,
            MenuItem,
            GlobalZIndex(-10),
        ));

        info!("Created standalone menu background");
    }
}

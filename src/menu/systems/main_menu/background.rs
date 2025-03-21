use bevy::prelude::*;
use bevy::ui::{PositionType, Val};

use crate::menu::components::{MenuBackground, MenuItem};

/// Sets up the menu background with starry pattern
pub fn setup_menu_background(mut commands: Commands, asset_server: &AssetServer) {
    info!("Setting up menu background");

    // Full screen background
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 1.0)),
        ImageNode::new(asset_server.load("textures/menu_background.png")),
        MenuBackground,
        MenuItem,
        Name::new("Menu Background"),
    ));
}

/// Updates the background color based on menu state
pub fn update_background(
    mut background_query: Query<&mut BackgroundColor, With<MenuBackground>>,
    time: Res<Time>,
) {
    // Create subtle color animation for the background
    for mut background in background_query.iter_mut() {
        let t = (time.elapsed_secs_f64() * 0.1).sin() * 0.5 + 0.5;
        background.0 = Color::srgba(
            0.05 + t as f32 * 0.03,
            0.05 + t as f32 * 0.02,
            0.10 + t as f32 * 0.05,
            1.0,
        );
    }
}

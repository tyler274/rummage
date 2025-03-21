use crate::menu::{
    backgrounds::components::MenuBackground, visibility::components::PreviousWindowSize,
};
use bevy::prelude::*;

/// Setup the menu background
pub fn setup_menu_background(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex::default(), // Ensure background is drawn at appropriate level
        BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
        MenuBackground,
        PreviousWindowSize::default(),
        Name::new("Menu Background"),
    ));

    debug!("Menu background setup complete");
}

/// Update the background appearance dynamically
pub fn update_background(
    backgrounds: Query<&BackgroundColor, (With<MenuBackground>, Changed<BackgroundColor>)>,
) {
    if !backgrounds.is_empty() {
        debug!("Menu background appearance updated");
    }
}

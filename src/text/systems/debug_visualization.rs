use bevy::prelude::*;

use crate::text::components::CardTextType;

/// Spawn debug visualization for text positions
pub fn spawn_debug_visualization(
    commands: &mut Commands,
    card_pos: Vec2,
    card_size: Vec2,
    _asset_server: &AssetServer,
) -> Entity {
    let debug_pos = card_pos;
    let debug_size = card_size * 0.9;

    // Create a simple sprite for debug visualization
    let entity = commands
        .spawn((
            // Use Sprite component for visualization
            Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.2), // Semi-transparent red
                custom_size: Some(debug_size),
                ..default()
            },
            // Add transform for positioning - use z=5 to ensure it's below the text but above the card
            Transform::from_translation(debug_pos.extend(5.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::Debug,
            // Add a name for debugging
            Name::new("Debug Visualization"),
        ))
        .id();

    entity
}

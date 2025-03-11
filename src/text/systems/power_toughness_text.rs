use bevy::prelude::*;

use crate::text::{
    components::{CardTextType, TextLayoutInfo},
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn power/toughness text for a card
pub fn spawn_power_toughness_text(
    commands: &mut Commands,
    pt: &str,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();
    
    // Calculate relative position offsets
    let horizontal_offset = layout.pt_x_offset;
    let vertical_offset = layout.pt_y_offset;
    
    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );
    
    let text_size = calculate_text_size(
        card_size,
        layout.pt_width,
        layout.pt_height,
    );
    
    let font_size = get_card_font_size(card_size, 24.0);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(pt.to_string()),
            // Use a relative transform instead of absolute world position
            // The z value is relative to the parent (card)
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Right),
            // Add our custom components
            CardTextType::PowerToughness,
            TextLayoutInfo {
                position: card_pos + local_offset, // Store absolute position for reference
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("P/T: {}", pt)),
        ))
        .id();

    entity
}

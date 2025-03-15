use bevy::prelude::*;

use crate::text::{
    components::{CardTextType, TextLayoutInfo},
    layout::CardTextLayout,
};

/// Spawn debug visualization for text boundaries
#[allow(dead_code)]
pub fn spawn_debug_visualization(
    commands: &mut Commands,
    _card_pos: Vec2,
    card_size: Vec2,
    _asset_server: &AssetServer,
) -> Entity {
    let layout = CardTextLayout::default();

    // Make sure parent entity has required components for proper hierarchy
    let parent_entity = commands
        .spawn((
            CardTextType::Debug,
            // Add required transform components for correct hierarchy
            Transform::default(),
            GlobalTransform::default(),
            // Add required visibility components for inheritance
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    // Visualize name text area
    let name_offset = Vec2::new(
        card_size.x * layout.name_x_offset,
        card_size.y * layout.name_y_offset,
    );
    let name_size = Vec2::new(
        card_size.x * layout.name_width,
        card_size.y * 0.08, // Approximate height for name
    );
    spawn_debug_box(
        commands,
        name_offset,
        name_size,
        Color::srgba(1.0, 0.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize mana cost text area
    let mana_offset = Vec2::new(
        card_size.x * layout.mana_cost_x_offset,
        card_size.y * layout.mana_cost_y_offset,
    );
    let mana_size = Vec2::new(
        card_size.x * 0.15, // Approximate width for mana cost
        card_size.y * 0.08, // Approximate height for mana cost
    );
    spawn_debug_box(
        commands,
        mana_offset,
        mana_size,
        Color::srgba(0.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    // Rules text area visualization replaced with simpler version
    let rules_offset = Vec2::new(
        0.0, 0.0, // Center of card
    );
    let rules_size = Vec2::new(
        card_size.x * 0.8, // 80% of card width
        card_size.y * 0.4, // 40% of card height
    );
    spawn_debug_box(
        commands,
        rules_offset,
        rules_size,
        Color::srgba(0.0, 0.0, 1.0, 0.2),
        parent_entity,
    );

    // P/T box visualization simplified
    let pt_offset = Vec2::new(
        card_size.x * 0.3,  // Bottom right
        card_size.y * -0.4, // Bottom right
    );
    let pt_size = Vec2::new(
        card_size.x * 0.15, // Small box
        card_size.y * 0.08, // Small box
    );
    spawn_debug_box(
        commands,
        pt_offset,
        pt_size,
        Color::srgba(1.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    parent_entity
}

/// Helper function to spawn a debug box
#[allow(dead_code)]
fn spawn_debug_box(
    commands: &mut Commands,
    local_offset: Vec2,
    size: Vec2,
    color: Color,
    parent: Entity,
) {
    let entity = commands
        .spawn((
            Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            // Use relative transform
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.05)),
            GlobalTransform::default(),
            CardTextType::Debug,
            TextLayoutInfo {
                alignment: JustifyText::Center,
            },
        ))
        .id();

    commands.entity(parent).add_child(entity);

    // Spawn a dot to mark the center point
    let dot_entity = commands
        .spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(3.0, 3.0)),
                ..default()
            },
            // Use relative transform
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.06)),
            GlobalTransform::default(),
            CardTextType::Debug,
        ))
        .id();

    commands.entity(parent).add_child(dot_entity);
}

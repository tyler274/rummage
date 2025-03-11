use bevy::prelude::*;

use crate::text::{
    components::{CardTextType, TextLayoutInfo},
    utils::{calculate_text_size, get_card_layout},
};

/// Spawn debug visualization for text boundaries
pub fn spawn_debug_visualization(
    commands: &mut Commands,
    card_pos: Vec2,
    card_size: Vec2,
    _asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();
    let parent_entity = commands.spawn(CardTextType::Debug).id();

    // Visualize name text area
    let name_offset = Vec2::new(
        card_size.x * layout.name_x_offset,
        card_size.y * layout.title_y_offset,
    );
    let name_size = calculate_text_size(card_size, layout.name_width, layout.title_height);
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
        card_size.y * layout.title_y_offset,
    );
    let mana_size = calculate_text_size(card_size, layout.mana_cost_width, layout.title_height);
    spawn_debug_box(
        commands,
        mana_offset,
        mana_size,
        Color::srgba(0.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize type line text area
    let type_offset = Vec2::new(0.0, card_size.y * layout.type_line_y_offset);
    let type_size = calculate_text_size(card_size, layout.type_line_width, layout.type_line_height);
    spawn_debug_box(
        commands,
        type_offset,
        type_size,
        Color::srgba(0.0, 0.0, 1.0, 0.2),
        parent_entity,
    );

    // Visualize rules text area - outer box (full text box)
    let rules_offset = Vec2::new(0.0, card_size.y * layout.text_box_y_offset);
    let rules_size = calculate_text_size(card_size, layout.text_box_width, layout.text_box_height);
    spawn_debug_box(
        commands,
        rules_offset,
        rules_size,
        Color::srgba(1.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize rules text area - inner box (with padding)
    let rules_inner_offset = Vec2::new(0.0, card_size.y * layout.text_box_y_offset);
    let rules_inner_size = calculate_text_size(
        card_size,
        layout.text_box_width - (layout.text_box_padding * 2.0),
        layout.text_box_height - (layout.text_box_padding * 2.0),
    );
    spawn_debug_box(
        commands,
        rules_inner_offset,
        rules_inner_size,
        Color::srgba(0.0, 0.5, 0.5, 0.3),
        parent_entity,
    );

    // Visualize power/toughness text area
    let pt_offset = Vec2::new(
        card_size.x * layout.pt_x_offset,
        card_size.y * layout.pt_y_offset,
    );
    let pt_size = calculate_text_size(card_size, layout.pt_width, layout.pt_height);
    spawn_debug_box(
        commands,
        pt_offset,
        pt_size,
        Color::srgba(1.0, 0.0, 1.0, 0.2),
        parent_entity,
    );

    // Return the parent entity
    parent_entity
}

/// Helper function to spawn a debug box
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
                position: local_offset, // Store local position
                size,
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

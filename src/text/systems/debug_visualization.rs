use bevy::prelude::*;

use crate::text::{
    components::{CardTextType, TextLayoutInfo},
    utils::{calculate_text_position, calculate_text_size, get_card_layout},
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
    let name_pos = calculate_text_position(
        card_pos,
        card_size,
        layout.name_x_offset,
        layout.title_y_offset,
    );
    let name_size = calculate_text_size(card_size, layout.name_width, layout.title_height);
    spawn_debug_box(
        commands,
        name_pos,
        name_size,
        Color::srgba(1.0, 0.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize mana cost text area
    let mana_pos = calculate_text_position(
        card_pos,
        card_size,
        layout.mana_cost_x_offset,
        layout.title_y_offset,
    );
    let mana_size = calculate_text_size(card_size, layout.mana_cost_width, layout.title_height);
    spawn_debug_box(
        commands,
        mana_pos,
        mana_size,
        Color::srgba(0.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize type line text area
    let type_pos = calculate_text_position(card_pos, card_size, 0.0, layout.type_line_y_offset);
    let type_size = calculate_text_size(card_size, layout.type_line_width, layout.type_line_height);
    spawn_debug_box(
        commands,
        type_pos,
        type_size,
        Color::srgba(0.0, 0.0, 1.0, 0.2),
        parent_entity,
    );

    // Visualize rules text area
    let rules_pos = calculate_text_position(card_pos, card_size, 0.0, layout.text_box_y_offset);
    let rules_size = calculate_text_size(card_size, layout.text_box_width, layout.text_box_height);
    spawn_debug_box(
        commands,
        rules_pos,
        rules_size,
        Color::srgba(1.0, 1.0, 0.0, 0.2),
        parent_entity,
    );

    // Visualize power/toughness text area
    let pt_pos =
        calculate_text_position(card_pos, card_size, layout.pt_x_offset, layout.pt_y_offset);
    let pt_size = calculate_text_size(card_size, layout.pt_width, layout.pt_height);
    spawn_debug_box(
        commands,
        pt_pos,
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
    position: Vec2,
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
            Transform::from_translation(position.extend(6.0)),
            GlobalTransform::default(),
            CardTextType::Debug,
            TextLayoutInfo {
                position,
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
            Transform::from_translation(position.extend(7.0)),
            GlobalTransform::default(),
            CardTextType::Debug,
        ))
        .id();

    commands.entity(parent).add_child(dot_entity);
}

use crate::text::components::CardTextType;
use bevy::prelude::*;

// Re-export CardTextLayout and utility functions from layout module
pub use crate::text::layout::{
    CardTextLayout, calculate_text_size, get_adaptive_font_size, get_card_layout,
};

/// Spawn debug bounds visualization for text
#[allow(dead_code)]
pub fn spawn_debug_bounds(
    commands: &mut Commands,
    transform: Transform,
    size: Vec2,
    parent: Option<Entity>,
) {
    // Create a small square at each corner of the text area
    // This helps visualize the text boundaries during development
    let corner_size = 5.0;
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;

    let corners = [
        // Top-left
        (
            Vec3::new(-half_width, half_height, 0.0),
            Color::srgb(1.0, 0.0, 0.0),
        ), // Red
        // Top-right
        (
            Vec3::new(half_width, half_height, 0.0),
            Color::srgb(0.0, 1.0, 0.0),
        ), // Green
        // Bottom-right
        (
            Vec3::new(half_width, -half_height, 0.0),
            Color::srgb(0.0, 0.0, 1.0),
        ), // Blue
        // Bottom-left
        (
            Vec3::new(-half_width, -half_height, 0.0),
            Color::srgb(1.0, 1.0, 0.0),
        ), // Yellow
    ];

    for (corner_pos, color) in corners.iter() {
        let corner_entity = commands
            .spawn((
                Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(corner_size, corner_size)),
                    ..Default::default()
                },
                Transform::from_translation(*corner_pos),
                CardTextType::Debug,
            ))
            .id();

        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(corner_entity);
        }
    }

    // Draw bounding box outline
    let outline_entity = commands
        .spawn((
            Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 0.3),
                custom_size: Some(size),
                ..Default::default()
            },
            transform,
            CardTextType::Debug,
        ))
        .id();

    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(outline_entity);
    }
}

/// Create a rectangular sprite for debugging
#[allow(dead_code)]
pub fn spawn_debug_rect(commands: &mut Commands, size: Vec2, color: Color, z_layer: f32) -> Entity {
    commands
        .spawn((
            Sprite {
                color,
                custom_size: Some(size),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, z_layer)),
            CardTextType::Debug,
        ))
        .id()
}

/// Calculate text position relative to a card
///
/// This function calculates the position of text elements relative to a card,
/// ensuring that the text stays properly aligned with the card regardless of
/// the card's position on screen.
///
/// Instead of using absolute world coordinates, we use the card's transform
/// as the parent and calculate relative offsets. This ensures that text
/// stays properly aligned with cards even when they're away from the center
/// of the screen.
#[allow(dead_code)]
pub fn calculate_text_position(
    card_pos: Vec2,
    card_size: Vec2,
    horizontal_offset: f32,
    vertical_offset: f32,
) -> Vec2 {
    // Calculate the offset in local card space
    let offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    // Apply the offset to the card position
    card_pos + offset
}

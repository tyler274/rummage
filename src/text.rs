use crate::card::{Card, CardTextContent, CardTextType, DebugConfig, SpawnedText};
use bevy::prelude::*;
use bevy::text::{JustifyText, Text2d, TextBounds, TextColor, TextFont, TextLayout};

/// Spawns debug visualization markers for card and text positions
///
/// # Debug Visualization Colors
/// - Red dots: Card center points
/// - Green dots: Text anchor points
/// - Blue dots (in drag.rs): Current drag position
///
/// This visualization helped identify several issues:
/// 1. Text offset calculations were initially incorrect
/// 2. Camera projection was affecting text positioning
/// 3. Parent-child transforms needed proper z-ordering
fn spawn_debug_bounds(commands: &mut Commands, card_pos: Vec2, _card_size: Vec2, text_pos: Vec2) {
    // Card center marker (red)
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(card_pos.x, card_pos.y, 100.0),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));

    // Text position marker (green)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(text_pos.x, text_pos.y, 100.0),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

/// Spawns text components for cards using relative transforms.
/// Text entities are created as children of card entities to maintain proper positioning.
///
/// # Text Positioning Evolution
/// 1. Initial Approach (Failed):
///    - Used absolute world coordinates
///    - Text position calculated manually during drag
///    - Led to positioning inconsistencies
///
/// 2. Screen Space Attempt (Failed):
///    - Tried converting between screen and world space
///    - Issues with camera projection and scaling
///    - Text appeared correct only in screen center
///
/// 3. Final Solution (Working):
///    - Text entities as children of card entities
///    - Relative transforms from card center
///    - Automatic position updates through parent-child relationship
///    - Consistent spacing regardless of screen position
///
/// # Text Layout
/// - Card Name: Centered at top
/// - Mana Cost: Top left corner
/// - Type Line: Center
/// - Power/Toughness: Bottom right
pub fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<
        (Entity, &CardTextContent, &Parent),
        (Without<SpawnedText>, With<CardTextContent>),
    >,
    card_query: Query<(&Transform, &Sprite), With<Card>>,
    asset_server: Res<AssetServer>,
    debug_config: Res<DebugConfig>,
) {
    let regular_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let emoji_font: Handle<Font> = asset_server.load("fonts/NotoColorEmoji.ttf");

    for (content_entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();

        if let Ok((card_transform, sprite)) = card_query.get(parent_entity) {
            let card_size = sprite.custom_size.unwrap_or(Vec2::new(100.0, 140.0));

            // Calculate relative offsets from card center
            let (offset, font_size, alignment, bounds) = match content.text_type {
                CardTextType::Name => (
                    Vec3::new(card_size.x * -0.05, card_size.y * 0.30, 1.0),
                    card_size.y * 0.09, // Scale font with card height for consistent proportions
                    JustifyText::Left,
                    Some(Vec2::new(card_size.x * 0.70, card_size.y * 0.5)), // Wide bounds for name wrapping
                ),
                CardTextType::Cost => (
                    Vec3::new(card_size.x * 0.32, card_size.y * 0.45, 1.0),
                    card_size.y * 0.06, // Larger size for mana symbols to improve visibility
                    JustifyText::Right,
                    None,
                ),
                CardTextType::Type => (
                    Vec3::new(-card_size.x * 0.10, card_size.y * 0.1, 1.0),
                    card_size.y * 0.045, // Slightly smaller for type line
                    JustifyText::Left,
                    Some(Vec2::new(card_size.x * 0.8, card_size.y * 0.5)),
                ),
                CardTextType::PowerToughness => (
                    Vec3::new(card_size.x * 0.35, -card_size.y * 0.46, 1.0),
                    card_size.y * 0.05, // Match name size for consistency
                    JustifyText::Right,
                    None,
                ),
                CardTextType::RulesText => (
                    Vec3::new(-card_size.x * 0.0, -card_size.y * 0.15, 1.0), // Centered horizontally but with left margin
                    card_size.y * 0.045,
                    JustifyText::Left, // Keep left justification for rules text
                    Some(Vec2::new(card_size.x * 0.80, card_size.y * 0.45)), // Slightly narrower bounds
                ),
            };

            // Create text entity with relative transform
            let text_entity = commands
                .spawn((
                    Text2d::new(content.text.clone()),
                    TextFont {
                        font: if content.text_type == CardTextType::Cost {
                            emoji_font.clone() // Use emoji font for mana symbols
                        } else {
                            regular_font.clone()
                        },
                        font_size,
                        ..default()
                    },
                    TextColor(if content.text_type == CardTextType::Cost {
                        Color::WHITE // White for better contrast
                    } else {
                        Color::BLACK
                    }),
                    TextLayout::new_with_justify(alignment),
                    Transform::from_translation(offset),
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    SpawnedText,
                ))
                .id();

            // Set up parent-child relationship
            commands.entity(parent_entity).add_child(text_entity);
            commands.entity(content_entity).insert(SpawnedText);

            // Add debug visualization only if enabled
            if debug_config.show_text_positions {
                spawn_debug_bounds(
                    &mut commands,
                    card_transform.translation.truncate(),
                    card_size,
                    card_transform.translation.truncate() + offset.truncate(),
                );
            }

            // Add text bounds if specified
            if let Some(bounds) = bounds {
                commands
                    .entity(text_entity)
                    .insert(TextBounds::from(bounds));
            }
        }
    }
}

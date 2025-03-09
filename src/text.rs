/// Text rendering and layout for Magic: The Gathering cards.
///
/// This module provides:
/// - Card text positioning and layout
/// - Text component management
/// - Font handling and scaling
/// - Debug visualization for text positions
///
/// # Important Note for Bevy 0.15.x Compatibility
/// As of Bevy 0.15.x, all *Bundle types (Text2dBundle, SpriteBundle, etc.) are deprecated.
/// Instead, spawn entities with individual components:
/// ```ignore
/// // OLD (deprecated):
/// commands.spawn(Text2dBundle { ... });
///
/// // NEW (correct):
/// commands.spawn((
///     Text2d::new("text"),
///     TextFont { ... },
///     TextColor(...),
///     TextLayout::default(),
///     Transform::from_xyz(...),
///     GlobalTransform::default(),
///     Visibility::Visible,
///     ViewVisibility::default(),
/// ));
/// ```
///
/// # Text Layout Strategy
/// - Card names use a two-line layout with:
///   - 70% card width bounds to encourage wrapping
///   - 20% card height to fit two lines
///   - Left-justified text for consistent alignment
///   - Positioned near top-left of card
/// - Other text elements (cost, type, rules) use single-line layouts
///   with specific positioning for each type
use crate::card::{Card, CardTextContent, CardTextType, DebugConfig, SpawnedText};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{Text2d, TextBounds};

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
/// - Mana Cost: Top right corner
/// - Type Line: Center
/// - Power/Toughness: Bottom right
/// - Rules Text: Center body
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
    let mana_font: Handle<Font> = asset_server.load("fonts/mana.ttf");

    for (content_entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();

        if let Ok((card_transform, sprite)) = card_query.get(parent_entity) {
            let card_size = sprite.custom_size.unwrap_or(Vec2::new(100.0, 140.0));

            // Calculate relative offsets from card center
            let (offset, font_size, _anchor) = match content.text_type {
                CardTextType::Name => (
                    Vec3::new(-card_size.x * 0.15, card_size.y * 0.35, 1.0), // Moved up slightly to accommodate two lines
                    card_size.y * 0.07, // Slightly smaller font to fit two lines
                    Anchor::TopLeft,
                ),
                CardTextType::Cost => (
                    Vec3::new(card_size.x * 0.32, card_size.y * 0.45, 1.0),
                    card_size.y * 0.06,
                    Anchor::CenterRight,
                ),
                CardTextType::Type => (
                    Vec3::new(-card_size.x * 0.10, card_size.y * 0.1, 1.0),
                    card_size.y * 0.045,
                    Anchor::CenterLeft,
                ),
                CardTextType::PowerToughness => (
                    Vec3::new(card_size.x * 0.35, -card_size.y * 0.46, 1.0),
                    card_size.y * 0.05,
                    Anchor::CenterRight,
                ),
                CardTextType::RulesText => (
                    Vec3::new(-card_size.x * 0.0, -card_size.y * 0.15, 1.0),
                    card_size.y * 0.045,
                    Anchor::CenterLeft,
                ),
            };

            // Create font and color settings
            let font = if content.text_type == CardTextType::Cost {
                mana_font.clone()
            } else {
                regular_font.clone()
            };

            let color = if content.text_type == CardTextType::Cost {
                Color::WHITE
            } else {
                Color::BLACK
            };

            // Create text layout based on type
            let text_layout = match content.text_type {
                CardTextType::Name => TextLayout::new_with_justify(JustifyText::Left),
                _ => TextLayout::default(),
            };

            // Create text entity with relative transform
            let text_entity = commands
                .spawn((
                    // Core text components
                    Text2d::new(content.text.clone()),
                    TextFont {
                        font,
                        font_size,
                        ..default()
                    },
                    TextColor(color),
                    text_layout,
                    TextBounds {
                        width: match content.text_type {
                            CardTextType::RulesText => Some(card_size.x * 0.8),
                            CardTextType::Type => Some(card_size.x * 0.8),
                            CardTextType::Name => Some(card_size.x * 0.7), // Narrower width to force wrapping
                            _ => None,
                        },
                        height: match content.text_type {
                            CardTextType::RulesText => Some(card_size.y * 0.3),
                            CardTextType::Type => Some(card_size.y * 0.1),
                            CardTextType::Name => Some(card_size.y * 0.2), // Taller height to accommodate two lines
                            _ => None,
                        },
                    },
                    // Transform components
                    Transform::from_translation(offset),
                    GlobalTransform::default(),
                    // Visibility components
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    SpawnedText,
                ))
                .id();

            // For mana costs, add a dark background to make white text visible
            if content.text_type == CardTextType::Cost {
                commands
                    .spawn((
                        Sprite {
                            color: Color::srgb(0.1, 0.1, 0.1),
                            custom_size: Some(Vec2::new(card_size.x * 0.15, card_size.y * 0.08)),
                            ..default()
                        },
                        Transform::from_translation(offset - Vec3::new(0.0, 0.0, 0.1)), // Slightly behind text
                        GlobalTransform::default(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ))
                    .set_parent(parent_entity);
            }

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
        }
    }
}

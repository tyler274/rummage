use bevy::prelude::*;

use crate::card::Card;
use crate::text::{
    components::{CardTextContent, CardTextType, DebugConfig, SpawnedText, TextLayoutInfo},
    utils::spawn_debug_bounds,
};

/// System to spawn text for cards
pub fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<
        (Entity, &CardTextContent, &Parent),
        (Without<SpawnedText>, With<CardTextContent>),
    >,
    card_query: Query<(&Transform, &Sprite), With<Card>>,
    asset_server: Res<AssetServer>,
    debug_config: Option<Res<DebugConfig>>,
) {
    for (entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();
        if let Ok((card_transform, card_sprite)) = card_query.get(parent_entity) {
            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Mark this entity as having spawned text
            commands.entity(entity).insert(SpawnedText);

            // Spawn name text
            let name_entity =
                spawn_name_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(parent_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity =
                spawn_mana_cost_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(parent_entity).add_child(mana_cost_entity);

            // Spawn type line text
            let type_line_entity =
                spawn_type_line_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(parent_entity).add_child(type_line_entity);

            // Spawn rules text
            let rules_text_entity =
                spawn_rules_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(parent_entity).add_child(rules_text_entity);

            // Spawn power/toughness text if applicable
            if let Some(pt) = &content.power_toughness {
                let pt_entity = spawn_power_toughness_text(
                    &mut commands,
                    pt,
                    card_pos,
                    card_size,
                    &asset_server,
                );
                commands.entity(parent_entity).add_child(pt_entity);
            }

            // Spawn debug visualization if enabled
            if let Some(debug_config) = &debug_config {
                if debug_config.show_text_positions {
                    let debug_entity = spawn_debug_visualization(
                        &mut commands,
                        card_pos,
                        card_size,
                        &asset_server,
                    );
                    commands.entity(parent_entity).add_child(debug_entity);
                }
            }
        }
    }
}

/// Spawn name text for a card
fn spawn_name_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let text_pos = card_pos + Vec2::new(-card_size.x * 0.3, card_size.y * 0.4);
    let text_size = Vec2::new(card_size.x * 0.7, card_size.y * 0.2);

    let entity = commands
        .spawn((
            Text::new(content.name.clone()),
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Left),
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::Name,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Left,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    if content.name.len() > 20 {
        // For long names, spawn debug bounds to visualize the text area
        let debug_entity = spawn_debug_bounds(commands, card_pos, card_size, text_pos);
        commands.entity(entity).add_child(debug_entity);
    }

    entity
}

/// Spawn mana cost text for a card
fn spawn_mana_cost_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let text_pos = card_pos + Vec2::new(card_size.x * 0.3, card_size.y * 0.4);

    let entity = commands
        .spawn((
            Text::new(content.mana_cost.clone()),
            TextFont {
                font: asset_server.load("fonts/Mana.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_no_wrap(),
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::ManaCost,
            TextLayoutInfo {
                position: text_pos,
                size: Vec2::new(card_size.x * 0.3, card_size.y * 0.1),
                alignment: JustifyText::Right,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    entity
}

/// Spawn type line text for a card
fn spawn_type_line_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let text_pos = card_pos + Vec2::new(0.0, card_size.y * 0.1);

    let entity = commands
        .spawn((
            Text::new(content.type_line.clone()),
            TextFont {
                font: asset_server.load("fonts/DejaVuSans.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_no_wrap(),
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::TypeLine,
            TextLayoutInfo {
                position: text_pos,
                size: Vec2::new(card_size.x * 0.8, card_size.y * 0.1),
                alignment: JustifyText::Center,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    entity
}

/// Spawn rules text for a card
fn spawn_rules_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let text_pos = card_pos + Vec2::new(0.0, -card_size.y * 0.1);

    let entity = commands
        .spawn((
            Text::new(content.rules_text.clone()),
            TextFont {
                font: asset_server.load("fonts/DejaVuSans.ttf"),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Left),
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::RulesText,
            TextLayoutInfo {
                position: text_pos,
                size: Vec2::new(card_size.x * 0.8, card_size.y * 0.4),
                alignment: JustifyText::Left,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    entity
}

/// Spawn power/toughness text for a card
fn spawn_power_toughness_text(
    commands: &mut Commands,
    pt: &str,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let text_pos = card_pos + Vec2::new(card_size.x * 0.3, -card_size.y * 0.4);

    let entity = commands
        .spawn((
            Text::new(pt.to_string()),
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_no_wrap(),
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::PowerToughness,
            TextLayoutInfo {
                position: text_pos,
                size: Vec2::new(card_size.x * 0.2, card_size.y * 0.1),
                alignment: JustifyText::Right,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    entity
}

/// Spawn debug visualization for text positions
fn spawn_debug_visualization(
    commands: &mut Commands,
    card_pos: Vec2,
    card_size: Vec2,
    _asset_server: &AssetServer,
) -> Entity {
    // Spawn card boundary visualization
    let entity = commands
        .spawn((
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3),
                custom_size: Some(card_size),
                ..default()
            },
            Transform::from_translation(card_pos.extend(5.0)),
            GlobalTransform::default(),
            CardTextType::Debug,
        ))
        .id();

    // Spawn text position markers
    let positions = [
        // Name position
        (
            Vec2::new(-card_size.x * 0.3, card_size.y * 0.4),
            Color::srgb(1.0, 0.0, 0.0),
        ),
        // Mana cost position
        (
            Vec2::new(card_size.x * 0.3, card_size.y * 0.4),
            Color::srgb(0.0, 1.0, 0.0),
        ),
        // Type line position
        (
            Vec2::new(0.0, card_size.y * 0.1),
            Color::srgb(0.0, 0.0, 1.0),
        ),
        // Rules text position
        (
            Vec2::new(0.0, -card_size.y * 0.1),
            Color::srgb(1.0, 1.0, 0.0),
        ),
        // Power/toughness position
        (
            Vec2::new(card_size.x * 0.3, -card_size.y * 0.4),
            Color::srgb(1.0, 0.0, 1.0),
        ),
    ];

    for (offset, color) in positions {
        let pos = card_pos + offset;
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            Transform::from_translation(pos.extend(10.0)),
            GlobalTransform::default(),
            CardTextType::Debug,
        ));
    }

    entity
}

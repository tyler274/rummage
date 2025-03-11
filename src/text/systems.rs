use bevy::prelude::*;

use crate::card::{Card, CardDetails};
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
    card_query: Query<(Entity, &Transform, &Sprite, &Card), With<Card>>,
    asset_server: Res<AssetServer>,
    debug_config: Option<Res<DebugConfig>>,
) {
    info!(
        "Running spawn_card_text system, found {} text content entities",
        text_content_query.iter().count()
    );
    info!("Found {} card entities", card_query.iter().count());

    // Directly spawn text for all cards if no text content entities are found
    if text_content_query.iter().count() == 0 && card_query.iter().count() > 0 {
        info!("No text content entities found, spawning text directly for cards");

        for (card_entity, card_transform, card_sprite, card) in card_query.iter() {
            info!("Spawning text for card: {}", card.name);

            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Create text content
            let text_content = CardTextContent {
                name: card.name.clone(),
                mana_cost: card.cost.to_string(),
                type_line: card.type_line(),
                rules_text: card.rules_text.clone(),
                power_toughness: if let CardDetails::Creature(creature) = &card.card_details {
                    Some(format!("{}/{}", creature.power, creature.toughness))
                } else {
                    None
                },
            };

            // Spawn name text
            let name_entity = spawn_name_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(card_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity = spawn_mana_cost_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(card_entity).add_child(mana_cost_entity);

            // Spawn type line text
            let type_line_entity = spawn_type_line_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(card_entity).add_child(type_line_entity);

            // Spawn rules text
            let rules_text_entity = spawn_rules_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(card_entity).add_child(rules_text_entity);

            // Spawn power/toughness text if applicable
            if let Some(pt) = &text_content.power_toughness {
                let pt_entity = spawn_power_toughness_text(
                    &mut commands,
                    pt,
                    card_pos,
                    card_size,
                    &asset_server,
                );
                commands.entity(card_entity).add_child(pt_entity);
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
                    commands.entity(card_entity).add_child(debug_entity);
                }
            }
        }
    }

    // Process text content entities with parents
    for (text_entity, text_content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();
        info!(
            "Processing text content entity {:?} with parent {:?}",
            text_entity, parent_entity
        );

        if let Ok((card_entity, card_transform, card_sprite, _card)) = card_query.get(parent_entity)
        {
            info!(
                "Found card transform and sprite for parent entity {:?}",
                parent_entity
            );

            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Spawn name text
            let name_entity = spawn_name_text(
                &mut commands,
                text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(text_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity = spawn_mana_cost_text(
                &mut commands,
                text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(text_entity).add_child(mana_cost_entity);

            // Spawn type line text
            let type_line_entity = spawn_type_line_text(
                &mut commands,
                text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(text_entity).add_child(type_line_entity);

            // Spawn rules text
            let rules_text_entity = spawn_rules_text(
                &mut commands,
                text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(text_entity).add_child(rules_text_entity);

            // Spawn power/toughness text if it exists
            if let Some(pt) = &text_content.power_toughness {
                let pt_entity = spawn_power_toughness_text(
                    &mut commands,
                    pt,
                    card_pos,
                    card_size,
                    &asset_server,
                );
                commands.entity(text_entity).add_child(pt_entity);
            }

            // Mark as spawned
            commands.entity(text_entity).insert(SpawnedText);

            // Add debug visualization if enabled
            if let Some(debug_config) = &debug_config {
                if debug_config.show_text_positions {
                    let debug_entity = spawn_debug_visualization(
                        &mut commands,
                        card_pos,
                        card_size,
                        &asset_server,
                    );
                    commands.entity(card_entity).add_child(debug_entity);
                }
            }
        } else {
            warn!(
                "Could not find card transform and sprite for parent entity {:?}",
                parent_entity
            );
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

    // Create a simple sprite with the card name as a label
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0), // Transparent sprite
                custom_size: Some(text_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::Name,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Left,
            },
            // Add a name for debugging
            Name::new(format!("Text: {}", content.name)),
        ))
        .id();

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
    let text_size = Vec2::new(card_size.x * 0.3, card_size.y * 0.2);

    // Create a simple sprite with the mana cost as a label
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0), // Transparent sprite
                custom_size: Some(text_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::ManaCost,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("Mana Cost: {}", content.mana_cost)),
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
    let text_size = Vec2::new(card_size.x * 0.8, card_size.y * 0.1);

    // Create a simple sprite with the type line as a label
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0), // Transparent sprite
                custom_size: Some(text_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::TypeLine,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Center,
            },
            // Add a name for debugging
            Name::new(format!("Type Line: {}", content.type_line)),
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
    let text_size = Vec2::new(card_size.x * 0.8, card_size.y * 0.3);

    // Create a simple sprite with the rules text as a label
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0), // Transparent sprite
                custom_size: Some(text_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::RulesText,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Center,
            },
            // Add a name for debugging
            Name::new(format!("Rules Text: {}", content.rules_text)),
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
    let text_size = Vec2::new(card_size.x * 0.2, card_size.y * 0.1);

    // Create a simple sprite with the power/toughness as a label
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0), // Transparent sprite
                custom_size: Some(text_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::PowerToughness,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("P/T: {}", pt)),
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
    let debug_pos = card_pos;
    let debug_size = card_size * 0.9;

    // Create a simple sprite for debug visualization
    let entity = commands
        .spawn((
            // Use Sprite component directly
            Sprite {
                color: Color::rgba(1.0, 0.0, 0.0, 0.2), // Semi-transparent red
                custom_size: Some(debug_size),
                ..default()
            },
            // Add transform for positioning
            Transform::from_translation(debug_pos.extend(5.0)),
            GlobalTransform::default(),
            // Add our custom components
            CardTextType::Debug,
            Name::new("Debug Visualization"),
        ))
        .id();

    entity
}

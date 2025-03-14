use bevy::prelude::*;

use super::{
    mana_cost_text::create_mana_cost_text, name_text::create_name_text,
    power_toughness_text::spawn_power_toughness_text, rules_text::spawn_rules_text,
    type_line_text::spawn_type_line_text,
};
use crate::card::{Card, CardDetails};
use crate::text::components::{
    CardManaCostText, CardNameText, CardPowerToughness, CardRulesText, CardTextContent,
    CardTypeLine, DebugConfig, SpawnedText,
};
use crate::text::systems::debug_visualization::spawn_debug_visualization;

/// System to spawn text for cards
pub fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<
        (Entity, &CardTextContent, &Parent),
        (Without<SpawnedText>, With<CardTextContent>),
    >,
    name_query: Query<(Entity, &CardNameText, &Parent), (Without<SpawnedText>, With<CardNameText>)>,
    mana_cost_query: Query<
        (Entity, &CardManaCostText, &Parent),
        (Without<SpawnedText>, With<CardManaCostText>),
    >,
    type_line_query: Query<
        (Entity, &CardTypeLine, &Parent),
        (Without<SpawnedText>, With<CardTypeLine>),
    >,
    rules_text_query: Query<
        (Entity, &CardRulesText, &Parent),
        (Without<SpawnedText>, With<CardRulesText>),
    >,
    power_toughness_query: Query<
        (Entity, &CardPowerToughness, &Parent),
        (Without<SpawnedText>, With<CardPowerToughness>),
    >,
    card_query: Query<(Entity, &Transform, &Sprite, &Card), Without<SpawnedText>>,
    asset_server: Res<AssetServer>,
    debug_config: Option<Res<DebugConfig>>,
) {
    // Only log if we have cards to process
    if card_query.iter().count() > 0 || text_content_query.iter().count() > 0 {
        info!(
            "Running spawn_card_text system, found {} text content entities",
            text_content_query.iter().count()
        );
        info!("Found {} cards without text", card_query.iter().count());
    }

    // Spawn text for cards that don't have specialized text components yet
    if card_query.iter().count() > 0 {
        info!("Spawning text for {} cards", card_query.iter().count());

        for (card_entity, card_transform, card_sprite, card) in card_query.iter() {
            info!("Spawning text for card: {}", card.name);

            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Create text content components
            let name_component = CardNameText {
                name: card.name.clone(),
            };

            let mana_cost_component = CardManaCostText {
                mana_cost: card.cost.to_string(),
            };

            let type_line_component = CardTypeLine {
                type_line: card.type_line(),
            };

            let rules_text_component = CardRulesText {
                rules_text: card.rules_text.clone(),
            };

            // Create text content (for backward compatibility)
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
            let name_entity = create_name_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(name_entity).insert(name_component);
            commands.entity(card_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity = create_mana_cost_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands
                .entity(mana_cost_entity)
                .insert(mana_cost_component);
            commands.entity(card_entity).add_child(mana_cost_entity);

            // Spawn type line text
            let type_line_entity = spawn_type_line_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands
                .entity(type_line_entity)
                .insert(type_line_component);
            commands.entity(card_entity).add_child(type_line_entity);

            // Spawn rules text
            let rules_text_entity = spawn_rules_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands
                .entity(rules_text_entity)
                .insert(rules_text_component);
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
                commands.entity(pt_entity).insert(CardPowerToughness {
                    power_toughness: pt.clone(),
                });
                commands.entity(card_entity).add_child(pt_entity);
            }

            // Add debug visualization if enabled
            if let Some(debug_config) = debug_config.as_ref() {
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

            // Mark the card as having spawned text
            commands.entity(card_entity).insert(SpawnedText);
        }
    }

    // Process text content entities that don't have SpawnedText (legacy support)
    for (text_entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();
        if let Ok((_, card_transform, card_sprite, _)) = card_query.get(parent_entity) {
            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Spawn name text
            let name_entity =
                create_name_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(name_entity).insert(CardNameText {
                name: content.name.clone(),
            });
            commands.entity(parent_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity =
                create_mana_cost_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(mana_cost_entity).insert(CardManaCostText {
                mana_cost: content.mana_cost.clone(),
            });
            commands.entity(parent_entity).add_child(mana_cost_entity);

            // Spawn type line text
            let type_line_entity =
                spawn_type_line_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(type_line_entity).insert(CardTypeLine {
                type_line: content.type_line.clone(),
            });
            commands.entity(parent_entity).add_child(type_line_entity);

            // Spawn rules text
            let rules_text_entity =
                spawn_rules_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(rules_text_entity).insert(CardRulesText {
                rules_text: content.rules_text.clone(),
            });
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
                commands.entity(pt_entity).insert(CardPowerToughness {
                    power_toughness: pt.clone(),
                });
                commands.entity(parent_entity).add_child(pt_entity);
            }

            // Add debug visualization if enabled
            if let Some(debug_config) = debug_config.as_ref() {
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

            // Mark the text content and parent as having spawned text
            commands.entity(text_entity).insert(SpawnedText);
            commands.entity(parent_entity).insert(SpawnedText);
        } else {
            warn!(
                "Could not find card transform and sprite for parent entity {:?}",
                parent_entity
            );
        }
    }

    // Process individual specialized text components
    process_specialized_text_components(
        &mut commands,
        &name_query,
        &card_query,
        &asset_server,
        create_name_text,
        |_entity, component| {
            let content = CardTextContent {
                name: component.name.clone(),
                mana_cost: String::new(),
                type_line: String::new(),
                rules_text: String::new(),
                power_toughness: None,
            };
            (content, SpawnedText)
        },
        debug_config.as_deref(),
    );

    process_specialized_text_components(
        &mut commands,
        &mana_cost_query,
        &card_query,
        &asset_server,
        create_mana_cost_text,
        |_entity, component| {
            let content = CardTextContent {
                name: String::new(),
                mana_cost: component.mana_cost.clone(),
                type_line: String::new(),
                rules_text: String::new(),
                power_toughness: None,
            };
            (content, SpawnedText)
        },
        debug_config.as_deref(),
    );

    process_specialized_text_components(
        &mut commands,
        &type_line_query,
        &card_query,
        &asset_server,
        spawn_type_line_text,
        |_entity, component| {
            let content = CardTextContent {
                name: String::new(),
                mana_cost: String::new(),
                type_line: component.type_line.clone(),
                rules_text: String::new(),
                power_toughness: None,
            };
            (content, SpawnedText)
        },
        debug_config.as_deref(),
    );

    process_specialized_text_components(
        &mut commands,
        &rules_text_query,
        &card_query,
        &asset_server,
        spawn_rules_text,
        |_entity, component| {
            let content = CardTextContent {
                name: String::new(),
                mana_cost: String::new(),
                type_line: String::new(),
                rules_text: component.rules_text.clone(),
                power_toughness: None,
            };
            (content, SpawnedText)
        },
        debug_config.as_deref(),
    );

    // Special case for power/toughness
    for (entity, component, parent) in power_toughness_query.iter() {
        let parent_entity = parent.get();
        if let Ok((_, transform, sprite, _)) = card_query.get(parent_entity) {
            let card_size = sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = transform.translation.truncate();

            // Create the power/toughness text
            let pt_entity = spawn_power_toughness_text(
                &mut commands,
                &component.power_toughness,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(parent_entity).add_child(pt_entity);

            // Mark as spawned
            commands.entity(entity).insert(SpawnedText);
        }
    }
}

/// Process specialized text components with a common pattern
fn process_specialized_text_components<T: Component, G>(
    commands: &mut Commands,
    query: &Query<(Entity, &T, &Parent), (Without<SpawnedText>, With<T>)>,
    card_query: &Query<(Entity, &Transform, &Sprite, &Card), Without<SpawnedText>>,
    asset_server: &AssetServer,
    spawn_text_fn: fn(&mut Commands, &CardTextContent, Vec2, Vec2, &AssetServer) -> Entity,
    create_content_fn: G,
    _debug_config: Option<&DebugConfig>,
) where
    G: Fn(Entity, &T) -> (CardTextContent, SpawnedText),
{
    for (entity, component, parent) in query.iter() {
        let parent_entity = parent.get();
        if let Ok((_, transform, sprite, _)) = card_query.get(parent_entity) {
            let card_size = sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = transform.translation.truncate();

            // Create temporary CardTextContent
            let (content, spawned_marker) = create_content_fn(entity, component);

            // Spawn the text
            let text_entity = spawn_text_fn(commands, &content, card_pos, card_size, asset_server);
            commands.entity(parent_entity).add_child(text_entity);

            // Mark as spawned
            commands.entity(entity).insert(spawned_marker);
        }
    }
}

use bevy::prelude::*;

use crate::card::{Card, CardDetails};
use crate::text::{
    components::{CardTextContent, DebugConfig, SpawnedText},
    systems::{
        debug_visualization::spawn_debug_visualization, mana_cost_text::create_mana_cost_text,
        name_text::create_name_text, power_toughness_text::spawn_power_toughness_text,
        rules_text::spawn_rules_text, type_line_text::spawn_type_line_text,
    },
};

/// System to spawn text for cards
pub fn spawn_card_text(
    mut commands: Commands,
    text_content_query: Query<
        (Entity, &CardTextContent, &Parent),
        (Without<SpawnedText>, With<CardTextContent>),
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

    // Directly spawn text for all cards that don't have text yet
    if card_query.iter().count() > 0 {
        info!("Spawning text for {} cards", card_query.iter().count());

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
            let name_entity = create_name_text(
                &mut commands,
                &text_content,
                card_pos,
                card_size,
                &asset_server,
            );
            commands.entity(card_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity = create_mana_cost_text(
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

    // Process text content entities that don't have SpawnedText
    for (text_entity, content, parent) in text_content_query.iter() {
        let parent_entity = parent.get();
        if let Ok((_, card_transform, card_sprite, _)) = card_query.get(parent_entity) {
            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Spawn name text
            let name_entity =
                create_name_text(&mut commands, content, card_pos, card_size, &asset_server);
            commands.entity(parent_entity).add_child(name_entity);

            // Spawn mana cost text
            let mana_cost_entity =
                create_mana_cost_text(&mut commands, content, card_pos, card_size, &asset_server);
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
}

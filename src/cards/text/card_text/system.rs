use bevy::prelude::*;

use crate::cards::{Card, CardCost, CardDetails, CardDetailsComponent, CardName, CardTypeInfo};
use crate::text::components::{
    CardManaCostText, CardNameText, CardPowerToughness, CardRulesText, CardTypeLine, DebugConfig,
    SpawnedText,
};

use super::processors::{
    process_mana_cost_text_components, process_name_text_components, process_rules_text_components,
    process_type_line_text_components,
};
use crate::cards::text::{
    mana_cost_text::spawn_mana_cost_text, name_text::create_name_text,
    power_toughness_text::spawn_power_toughness_text, rules_text::spawn_rules_text,
    type_line_text::spawn_type_line_text,
};

/// System to spawn text for cards
#[allow(dead_code)]
pub fn spawn_card_text(
    mut commands: Commands,
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
    card_query: Query<
        (
            Entity,
            &Transform,
            &Sprite,
            &Card,
            &CardName,
            &CardCost,
            &CardTypeInfo,
            &CardRulesText,
            &CardDetailsComponent,
        ),
        Without<SpawnedText>,
    >,
    asset_server: Res<AssetServer>,
    debug_config: Option<Res<DebugConfig>>,
) {
    // Only log if we have cards to process
    if card_query.iter().count() > 0 {
        info!("Running spawn_card_text system");
        info!("Found {} cards without text", card_query.iter().count());
    }

    // Spawn text for cards that don't have specialized text components yet
    if card_query.iter().count() > 0 {
        info!("Spawning text for {} cards", card_query.iter().count());

        for (
            card_entity,
            card_transform,
            card_sprite,
            _card,
            card_name,
            card_cost,
            card_type_info,
            card_rules,
            card_details,
        ) in card_query.iter()
        {
            info!("Spawning text for card: {}", card_name.name);

            let card_size = card_sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = card_transform.translation.truncate();

            // Create text content components
            let name_component = CardNameText {
                name: card_name.name.clone(),
            };

            let mana_cost_component = CardManaCostText {
                mana_cost: card_cost.cost.to_string(),
            };

            let type_line_component = CardTypeLine {
                type_line: Card::type_line_from_components(&card_type_info.types),
            };

            let rules_text_component = CardRulesText {
                rules_text: card_rules.rules_text.clone(),
            };

            // Spawn name text
            let name_entity = create_name_text(
                &mut commands,
                &name_component,
                card_pos,
                card_size,
                &asset_server,
            );

            // Spawn mana cost text
            let mana_cost_entity = spawn_mana_cost_text(
                &mut commands,
                &mana_cost_component,
                card_pos,
                card_size,
                &asset_server,
            );

            // Spawn type line text
            let type_line_entity = spawn_type_line_text(
                &mut commands,
                &type_line_component,
                card_pos,
                card_size,
                &asset_server,
            );

            // Spawn rules text
            let rules_text_entity = spawn_rules_text(
                &mut commands,
                &rules_text_component,
                card_pos,
                card_size,
                &asset_server,
            );

            // Add all text entities as children of the card
            commands
                .entity(card_entity)
                .add_child(name_entity)
                .add_child(mana_cost_entity)
                .add_child(type_line_entity)
                .add_child(rules_text_entity);

            // Spawn power/toughness text if applicable
            if let CardDetails::Creature(creature) = &card_details.details {
                let pt_component = CardPowerToughness {
                    power_toughness: format!("{}/{}", creature.power, creature.toughness),
                };

                let pt_entity = spawn_power_toughness_text(
                    &mut commands,
                    &pt_component,
                    card_pos,
                    card_size,
                    &asset_server,
                );

                commands.entity(card_entity).add_child(pt_entity);
            }

            // Mark this card as having its text spawned
            commands.entity(card_entity).insert(SpawnedText);
        }
    }

    // Process individual specialized text components
    process_name_text_components(
        &mut commands,
        &name_query,
        &card_query,
        &asset_server,
        debug_config.as_deref(),
    );

    process_mana_cost_text_components(
        &mut commands,
        &mana_cost_query,
        &card_query,
        &asset_server,
        debug_config.as_deref(),
    );

    process_type_line_text_components(
        &mut commands,
        &type_line_query,
        &card_query,
        &asset_server,
        debug_config.as_deref(),
    );

    process_rules_text_components(
        &mut commands,
        &rules_text_query,
        &card_query,
        &asset_server,
        debug_config.as_deref(),
    );

    // Special case for power/toughness
    for (entity, component, parent) in power_toughness_query.iter() {
        let parent_entity = parent.get();
        if let Ok((_, transform, sprite, _card, _name, _cost, _types, _rules, _details)) =
            card_query.get(parent_entity)
        {
            let card_size = sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = transform.translation.truncate();

            // Create the power/toughness text
            let pt_component = CardPowerToughness {
                power_toughness: component.power_toughness.clone(),
            };

            let pt_entity = spawn_power_toughness_text(
                &mut commands,
                &pt_component,
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

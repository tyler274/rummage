use bevy::prelude::*;

use crate::cards::text::{
    name_text::create_name_text, power_toughness_text::spawn_power_toughness_text,
    rules_text::spawn_rules_text, type_line_text::spawn_type_line_text,
};
use crate::cards::{Card, CardCost, CardDetails, CardDetailsComponent, CardName, CardTypeInfo};
use crate::text::components::{
    CardManaCostText, CardNameText, CardPowerToughness, CardRulesText, CardTypeLine, DebugConfig,
    SpawnedText,
};

/// Spawn all text components for a single card
/// This is a convenience function that handles creating and spawning all text components
/// for a given card entity. It's useful for cards that need to have text dynamically
/// generated or updated.
pub fn spawn_card_text_components(
    commands: &mut Commands,
    card_entity: Entity,
    card_components: (
        &Card,
        &CardName,
        &CardCost,
        &CardTypeInfo,
        &CardDetailsComponent,
        &CardRulesText,
    ),
    transform: &Transform,
    sprite: &Sprite,
    asset_server: &AssetServer,
    _debug_config: Option<&DebugConfig>,
) {
    let (_card, card_name, card_cost, card_type_info, card_details, card_rules) = card_components;

    info!("Spawning text for card: {}", card_name.name);

    let card_size = sprite.custom_size.unwrap_or(Vec2::ONE);
    let card_pos = transform.translation.truncate();

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
    let name_entity =
        create_name_text(commands, &name_component, card_pos, card_size, asset_server);

    // Spawn mana cost text
    let mana_cost_entity = crate::cards::text::mana_cost_text::spawn_mana_cost_text_from_text(
        commands,
        &mana_cost_component,
        card_pos,
        card_size,
        asset_server,
    );

    // Spawn type line text
    let type_line_entity = spawn_type_line_text(
        commands,
        &type_line_component,
        card_pos,
        card_size,
        asset_server,
    );

    // Spawn rules text
    let rules_text_entity = spawn_rules_text(
        commands,
        &rules_text_component,
        card_pos,
        card_size,
        asset_server,
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

        let pt_entity =
            spawn_power_toughness_text(commands, &pt_component, card_pos, card_size, asset_server);

        commands.entity(card_entity).add_child(pt_entity);
    }

    // Mark this card as having its text spawned
    commands.entity(card_entity).insert(SpawnedText);
}

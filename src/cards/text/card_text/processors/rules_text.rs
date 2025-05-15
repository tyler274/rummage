use bevy::prelude::ChildOf;
use bevy::prelude::*;

use crate::cards::text::rules_text::spawn_rules_text;
use crate::cards::{Card, CardCost, CardDetailsComponent, CardName, CardTypeInfo};
use crate::text::components::{CardRulesText, DebugConfig, SpawnedText};

/// Process specialized rules text components
#[allow(dead_code)]
pub fn process_rules_text_components(
    commands: &mut Commands,
    query: &Query<(Entity, &CardRulesText, &ChildOf), (Without<SpawnedText>, With<CardRulesText>)>,
    card_query: &Query<
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
    asset_server: &AssetServer,
    _debug_config: Option<&DebugConfig>,
) {
    for (entity, component, child_of_component) in query.iter() {
        let parent_entity = child_of_component.parent();
        if let Ok((_, transform, sprite, _card, _name, _cost, _types, _rules, _details)) =
            card_query.get(parent_entity)
        {
            let card_size = sprite.custom_size.unwrap_or(Vec2::ONE);
            let card_pos = transform.translation.truncate();

            // Spawn the text
            let text_entity =
                spawn_rules_text(commands, component, card_pos, card_size, asset_server);
            commands.entity(parent_entity).add_child(text_entity);

            // Mark as spawned
            commands.entity(entity).insert(SpawnedText);
        }
    }
}

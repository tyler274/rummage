use bevy::prelude::*;

use crate::cards::Card;
use crate::mana::render::components::CardManaCostText as ManaCardManaCostText;
use crate::text::components::CardManaCostText as TextCardManaCostText;

/// System implementation that finds cards and creates mana cost text for them
#[allow(dead_code)]
pub fn mana_cost_text_system(
    commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    // Re-export the mana cost text system from the mana module
    crate::mana::render::systems::mana_cost_text_system(commands, query, asset_server);
}

/// Convert from text module CardManaCostText to mana module CardManaCostText
pub fn convert_mana_cost_text(text_component: &TextCardManaCostText) -> ManaCardManaCostText {
    ManaCardManaCostText {
        mana_cost: text_component.mana_cost.clone(),
    }
}

/// Spawn mana cost text for a card, accepting the text module's CardManaCostText
pub fn spawn_mana_cost_text_from_text(
    commands: &mut Commands,
    text_component: &TextCardManaCostText,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let mana_component = convert_mana_cost_text(text_component);
    crate::mana::render::systems::spawn_mana_cost_text(
        commands,
        &mana_component,
        card_pos,
        card_size,
        asset_server,
    )
}

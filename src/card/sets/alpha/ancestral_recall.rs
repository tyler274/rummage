use bevy::prelude::*;

use crate::card::{
    Card, CardCost, CardDetails, CardDetailsComponent, CardKeywords, CardName, CardRulesText,
    CardTypeInfo, CardTypes, Rarity,
};
use crate::mana::Mana;

use super::set_info;

/// Spawn Ancestral Recall card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    // Create the card using the builder
    let card = Card::builder("Ancestral Recall")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Target player draws three cards.")
        .build_or_panic();

    // Get individual components for backward compatibility
    let (card, card_name, card_cost, card_type_info, card_details, card_rules_text, card_keywords) =
        card.get_components();

    let entity = commands
        .spawn(card)
        .insert(card_name)
        .insert(card_cost)
        .insert(card_type_info)
        .insert(card_details)
        .insert(card_rules_text)
        .insert(card_keywords)
        .insert(set_info())
        .insert(Rarity::Rare)
        .insert(Name::new("Ancestral Recall"))
        .id();

    Some(entity)
}

/// Get the card components
#[allow(dead_code)]
pub fn get_card() -> (
    Card,
    CardName,
    CardCost,
    CardTypeInfo,
    CardDetailsComponent,
    CardRulesText,
    CardKeywords,
) {
    let card = Card::builder("Ancestral Recall")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Target player draws three cards.")
        .build_or_panic();

    // Return the card and its individual components
    card.get_components()
}

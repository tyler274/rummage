use crate::cards::{
    card::Card,
    components::{
        CardCost, CardDetailsComponent, CardKeywords, CardName, CardRulesText, Draggable,
        NoUntapCondition, NoUntapEffect, PermanentState,
    },
    details::{
        ArtifactCard, CardDetails, CreatureCard, EnchantmentCard, LandCard, SpellCard, SpellType,
    },
    keywords::{KeywordAbilities, KeywordAbility},
    rarity::Rarity,
    set::CardSet,
    systems::{debug_render_text_positions, handle_card_dragging},
    types::{CardTypes, CreatureType, ReflectableCardTypes, ReflectableCreatureType},
};
use crate::mana::{Mana, ReflectableColor};
use bevy::prelude::*;

/// Plugin for registering card-related systems and components
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Card>()
            .register_type::<CardName>()
            .register_type::<CardCost>()
            // CardTypeInfo contains bitflags which now support reflection
            .register_type::<CardDetailsComponent>()
            .register_type::<CardRulesText>()
            .register_type::<CardKeywords>()
            .register_type::<PermanentState>()
            .register_type::<CardSet>()
            .register_type::<Rarity>()
            .register_type::<CardDetails>()
            .register_type::<CreatureCard>()
            // Register the reflectable wrappers for bitflags
            .register_type::<ReflectableCardTypes>()
            .register_type::<ReflectableCreatureType>()
            .register_type::<KeywordAbility>()
            .register_type::<KeywordAbilities>()
            .register_type::<SpellType>()
            .register_type::<SpellCard>()
            .register_type::<EnchantmentCard>()
            .register_type::<ArtifactCard>()
            .register_type::<LandCard>()
            .register_type::<NoUntapEffect>()
            .register_type::<NoUntapCondition>()
            .register_type::<Draggable>()
            .register_type::<Mana>()
            // Register the reflectable wrapper for Color
            .register_type::<ReflectableColor>()
            .register_type::<std::collections::HashSet<KeywordAbility>>()
            .register_type::<std::collections::HashMap<KeywordAbility, String>>()
            .add_systems(Update, handle_card_dragging)
            .add_systems(Update, debug_render_text_positions);
    }
}

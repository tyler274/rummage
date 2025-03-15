use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cards::types::CreatureType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum CardDetails {
    Creature(CreatureCard),
    Planeswalker { loyalty: i32 },
    Instant(SpellCard),
    Sorcery(SpellCard),
    Enchantment(EnchantmentCard),
    Artifact(ArtifactCard),
    Land(LandCard),
    Other,
}

impl Default for CardDetails {
    fn default() -> Self {
        CardDetails::Other
    }
}

impl CardDetails {
    /// Create a new Creature card details
    #[allow(dead_code)]
    pub fn new_creature(power: i32, toughness: i32) -> Self {
        CardDetails::Creature(CreatureCard {
            power,
            toughness,
            creature_type: crate::cards::types::CreatureType::NONE,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct SpellCard {
    pub spell_type: SpellType,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum SpellType {
    Instant,
    Sorcery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct EnchantmentCard {
    pub enchantment_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ArtifactCard {
    pub artifact_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct LandCard {
    pub land_type: Option<String>,
    pub produces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct CreatureCard {
    pub power: i32,
    pub toughness: i32,
    #[reflect(ignore)]
    pub creature_type: CreatureType,
}

/// A struct representing a creature on the field with tracking for its current power/toughness
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct CreatureOnField {
    pub card: crate::cards::Card,
    pub power_modifier: i64,
    pub toughness_modifier: i64,
    pub battle_damage: u64,
    pub token: bool,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct CardVisualBundle {
    pub sprite: Sprite,
    pub card: crate::cards::card::Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

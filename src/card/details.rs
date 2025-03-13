use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::types::CreatureType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellCard {
    pub spell_type: SpellType,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellType {
    Instant,
    Sorcery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnchantmentCard {
    pub enchantment_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactCard {
    pub artifact_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandCard {
    pub land_type: Option<String>,
    pub produces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatureCard {
    pub power: i32,
    pub toughness: i32,
    pub creature_type: CreatureType,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatureOnField {
    pub power_modifier: i64,
    pub toughness_modifier: i64,
    pub battle_damage: u64,
    pub token: bool,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct CardBundle {
    pub sprite: Sprite,
    pub card: super::Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

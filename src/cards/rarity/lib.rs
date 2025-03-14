use bevy::prelude::*;

/// Card rarity in Magic: The Gathering
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
    Special,
    Bonus,
    Promo,
}

impl From<&str> for Rarity {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "common" => Rarity::Common,
            "uncommon" => Rarity::Uncommon,
            "rare" => Rarity::Rare,
            "mythic" | "mythic rare" => Rarity::MythicRare,
            "special" => Rarity::Special,
            "bonus" => Rarity::Bonus,
            "promo" => Rarity::Promo,
            _ => Rarity::Common,
        }
    }
}

use bevy::prelude::*;
use bitflags::bitflags;

bitflags! {
    #[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
    struct CardType: u8 {
        const CREATURE = 0b00001;
        const INSTANT = 0b00010;
        const SORCERY = 0b00100;
        const ENCHANTMENT = 0b01000;
        const ARTIFACT = 0b10000;
        const LAND = 0b100000;
        const LEGENDARY = 0b1000000;
    }
}

bitflags! {
    // There are about 300 creatures types.
    #[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
    struct CreatureType: u8 {
        const HUMAN = 0b00001;
        const WIZARD = 0b00010;
        const DRAGON = 0b00100;
        const ANGEL = 0b01000;
        const DEMON = 0b10000;
        // TODO: Add more creature types
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
#[require(CardType)]
pub(crate) struct Card {
    name: String,
    // The cost written on the card, e.g. "1WU"
    cost: u64,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
#[require(Card, CreatureType)]
struct Creature {
    power: u64,
    toughness: u64,
    // TODO: Placeholder for actual rules simulation
    abilities: Vec<String>,
}

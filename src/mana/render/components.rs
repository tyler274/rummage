use bevy::prelude::*;

/// Component for mana cost text on a card
#[derive(Component, Clone, Debug)]
pub struct CardManaCostText {
    /// The mana cost as a string, e.g. "{1}{W}{U}"
    pub mana_cost: String,
}

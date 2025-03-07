use bevy::prelude::*;
use crate::{
    card::Card,
    mana::ManaPool,
};

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct Hand {
    cards: Vec<Card>,
}

#[derive(Component, Default, Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct Player {
    name: String,
    life: i32,
    mana_pool: ManaPool,
    hand: Hand,
}

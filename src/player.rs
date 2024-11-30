use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{card::Card, mana::ManaPool};

pub(crate) struct Hand {
    cards: Vec<Card>,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(ManaPool)]
pub(crate) struct Player {
    name: String,
    health: u64,
}

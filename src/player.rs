use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::mana::ManaPool;

pub(crate) struct Hand {
    cards: HashSet<Entity>,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(ManaPool)]
pub(crate) struct Player {
    name: String,
    health: u64,
}

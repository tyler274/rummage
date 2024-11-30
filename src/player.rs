use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::mana::ManaPool;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Player {
    name: String,
    health: u64,
    mana: ManaPool,
}

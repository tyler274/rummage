use bevy::prelude::*;
use crate::card::Card;
use crate::mana::ManaPool;

#[derive(Component, Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub cards: Vec<Card>,
}

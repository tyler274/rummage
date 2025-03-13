use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tracks various counters that can be placed on permanents
#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct PermanentCounters {
    /// +1/+1 counters
    pub plus_one_plus_one: u32,
    /// -1/-1 counters
    pub minus_one_minus_one: u32,
    /// Loyalty counters (for planeswalkers)
    pub loyalty: u32,
    /// Charge counters (for artifacts like Chalice of the Void)
    pub charge: u32,
    /// Poison counters (for players)
    pub poison: u32,
    /// Age counters (for cumulative upkeep and Vanishing)
    pub age: u32,
    /// Arrow counters (for equipment like Obsidian Battle-Axe)
    pub arrow: u32,
    /// Awakening counters (for sagas)
    pub awakening: u32,
    /// Blame counters
    pub blame: u32,
    /// Bloodline counters
    pub bloodline: u32,
    /// Bounty counters
    pub bounty: u32,
    /// Brick counters (for Wall of Roots)
    pub brick: u32,
    /// Coin counters
    pub coin: u32,
    /// Corruption counters
    pub corruption: u32,
    /// Credit counters
    pub credit: u32,
    /// Cube counters
    pub cube: u32,
    /// Currency counters
    pub currency: u32,
    /// Death counters
    pub death: u32,
    /// Depletion counters
    pub depletion: u32,
    /// Despair counters
    pub despair: u32,
    /// Devotion counters
    pub devotion: u32,
    /// Divinity counters
    pub divinity: u32,
    /// Doom counters
    pub doom: u32,
    /// Dream counters
    pub dream: u32,
    /// Echo counters
    pub echo: u32,
    /// Egg counters
    pub egg: u32,
    /// Elixir counters
    pub elixir: u32,
    /// Energy counters (for players and permanents)
    pub energy: u32,
    /// Experience counters (for players)
    pub experience: u32,
    /// Eyeball counters
    pub eyeball: u32,
    /// Fade counters
    pub fade: u32,
    /// Fate counters
    pub fate: u32,
    /// Feather counters
    pub feather: u32,
    /// Flood counters
    pub flood: u32,
    /// Fungus counters
    pub fungus: u32,
    /// Fury counters
    pub fury: u32,
    /// Fuse counters
    pub fuse: u32,
    /// Gem counters
    pub gem: u32,
    /// Gold counters
    pub gold: u32,
    /// Growth counters
    pub growth: u32,
    /// Hatchling counters
    pub hatchling: u32,
    /// Healing counters
    pub healing: u32,
    /// Hit counters
    pub hit: u32,
    /// Hoofprint counters
    pub hoofprint: u32,
    /// Hour counters
    pub hour: u32,
    /// Hourglass counters
    pub hourglass: u32,
    /// Hunger counters
    pub hunger: u32,
    /// Ice counters
    pub ice: u32,
    /// Infection counters
    pub infection: u32,
    /// Intervention counters
    pub intervention: u32,
    /// Javelin counters
    pub javelin: u32,
    /// Journey counters
    pub journey: u32,
    /// Ki counters
    pub ki: u32,
    /// Knowledge counters
    pub knowledge: u32,
    /// Level counters
    pub level: u32,
    /// Luck counters
    pub luck: u32,
    /// Magnet counters
    pub magnet: u32,
    /// Manifestation counters
    pub manifestation: u32,
    /// Mannequin counters
    pub mannequin: u32,
    /// Matrix counters
    pub matrix: u32,
    /// Mine counters
    pub mine: u32,
    /// Mining counters
    pub mining: u32,
    /// Mire counters
    pub mire: u32,
    /// Music counters
    pub music: u32,
    /// Muster counters
    pub muster: u32,
    /// Net counters
    pub net: u32,
    /// Night counters
    pub night: u32,
    /// Omen counters
    pub omen: u32,
    /// Ore counters
    pub ore: u32,
    /// Page counters
    pub page: u32,
    /// Pain counters
    pub pain: u32,
    /// Paralyzation counters
    pub paralyzation: u32,
    /// Petal counters
    pub petal: u32,
    /// Petrification counters
    pub petrification: u32,
    /// Phylactery counters
    pub phylactery: u32,
    /// Pin counters
    pub pin: u32,
    /// Plague counters
    pub plague: u32,
    /// Pressure counters
    pub pressure: u32,
    /// Prey counters
    pub prey: u32,
    /// Protection counters
    pub protection: u32,
    /// Quest counters
    pub quest: u32,
    /// Rust counters
    pub rust: u32,
    /// Scream counters
    pub scream: u32,
    /// Shell counters
    pub shell: u32,
    /// Shield counters
    pub shield: u32,
    /// Shred counters
    pub shred: u32,
    /// Sleep counters
    pub sleep: u32,
    /// Slime counters
    pub slime: u32,
    /// Soot counters
    pub soot: u32,
    /// Spark counters
    pub spark: u32,
    /// Spore counters
    pub spore: u32,
    /// Storage counters
    pub storage: u32,
    /// Strife counters
    pub strife: u32,
    /// Study counters
    pub study: u32,
    /// Theft counters
    pub theft: u32,
    /// Tide counters
    pub tide: u32,
    /// Time counters
    pub time: u32,
    /// Tower counters
    pub tower: u32,
    /// Training counters
    pub training: u32,
    /// Trap counters
    pub trap: u32,
    /// Treasure counters
    pub treasure: u32,
    /// Velocity counters
    pub velocity: u32,
    /// Verse counters
    pub verse: u32,
    /// Vitality counters
    pub vitality: u32,
    /// Vortex counters
    pub vortex: u32,
    /// Wage counters
    pub wage: u32,
    /// Wind counters
    pub wind: u32,
    /// Wish counters
    pub wish: u32,
    /// Custom counters (for custom counter types not listed above)
    pub custom: HashMap<String, u32>,
}

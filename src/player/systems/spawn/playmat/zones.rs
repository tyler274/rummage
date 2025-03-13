//! Coordinates spawning of all player playmat zones

use crate::camera::components::AppLayer;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

use super::{battlefield, command, exile, graveyard, hand, library};

/// Zone component for all playmat zones
#[derive(Component, Debug)]
pub struct PlaymatZone {
    /// The player this zone belongs to
    pub player_id: Entity,
    /// The type of zone
    pub zone_type: ZoneType,
}

/// Types of zones on a player's playmat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    /// Main play area for permanents
    Battlefield,
    /// Player's hand
    Hand,
    /// Player's deck/library
    Library,
    /// Player's discard pile
    Graveyard,
    /// Cards removed from game
    Exile,
    /// Special zone for commanders and emblems
    Command,
}

/// Spawns all zones for a player's playmat
pub fn spawn_player_zones(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    playmat_entity: Entity,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
) {
    info!("Spawning zones for player {}'s playmat", player.name);

    // Spawn each zone as a child of the playmat
    battlefield::spawn_battlefield_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );
    hand::spawn_hand_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );
    library::spawn_library_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );
    graveyard::spawn_graveyard_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );
    exile::spawn_exile_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );
    command::spawn_command_zone(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );

    info!(
        "Finished spawning all zones for player {}'s playmat",
        player.name
    );
}

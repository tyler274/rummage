//! Player playmat system for spawning and managing the player's board layout
//! as defined in the playmat documentation.

mod battlefield;
mod command;
mod exile;
mod graveyard;
mod hand;
mod library;
mod zones;

use crate::camera::components::AppLayer;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

// Re-export the main function for convenience
pub use zones::spawn_player_zones;

/// Playmat component to identify and query the player's playmat
#[derive(Component, Debug)]
pub struct PlayerPlaymat {
    /// The player this playmat belongs to
    pub player_id: Entity,
    /// The player's index (0-3) for positioning
    pub player_index: usize,
}

/// Spawns a complete playmat for a player with all zones
///
/// This function coordinates the spawning of all playmat zones for a player,
/// including battlefield, hand, library, graveyard, exile, and command zone.
pub fn spawn_player_playmat(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
    player_position: Vec3,
) -> Entity {
    info!(
        "Spawning playmat for player {} at position {:?}",
        player.name, player_position
    );

    // Create the main playmat entity
    let playmat_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(player_position),
                ..default()
            },
            PlayerPlaymat {
                player_id: player_entity,
                player_index: player.player_index,
            },
            AppLayer::game_layers(),
            Name::new(format!("Playmat-{}", player.name)),
        ))
        .id();

    // Spawn all the zones as children of the playmat
    zones::spawn_player_zones(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );

    info!(
        "Playmat spawned for player {} with entity {:?}",
        player.name, playmat_entity
    );

    playmat_entity
}

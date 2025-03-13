//! Command zone implementation for the player playmat

use crate::camera::components::AppLayer;
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

use super::PlaymatZone;

/// Spawn the command zone for a player
pub fn spawn_command_zone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    playmat_entity: Entity,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
) -> Entity {
    info!("Spawning command zone for player {}", player.name);

    // Determine position relative to playmat based on player index
    let position = match player.player_index {
        0 => Vec3::new(200.0, 0.0, 0.0),  // Bottom player
        1 => Vec3::new(0.0, 200.0, 0.0),  // Right player
        2 => Vec3::new(-200.0, 0.0, 0.0), // Top player
        3 => Vec3::new(0.0, -200.0, 0.0), // Left player
        _ => Vec3::ZERO,
    };

    // Create the command zone entity
    let command_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            PlaymatZone {
                player_id: player_entity,
                zone_type: Zone::CommandZone,
            },
            AppLayer::game_layers(),
            Name::new(format!("Command-{}", player.name)),
        ))
        .set_parent(playmat_entity)
        .id();

    info!(
        "Command zone spawned for player {} with entity {:?}",
        player.name, command_entity
    );

    command_entity
}

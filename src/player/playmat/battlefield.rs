//! Battlefield zone implementation for the player playmat

use crate::camera::components::AppLayer;
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

use super::PlaymatZone;

/// Spawn the battlefield zone for a player
pub fn spawn_battlefield_zone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    playmat_entity: Entity,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
) -> Entity {
    info!("Spawning battlefield zone for player {}", player.name);

    // Determine position relative to playmat based on player index
    let position = match player.player_index {
        0 => Vec3::new(0.0, 0.0, 0.0), // Bottom player
        1 => Vec3::new(0.0, 0.0, 0.0), // Right player
        2 => Vec3::new(0.0, 0.0, 0.0), // Top player
        3 => Vec3::new(0.0, 0.0, 0.0), // Left player
        _ => Vec3::ZERO,
    };

    // Create the battlefield zone entity
    let battlefield_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            PlaymatZone {
                player_id: player_entity,
                zone_type: Zone::Battlefield,
            },
            AppLayer::game_layers(),
            Name::new(format!("Battlefield-{}", player.name)),
        ))
        .set_parent(playmat_entity)
        .id();

    info!(
        "Battlefield zone spawned for player {} with entity {:?}",
        player.name, battlefield_entity
    );

    battlefield_entity
}

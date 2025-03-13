use crate::player::Player;
use bevy::prelude::*;

use super::events::{EntersBattlefieldEvent, ZoneChangeEvent};
use super::resources::ZoneManager;

/// System for handling card movement between zones
pub fn handle_zone_changes(
    _commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut events: EventReader<ZoneChangeEvent>,
    _turn_manager: Option<Res<crate::game_engine::turns::TurnManager>>,
) {
    for event in events.read() {
        // Process the zone change
        zone_manager.move_card(event.card, event.owner, event.source, event.destination);
    }
}

/// System for handling permanents entering the battlefield
pub fn handle_enters_battlefield(
    _commands: Commands,
    mut enter_events: EventReader<EntersBattlefieldEvent>,
    _turn_manager: Option<Res<crate::game_engine::turns::TurnManager>>,
) {
    // Handle "enters the battlefield" effects
    for event in enter_events.read() {
        // A permanent has entered the battlefield
        info!(
            "Permanent {:?} entered the battlefield (owner: {:?}, tapped: {})",
            event.permanent, event.owner, event.enters_tapped
        );

        // Here we would handle any ETB (enters-the-battlefield) triggered abilities
        // For now we're just logging
    }
}

/// System for initializing the ZoneManager resource
///
/// This function is intended to be used during game setup to initialize
/// zone management for each player. Currently not actively used but will
/// be needed for proper game initialization in the future.
///
/// TODO: Implement zone management initialization as part of game setup
#[allow(dead_code)]
pub fn setup_zone_manager(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    // Create a new zone manager
    let mut zone_manager = ZoneManager::default();

    // Initialize zones for each player
    for player in player_query.iter() {
        zone_manager.init_player_zones(player);
    }

    // Add the zone manager as a resource
    commands.insert_resource(zone_manager);
}

/// Register zone systems with the app
pub fn register_zone_systems(app: &mut App) {
    app.add_systems(
        Update,
        (handle_zone_changes, handle_enters_battlefield)
            .run_if(crate::game_engine::game_state_condition),
    );
}

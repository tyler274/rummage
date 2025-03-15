use crate::player::Player;
use bevy::prelude::*;

use super::events::{EntersBattlefieldEvent, ZoneChangeEvent};
use super::resources::ZoneManager;
use super::types::{Zone, ZoneMarker};
use crate::game_engine::permanent::{
    Permanent, PermanentController, PermanentOwner, PermanentState,
};

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

/// System to process zone change events
pub fn process_zone_changes(
    mut commands: Commands,
    mut zone_events: EventReader<ZoneChangeEvent>,
    mut enters_battlefield_events: EventWriter<EntersBattlefieldEvent>,
    turn_manager: Option<Res<crate::game_engine::turns::TurnManager>>,
) {
    let current_turn = turn_manager.map(|t| t.turn_number).unwrap_or(0);

    for event in zone_events.read() {
        // Update the card's zone marker
        commands.entity(event.card).insert(ZoneMarker {
            zone_type: event.destination,
            owner: Some(event.owner),
        });

        // Handle entering the battlefield
        if event.destination == Zone::Battlefield {
            // Add permanent components when a card enters the battlefield
            commands
                .entity(event.card)
                .insert(Permanent)
                .insert(PermanentState::new(current_turn))
                .insert(PermanentOwner::new(event.owner))
                .insert(PermanentController::new(event.owner));

            // Send an enters battlefield event
            enters_battlefield_events.send(EntersBattlefieldEvent {
                permanent: event.card,
                owner: event.owner,
                enters_tapped: false, // Default to untapped, can be modified by effects
            });
        } else if event.source == Zone::Battlefield {
            // Remove permanent components when a card leaves the battlefield
            commands
                .entity(event.card)
                .remove::<Permanent>()
                .remove::<PermanentState>()
                .remove::<PermanentOwner>()
                .remove::<PermanentController>();
        }
    }
}

/// Register zone systems with the app
pub fn register_zone_systems(app: &mut App) {
    app.add_systems(
        Update,
        (handle_zone_changes, handle_enters_battlefield)
            .run_if(crate::game_engine::game_state_condition),
    );
}

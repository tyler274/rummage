use super::combat::CombatState;
use crate::game_engine::commander::{CombatDamageEvent, Commander};
use crate::game_engine::state::GameState;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

/// Setup a test combat environment with attackers and blockers
#[allow(dead_code)]
pub fn setup_test_combat(
    app: &mut App,
    attackers: Vec<(Entity, Entity)>, // (attacker, defender) pairs
    blockers: Vec<(Entity, Entity)>,  // (blocker, attacker) pairs
    commander_entities: Vec<Entity>,  // Which entities are commanders
) {
    let world = app.world_mut();
    let mut combat_state = world.resource_mut::<CombatState>();

    // Add attackers
    for (attacker, defender) in attackers {
        combat_state.attackers.insert(attacker, defender);
    }

    // Add blockers
    for (blocker, attacker) in blockers {
        combat_state
            .blockers
            .entry(attacker)
            .or_default()
            .push(blocker);
        combat_state
            .blocked_status
            .insert(attacker, super::combat::BlockedStatus::Blocked);
    }

    // Set up commander damage tracking
    for commander in commander_entities {
        combat_state
            .commander_damage_this_combat
            .insert(commander, HashMap::new());
    }

    combat_state.in_declare_attackers = true;
    combat_state.in_declare_blockers = true;
}

/// Apply combat damage from a list of events
#[allow(dead_code)]
pub fn apply_combat_damage(app: &mut App, damage_events: Vec<CombatDamageEvent>) {
    let world = app.world_mut();

    // Get pending combat damage
    let mut pending_events = world
        .resource_mut::<CombatState>()
        .pending_combat_damage
        .drain(..)
        .collect::<Vec<_>>();

    // Add the provided damage events
    pending_events.extend(damage_events);

    // Process each damage event
    for event in pending_events {
        // Get the entity first
        let entity_opt = world
            .query_filtered::<Entity, With<Player>>()
            .iter(world)
            .find(|&id| id == event.target);

        if let Some(player_entity) = entity_opt {
            // Then get the player component mutably
            if let Some(mut player) = world.get_mut::<Player>(player_entity) {
                player.life -= event.damage as i32;
            }

            // Update game state
            if let Some(mut game_state) = world.get_resource_mut::<GameState>() {
                game_state.state_based_actions_performed = true;
            }

            // Handle commander damage separately
            if event.source_is_commander {
                if let Some(mut commander) = world.get_mut::<Commander>(event.source) {
                    // Update the commander's damage tracking
                    if let Some(damage_entry) = commander
                        .damage_dealt
                        .iter_mut()
                        .find(|(p, _)| *p == player_entity)
                    {
                        // Update existing damage entry
                        damage_entry.1 += event.damage;
                    } else {
                        // Add a new damage entry
                        commander.damage_dealt.push((player_entity, event.damage));
                    }
                }
            }
        }
    }
}

/// Add an attacker with a specific target
#[allow(dead_code)]
pub fn add_attacker_with_target(app: &mut App, attacker: Entity, target: Entity) {
    let world = app.world_mut();
    let mut combat_state = world.resource_mut::<CombatState>();
    combat_state.attackers.insert(attacker, target);
    combat_state.in_declare_attackers = true;
}

/// Assign a blocker to an attacker
#[allow(dead_code)]
pub fn assign_blocker(app: &mut App, attacker: Entity, blocker: Entity) {
    let world = app.world_mut();
    let mut combat_state = world.resource_mut::<CombatState>();

    if !combat_state.attackers.contains_key(&attacker) {
        panic!("Cannot assign blocker to unregistered attacker");
    }

    combat_state
        .blocked_status
        .insert(attacker, super::combat::BlockedStatus::Blocked);
    combat_state
        .blockers
        .entry(attacker)
        .or_default()
        .push(blocker);
    combat_state.in_declare_blockers = true;
}

/// Deal damage to all players
#[allow(dead_code)]
pub fn deal_damage_to_players(app: &mut App, amount: i32) {
    let world = app.world_mut();

    // Deal damage to all players
    if amount > 0 {
        // Get all player entities
        let players: Vec<Entity> = world
            .query_filtered::<Entity, With<Player>>()
            .iter(world)
            .collect();

        // Then apply damage to each player one at a time
        for player_entity in players {
            if let Some(mut player) = world.get_mut::<Player>(player_entity) {
                player.life -= amount;
            }
        }
    }
}

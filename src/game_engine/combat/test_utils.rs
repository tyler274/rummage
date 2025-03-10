use super::combat::CombatState;
use crate::game_engine::commander::{CombatDamageEvent, Commander};
use crate::game_engine::state::GameState;
use crate::menu::GameMenuState;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup_test_combat(
    app: &mut App,
    attackers: Vec<(Entity, Entity)>, // (attacker, defender) pairs
    blockers: Vec<(Entity, Entity)>,  // (blocker, attacker) pairs
    commander_entities: Vec<Entity>,  // Which entities are commanders
) {
    let mut world = app.world_mut();
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
            .or_insert_with(Vec::new)
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

pub fn apply_combat_damage(app: &mut App, damage_events: Vec<CombatDamageEvent>) {
    let mut world = app.world_mut();
    let mut combat_state = world.resource_mut::<CombatState>();

    // Add all damage events to pending list
    for event in damage_events {
        combat_state.pending_combat_damage.push(event);
    }

    // Process each damage event
    let pending_events = combat_state
        .pending_combat_damage
        .drain(..)
        .collect::<Vec<_>>();
    for event in pending_events {
        // Get the player entity and apply damage
        if let Some((player_entity, mut player)) = world
            .query::<(Entity, &mut Player)>()
            .iter_mut(&mut world)
            .find(|(id, _)| *id == event.target)
        {
            player.life -= event.damage as i32;
            world
                .resource_mut::<GameState>()
                .state_based_actions_performed = true;

            // If it was a commander, track commander damage
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

pub fn add_attacker_with_target(app: &mut App, attacker: Entity, target: Entity) {
    let mut world = app.world_mut();
    let mut combat_state = world.resource_mut::<CombatState>();
    combat_state.attackers.insert(attacker, target);
    combat_state.in_declare_attackers = true;
}

pub fn assign_blocker(app: &mut App, attacker: Entity, blocker: Entity) {
    let mut world = app.world_mut();
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
        .or_insert_with(Vec::new)
        .push(blocker);
    combat_state.in_declare_blockers = true;
}

pub fn deal_damage_to_players(app: &mut App, amount: i32) {
    let mut world = app.world_mut();

    // Deal damage to all players
    if amount > 0 {
        // Get all player entities
        let players: Vec<Entity> = world.query::<Entity>().iter(&world).collect();

        // Then apply damage to each player
        for player_entity in players {
            if let Some(mut player) = world.get_mut::<Player>(player_entity) {
                player.life -= amount;
            }
        }

        // Finally update game state
        if let Some(mut game_state) = world.get_resource_mut::<GameState>() {
            game_state.state_based_actions_performed = true;
        }
    }
}

use super::components::Commander;
use super::components::{CommanderZoneLocation, EliminationReason};
use super::events::{CombatDamageEvent, CommanderZoneChoiceEvent, PlayerEliminatedEvent};
use super::resources::{CommandZone, CommandZoneManager};
use super::rules::CommanderRules;
use crate::card::Card;
use crate::game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager};
use crate::mana::Mana;
use crate::menu::GameMenuState;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Initialize Commander-specific resources and components
pub fn setup_commander(mut commands: Commands) {
    commands.insert_resource(CommandZone::default());
    commands.insert_resource(CommandZoneManager::default());
}

/// Calculate the mana cost of a Commander including the Commander tax
pub fn calculate_commander_cost(
    commander: Entity,
    base_cost: Mana,
    cmd_zone_manager: &CommandZoneManager,
) -> Mana {
    let mut final_cost = base_cost.clone();

    // Get the commander's cast count and add tax
    let cast_count = cmd_zone_manager.get_cast_count(commander);
    final_cost.colorless += CommanderRules::calculate_tax(cast_count);

    final_cost
}

/// Check if any player has lost due to commander damage
pub fn check_commander_damage_loss(
    mut commands: Commands,
    commander_query: Query<&Commander>,
    player_query: Query<(Entity, &Player)>,
) {
    for (player_entity, _player) in player_query.iter() {
        // Check each commander for damage dealt to this player
        for commander in commander_query.iter() {
            if let Some(damage) = commander
                .damage_dealt
                .iter()
                .find(|(p, _)| p == &player_entity)
            {
                if damage.1 >= CommanderRules::COMMANDER_DAMAGE_THRESHOLD {
                    // Player has lost due to commander damage
                    commands.spawn(PlayerEliminatedEvent {
                        player: player_entity,
                        reason: EliminationReason::CommanderDamage(commander.owner),
                    });
                    break;
                }
            }
        }
    }
}

/// Record commander damage from combat
pub fn record_commander_damage(
    mut commander_query: Query<&mut Commander>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for event in damage_events.read() {
        // Only process commander combat damage
        if !event.source_is_commander || !event.is_combat_damage || event.damage == 0 {
            continue;
        }

        if let Ok(mut commander) = commander_query.get_mut(event.source) {
            // Update the commander's damage tracking
            if let Some(damage_entry) = commander
                .damage_dealt
                .iter_mut()
                .find(|(p, _)| *p == event.target)
            {
                // Update existing damage entry
                damage_entry.1 += event.damage;
            } else {
                // Add a new damage entry
                commander.damage_dealt.push((event.target, event.damage));
            }

            // Record that the commander dealt damage to this player this turn
            commander.dealt_combat_damage_this_turn.insert(event.target);
        }
    }
}

/// Handle commander changing zones
pub fn handle_commander_zone_change(
    mut commands: Commands,
    _zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut zone_events: EventReader<ZoneChangeEvent>,
    commander_query: Query<(Entity, &Commander)>,
) {
    for event in zone_events.read() {
        // Check if the card is a commander
        if let Ok((entity, commander)) = commander_query.get(event.card) {
            // Update the commander's zone status
            let new_zone = match event.destination {
                Zone::CommandZone => CommanderZoneLocation::CommandZone,
                Zone::Battlefield => CommanderZoneLocation::Battlefield,
                Zone::Graveyard => CommanderZoneLocation::Graveyard,
                Zone::Exile => CommanderZoneLocation::Exile,
                Zone::Hand => CommanderZoneLocation::Hand,
                Zone::Library => CommanderZoneLocation::Library,
                Zone::Stack => CommanderZoneLocation::Stack,
            };

            cmd_zone_manager.update_commander_zone(entity, new_zone);

            // Special handling for commander death/exile
            if (event.destination == Zone::Graveyard || event.destination == Zone::Exile)
                && (event.source == Zone::Battlefield || event.source == Zone::Stack)
            {
                // Spawn a choice event for the player
                commands.spawn(CommanderZoneChoiceEvent {
                    commander: entity,
                    owner: commander.owner,
                    current_zone: event.destination,
                    can_go_to_command_zone: true,
                });
            }
        }
    }
}

/// Process player choices for commander zone changes
pub fn process_commander_zone_choices(
    mut _commands: Commands,
    mut choice_events: EventReader<CommanderZoneChoiceEvent>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut _commander_query: Query<&mut Commander>,
) {
    for event in choice_events.read() {
        if event.can_go_to_command_zone {
            // Move the commander to the command zone
            zone_manager.move_card(
                event.commander,
                event.owner,
                event.current_zone,
                Zone::CommandZone,
            );

            // Update the commander zone status
            cmd_zone_manager
                .update_commander_zone(event.commander, CommanderZoneLocation::CommandZone);

            // Increment zone transition count
            let count = cmd_zone_manager
                .zone_transition_count
                .entry(event.commander)
                .or_insert(0);
            *count += 1;

            // Notify that the commander moved to the command zone
            info!("Commander moved to command zone");
        }
    }
}

/// Handle casting a commander from the command zone
pub fn handle_commander_casting(
    _commands: Commands,
    _zone_manager: ResMut<ZoneManager>,
    _cmd_zone_manager: ResMut<CommandZoneManager>,
    _commander_query: Query<&mut Commander>,
    _cards: Query<(Entity, &Card)>,
    // We would need other queries and inputs here
) {
    // Implementation will be added later
}

/// Validate that all cards in a player's deck match their commander's color identity
pub fn validate_commander_deck(
    _card_query: Query<(Entity, &Card)>,
    cmd_zone_manager: Res<CommandZoneManager>,
    player_query: Query<(Entity, &Player)>,
) -> HashMap<Entity, Vec<Entity>> {
    // Map to store players and their illegal cards
    let illegal_cards = HashMap::new();

    // For each player, check their deck against their commander's color identity
    for (player_entity, _) in player_query.iter() {
        let commanders = cmd_zone_manager.get_player_commanders(player_entity);
        if commanders.is_empty() {
            continue;
        }

        // Get combined color identity of all commanders
        let mut combined_identity = HashSet::new();
        for &commander in &commanders {
            if let Some(colors) = cmd_zone_manager.commander_colors.get(&commander) {
                combined_identity.extend(colors.iter().cloned());
            }
        }

        // TODO: This would need to check all cards in a player's deck
        // For now, this is just a placeholder implementation
    }

    illegal_cards
}

/// Track commander damage for UI display and game rules
pub fn track_commander_damage(
    _commands: Commands,
    _game_state: ResMut<GameMenuState>,
    _commanders: Query<(Entity, &Commander)>,
    _players: Query<Entity, With<Player>>,
    _cmd_zone_manager: Res<CommandZoneManager>,
    // We'll need a damage event/component to track actual damage
) {
    // Implementation will be added later
}

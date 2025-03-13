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
///
/// This system will be used during setup to initialize commander-related resources.
/// Currently not actively called as setup is handled elsewhere.
#[allow(dead_code)]
pub fn setup_commander(mut commands: Commands) {
    commands.insert_resource(CommandZone::default());
    commands.insert_resource(CommandZoneManager::default());
}

/// Calculate the mana cost of a Commander including the Commander tax
///
/// Commanders cost an additional {2} for each time they've been cast from the command zone previously.
#[allow(dead_code)]
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

/// Handle the casting of a commander from the command zone
///
/// This system will process commander casting, applying tax and moving
/// the commander to the appropriate zone in the future.
#[allow(dead_code)]
pub fn handle_commander_casting(
    _commands: Commands,
    _zone_manager: ResMut<ZoneManager>,
    _cmd_zone_manager: ResMut<CommandZoneManager>,
    _commander_query: Query<&mut Commander>,
    _cards: Query<(Entity, &Card)>,
    // We would need other queries and inputs here
) {
    // TODO: Implement commander casting logic
    todo!(
        "Implement commander casting from command zone, including tax calculation and zone transitions"
    );
}

/// Validate that a deck follows commander rules
///
/// This system checks that:
/// - Each card in the deck matches the commander's color identity
/// - The deck size is correct for the format
/// - Other commander-specific deckbuilding restrictions
#[allow(dead_code)]
pub fn validate_commander_deck(
    _card_query: Query<(Entity, &Card)>,
    cmd_zone_manager: Res<CommandZoneManager>,
    player_query: Query<(Entity, &Player)>,
) -> HashMap<Entity, Vec<Entity>> {
    // Map to store validation errors by player
    let mut validation_errors = HashMap::new();

    // TODO: For each player, check their deck against commander rules
    for (player_entity, _player) in player_query.iter() {
        // Get the player's commanders
        let commanders = cmd_zone_manager.get_player_commanders(player_entity);
        if commanders.is_empty() {
            // Add error: No commander
            validation_errors
                .entry(player_entity)
                .or_insert_with(Vec::new)
                .push(Entity::from_raw(0)); // Placeholder error entity
        }

        // TODO: Check color identity of each card against commander's color identity
        // TODO: Check deck size (99 cards + commander, or 98 cards + 2 partners)
    }

    validation_errors
}

/// Track combat damage dealt by commanders to players
///
/// This system will monitor and record damage from commanders to players.
/// This is important for the Commander damage rule (21 damage from a single commander).
#[allow(dead_code)]
pub fn track_commander_damage(
    _commands: Commands,
    _game_state: ResMut<GameMenuState>,
    _commanders: Query<(Entity, &Commander)>,
    _players: Query<Entity, With<Player>>,
    _cmd_zone_manager: Res<CommandZoneManager>,
    // We'll need a damage event/component to track actual damage
) {
    // TODO: Track damage from commanders to players
    todo!(
        "Implement tracking of commander damage; 21 damage from a single commander eliminates a player"
    );
}

/// Register all commander-related systems with the app
pub fn register_commander_systems(app: &mut App) {
    // Register events
    app.add_event::<CommanderZoneChoiceEvent>()
        .add_event::<PlayerEliminatedEvent>()
        .add_event::<CombatDamageEvent>();

    // Register systems that will run during the game
    app.add_systems(
        Update,
        (
            check_commander_damage_loss,
            record_commander_damage,
            handle_commander_zone_change,
            process_commander_zone_choices,
        )
            .run_if(in_state(GameMenuState::InGame)),
    );
}

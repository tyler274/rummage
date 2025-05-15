use crate::{
    cards::card::Card,
    game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager},
    mana::Mana,
    player::Player,
};
use bevy::prelude::*;
use std::collections::HashMap;

use super::components::Commander;
use super::components::{CommanderZoneLocation, EliminationReason};
use super::events::{CombatDamageEvent, CommanderZoneChoiceEvent, PlayerEliminatedEvent};
use super::resources::{CommandZone, CommandZoneManager};
use super::rules::CommanderRules;
use crate::game_engine::turns::TurnStartEvent;

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
    let mut final_cost = base_cost;

    // Get the commander's cast count and add tax
    let cast_count = cmd_zone_manager.get_cast_count(commander);
    final_cost.colorless += CommanderRules::calculate_tax(cast_count);

    final_cost
}

/// Check if any player has lost due to commander damage
pub fn check_commander_damage_loss(
    mut eliminated_events: EventWriter<PlayerEliminatedEvent>,
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
                    eliminated_events.write(PlayerEliminatedEvent {
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
    _zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut zone_events: EventReader<ZoneChangeEvent>,
    commander_query: Query<(Entity, &Commander)>,
    mut choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    for event in zone_events.read() {
        // Check if the card is a commander
        if let Ok((entity, commander)) = commander_query.get(event.card) {
            // Update the commander's zone status
            let new_zone = match event.destination {
                Zone::Command => CommanderZoneLocation::CommandZone,
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
                choice_events.write(CommanderZoneChoiceEvent {
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
                Zone::Command,
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

/// System to handle casting commanders from the command zone
#[allow(dead_code)]
pub fn handle_commander_casting(
    _commands: Commands,
    _zone_manager: ResMut<ZoneManager>,
    _cmd_zone_manager: ResMut<CommandZoneManager>,
    _commander_query: Query<&mut Commander>,
    _cards: Query<(Entity, &Card)>,
    // We would need other queries and inputs here
) {
    // TODO: Implement commander casting from command zone
    #[cfg(debug_assertions)]
    debug!("Commander casting system running");

    // Implementation will:
    // 1. Check if the card being cast is a commander
    // 2. If so, get the commander data (cast count, etc.)
    // 3. Calculate the commander tax (2 mana per previous cast)
    // 4. Apply the tax to the casting cost
    // 5. Move the commander from the command zone to the stack
    // 6. Increment the cast count
}

/// Validate commander decks according to the Commander format rules
#[allow(dead_code)]
pub fn validate_commander_deck(
    _card_query: Query<(Entity, &Card)>,
    _cmd_zone_manager: Res<CommandZoneManager>,
    player_query: Query<(Entity, &Player)>,
) -> HashMap<Entity, Vec<Entity>> {
    // TODO: Implement commander deck validation
    #[cfg(debug_assertions)]
    debug!("Validating commander decks");

    // A mapping of players to any deck validation errors
    let mut validation_errors: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // In Commander format, validation rules include:
    // 1. Deck must have exactly 100 cards (including commander)
    // 2. All cards must be in the commander's color identity
    // 3. No more than 1 copy of any non-basic land card
    // 4. Commander must be a legendary creature or have "can be your commander" text
    // 5. If using partners, they must both have partner ability

    // For now, skip validation and return empty errors
    for (player_entity, _) in player_query.iter() {
        validation_errors.insert(player_entity, Vec::new());
    }

    validation_errors
}

/// System to track damage dealt by commanders to players
pub fn track_commander_damage(
    _commands: Commands,
    commanders: Query<(Entity, &Commander)>,
    _players: Query<Entity, With<Player>>,
    _cmd_zone_manager: Res<CommandZoneManager>,
    // We'll need a damage event/component to track actual damage
) {
    // Only log if there's been commander damage dealt this turn
    #[cfg(debug_assertions)]
    {
        let commanders_with_damage_this_turn: Vec<(Entity, &Commander)> = commanders
            .iter()
            .filter(|(_, commander)| !commander.dealt_combat_damage_this_turn.is_empty())
            .collect();

        for (entity, commander) in commanders_with_damage_this_turn {
            for player in &commander.dealt_combat_damage_this_turn {
                if let Some(damage) = commander.damage_dealt.iter().find(|(p, _)| p == player) {
                    debug!(
                        "Commander {:?} dealt {} damage to player {:?}",
                        entity, damage.1, player
                    );
                }
            }
        }
    }

    // In Commander format, 21 or more damage from a single commander eliminates a player
    // Implementation will:
    // 1. Listen for combat damage events
    // 2. Check if the source is a commander
    // 3. If so, record the damage against the player
    // 4. Check if any player has taken 21+ damage from a single commander
    // 5. Eliminate players who have taken lethal commander damage
}

/// Resets commander damage tracking at the beginning of each turn
pub fn reset_commander_damage_tracking(
    mut commanders: Query<&mut Commander>,
    mut turn_events: EventReader<TurnStartEvent>,
) {
    // Only run when a new turn starts
    if turn_events.read().next().is_none() {
        return;
    }

    // Clear the damage tracking for the new turn
    for mut commander in &mut commanders {
        if !commander.dealt_combat_damage_this_turn.is_empty() {
            debug!("Resetting commander damage tracking for new turn");
            commander.dealt_combat_damage_this_turn.clear();
        }
    }
}

/// Register all commander-related systems with the app
#[allow(dead_code)]
pub fn register_commander_systems(app: &mut App) {
    // Register events
    app.add_event::<CommanderZoneChoiceEvent>()
        .add_event::<PlayerEliminatedEvent>()
        .add_event::<CombatDamageEvent>();

    // Register systems that will run during the game
    app.add_systems(
        Update,
        (
            track_commander_damage,
            handle_commander_zone_change,
            process_commander_zone_choices,
            check_commander_damage_loss,
            record_commander_damage,
            reset_commander_damage_tracking,
        )
            .run_if(crate::game_engine::game_state_condition),
    );

    // Setup systems
    app.add_systems(Startup, setup_commander);

    // TODO: Add commander casting system to appropriate phases
    // app.add_systems(
    //     Update,
    //     handle_commander_casting
    //         .run_if(in_state(GameState::Playing))
    //         .run_if(resource_exists::<GameStack>),
    // );
}

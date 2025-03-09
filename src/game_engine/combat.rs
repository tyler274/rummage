use crate::card::{Card, CardTypes, CreatureType};
use crate::game_engine::GameState;
use crate::game_engine::commander::{CombatDamageEvent, Commander, CommanderRules};
use crate::game_engine::turns::TurnManager;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::mana::Color;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Resource tracking the state of combat during a turn
#[derive(Resource)]
pub struct CombatState {
    /// Current attackers and defenders - maps attacker creature to defending player
    pub attackers: HashMap<Entity, Entity>,

    /// Maps attacking creature to its blocking creatures
    pub blockers: HashMap<Entity, Vec<Entity>>,

    /// Tracks whether each attacking creature is blocked or not
    pub blocked_status: HashMap<Entity, BlockedStatus>,

    /// Combat damage assignment - maps attacker to list of (target, damage) entries
    pub assigned_combat_damage: HashMap<Entity, Vec<(Entity, u32)>>,

    /// Pending combat damage events to be processed
    pub pending_combat_damage: Vec<CombatDamageEvent>,

    /// Tracks which players have been attacked this turn
    pub players_attacked_this_turn: HashSet<Entity>,

    /// Maps players to creatures attacking them
    pub creatures_attacking_each_player: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to players they must attack
    pub must_attack: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to players they cannot attack
    pub cannot_attack: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to what cannot block them
    pub cannot_be_blocked_by: HashMap<Entity, Vec<BlockRestriction>>,

    /// Commander damage tracking for this combat
    pub commander_damage_this_combat: HashMap<Entity, HashMap<Entity, u32>>,

    /// Tracks current phase of combat
    pub in_declare_attackers: bool,
    pub in_declare_blockers: bool,
    pub in_combat_damage: bool,

    /// For first strike/regular damage steps
    pub combat_damage_step_number: u8,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            attackers: HashMap::new(),
            blockers: HashMap::new(),
            blocked_status: HashMap::new(),
            assigned_combat_damage: HashMap::new(),
            pending_combat_damage: Vec::new(),
            players_attacked_this_turn: HashSet::new(),
            creatures_attacking_each_player: HashMap::new(),
            must_attack: HashMap::new(),
            cannot_attack: HashMap::new(),
            cannot_be_blocked_by: HashMap::new(),
            commander_damage_this_combat: HashMap::new(),
            in_declare_attackers: false,
            in_declare_blockers: false,
            in_combat_damage: false,
            combat_damage_step_number: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockedStatus {
    Unblocked,
    Blocked,
    BlockedButRemoved, // For creatures that were blocked but had all blockers removed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockRestriction {
    CreatureType(CreatureType),
    Power(Comparison, u32),
    Toughness(Comparison, u32),
    Color(Color),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
}

/// Event for when players declare attackers
#[derive(Event)]
pub struct DeclareAttackersEvent {
    pub attacking_player: Entity,
}

/// Event for when players declare blockers
#[derive(Event)]
pub struct DeclareBlockersEvent {
    pub defending_players: Vec<Entity>,
}

/// Event for assigning combat damage
#[derive(Event)]
pub struct AssignCombatDamageEvent {
    pub is_first_strike: bool,
}

/// Event generated when an attacker is declared
#[derive(Event)]
pub struct AttackerDeclaredEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

/// Event generated when a blocker is declared
#[derive(Event)]
pub struct BlockerDeclaredEvent {
    pub blocker: Entity,
    pub attacker: Entity,
}

/// Event that signals the beginning of combat
#[derive(Event)]
pub struct CombatBeginEvent {
    pub player: Entity,
}

/// Event that signals the end of combat
#[derive(Event)]
pub struct CombatEndEvent {
    pub player: Entity,
}

/// System to initialize the combat phase
pub fn initialize_combat_phase(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
    players: Query<Entity, With<Player>>,
) {
    // Clear combat state from previous combat
    combat_state.attackers.clear();
    combat_state.blockers.clear();
    combat_state.blocked_status.clear();
    combat_state.assigned_combat_damage.clear();
    combat_state.pending_combat_damage.clear();
    combat_state.creatures_attacking_each_player.clear();

    // Reset combat flags
    combat_state.in_declare_attackers = false;
    combat_state.in_declare_blockers = false;
    combat_state.in_combat_damage = false;
    combat_state.combat_damage_step_number = 0;

    // Prepare player tracking
    for player in players.iter() {
        combat_state
            .creatures_attacking_each_player
            .insert(player, Vec::new());
    }

    // Emit combat begin event for triggered abilities
    let active_player = turn_manager.active_player;
    commands.spawn(CombatBeginEvent {
        player: active_player,
    });
}

/// System to handle declaring attackers
pub fn declare_attackers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut attack_events: EventReader<AttackerDeclaredEvent>,
    creatures: Query<(Entity, &Card)>,
    commanders: Query<(Entity, &Commander)>,
) {
    combat_state.in_declare_attackers = true;

    for event in attack_events.read() {
        let attacker = event.attacker;
        let defender = event.defender;

        // Register the attacker and defender
        combat_state.attackers.insert(attacker, defender);
        combat_state.players_attacked_this_turn.insert(defender);

        // Initialize the blocked status
        combat_state
            .blocked_status
            .insert(attacker, BlockedStatus::Unblocked);

        // Register in the per-player tracking
        if let Some(attackers) = combat_state
            .creatures_attacking_each_player
            .get_mut(&defender)
        {
            attackers.push(attacker);
        }

        // Check if the attacker is a commander for damage tracking
        if let Ok((commander_entity, _)) = commanders.get(attacker) {
            // Initialize commander damage tracking for this combat
            if !combat_state
                .commander_damage_this_combat
                .contains_key(&defender)
            {
                combat_state
                    .commander_damage_this_combat
                    .insert(defender, HashMap::new());
            }

            if let Some(damage_map) = combat_state.commander_damage_this_combat.get_mut(&defender) {
                damage_map.insert(commander_entity, 0);
            }
        }
    }
}

/// System to handle declaring blockers
pub fn declare_blockers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut blocker_events: EventReader<BlockerDeclaredEvent>,
) {
    combat_state.in_declare_attackers = false;
    combat_state.in_declare_blockers = true;

    for event in blocker_events.read() {
        let blocker = event.blocker;
        let attacker = event.attacker;

        // Add blocker to the attacker's blockers list
        if let Some(blockers) = combat_state.blockers.get_mut(&attacker) {
            blockers.push(blocker);
        } else {
            combat_state.blockers.insert(attacker, vec![blocker]);
        }

        // Update the blocked status
        combat_state
            .blocked_status
            .insert(attacker, BlockedStatus::Blocked);
    }
}

/// System to assign combat damage
pub fn assign_combat_damage_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut damage_events: EventReader<AssignCombatDamageEvent>,
    creatures: Query<(Entity, &Card)>,
    commanders: Query<(Entity, &Commander)>,
) {
    combat_state.in_declare_blockers = false;
    combat_state.in_combat_damage = true;

    for _ in damage_events.read() {
        // Collect damage events first to avoid multiple mutable borrows
        let mut pending_damage_events = Vec::new();
        let mut commander_damage_updates = Vec::new();

        // Track which defenders need commander damage maps initialized
        let mut defenders_needing_maps = HashSet::new();

        // Process each attacker
        for (&attacker, &defender) in combat_state.attackers.iter() {
            let is_commander = commanders.contains(attacker);

            match combat_state.blocked_status.get(&attacker) {
                Some(BlockedStatus::Unblocked) => {
                    // Creature is unblocked, deal damage to defending player
                    if let Ok((_, card)) = creatures.get(attacker) {
                        if let crate::card::CardDetails::Creature(creature) = &card.card_details {
                            let damage = creature.power;

                            // Create a combat damage event
                            let damage_event = CombatDamageEvent {
                                source: attacker,
                                target: defender,
                                damage: damage as u32,
                                is_combat_damage: true,
                                source_is_commander: is_commander,
                            };

                            // Add to pending damage events
                            pending_damage_events.push(damage_event);

                            // Track commander damage if relevant
                            if is_commander {
                                defenders_needing_maps.insert(defender);
                                commander_damage_updates.push((defender, attacker, damage as u32));
                            }
                        }
                    }
                }
                Some(BlockedStatus::Blocked) => {
                    // Creature is blocked, deal damage to blockers
                    if let Some(blockers) = combat_state.blockers.get(&attacker) {
                        if let Ok((_, card)) = creatures.get(attacker) {
                            if let crate::card::CardDetails::Creature(creature) = &card.card_details
                            {
                                let power = creature.power;

                                // In a full implementation we would handle damage assignment to multiple blockers
                                // For now, we just distribute damage evenly (simplified)
                                let blocker_count = blockers.len() as u32;
                                if blocker_count > 0 {
                                    let damage_per_blocker = power as u32 / blocker_count;

                                    for &blocker in blockers {
                                        let damage_event = CombatDamageEvent {
                                            source: attacker,
                                            target: blocker,
                                            damage: damage_per_blocker,
                                            is_combat_damage: true,
                                            source_is_commander: is_commander,
                                        };

                                        // Add to pending damage events
                                        pending_damage_events.push(damage_event);
                                    }
                                }
                            }
                        }
                    }
                }
                Some(BlockedStatus::BlockedButRemoved) => {
                    // Creature was blocked but blockers were removed
                    // By MTG rules, this creature deals no combat damage
                }
                None => {
                    // No blocked status, shouldn't happen in normal flow
                    warn!("Attacker {:?} has no blocked status", attacker);
                }
            }
        }

        // Initialize commander damage maps for any defenders that need them
        for defender in defenders_needing_maps {
            if !combat_state
                .commander_damage_this_combat
                .contains_key(&defender)
            {
                combat_state
                    .commander_damage_this_combat
                    .insert(defender, HashMap::new());
            }
        }

        // Now that we've collected all damage events, update the combat state
        for damage_event in pending_damage_events {
            // Add to pending combat damage
            combat_state.pending_combat_damage.push(damage_event);
        }

        // Update commander damage tracking
        for (defender, attacker, damage) in commander_damage_updates {
            if let Some(damage_map) = combat_state.commander_damage_this_combat.get_mut(&defender) {
                damage_map.insert(attacker, damage);
            }
        }

        // Increment the combat damage step number
        combat_state.combat_damage_step_number += 1;
    }
}

/// System to process combat damage
pub fn process_combat_damage_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut game_state: ResMut<GameState>,
) {
    // Send out all pending damage events
    for event in combat_state.pending_combat_damage.drain(..) {
        commands.spawn(event.clone());

        // If target is a player, apply damage directly (would be handled by another system)
        // In the real implementation we would check if the entity is a player
        // and apply damage to their life total

        // Since GameState doesn't track player life totals directly,
        // we'll just emit events and let another system handle it
    }

    // Reset combat flags after all damage is processed
    combat_state.in_combat_damage = false;
}

/// System to clean up after combat
pub fn end_combat_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
) {
    // Clear all combat data
    combat_state.attackers.clear();
    combat_state.blockers.clear();
    combat_state.blocked_status.clear();
    combat_state.assigned_combat_damage.clear();
    combat_state.pending_combat_damage.clear();
    combat_state.commander_damage_this_combat.clear();

    // Emit combat end event for triggered abilities
    let active_player = turn_manager.active_player;
    commands.spawn(CombatEndEvent {
        player: active_player,
    });
}

/// Register all combat-related systems and events
pub fn register_combat_systems(app: &mut App) {
    app.insert_resource(CombatState::default())
        .add_event::<DeclareAttackersEvent>()
        .add_event::<DeclareBlockersEvent>()
        .add_event::<AssignCombatDamageEvent>()
        .add_event::<AttackerDeclaredEvent>()
        .add_event::<BlockerDeclaredEvent>()
        .add_event::<CombatBeginEvent>()
        .add_event::<CombatEndEvent>()
        .add_systems(
            Update,
            (
                initialize_combat_phase,
                declare_attackers_system,
                declare_blockers_system,
                assign_combat_damage_system,
                process_combat_damage_system,
                end_combat_system,
            ),
        );
}

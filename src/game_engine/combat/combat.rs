use crate::cards::CreatureType;
use crate::game_engine::commander::CombatDamageEvent;
use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use crate::mana::ManaColor;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

// Event types
#[derive(Event)]
pub struct DeclareAttackersEvent {
    pub player: Entity,
}

#[derive(Event)]
pub struct DeclareBlockersEvent {
    pub player: Entity,
}

#[derive(Event)]
pub struct AssignCombatDamageEvent {
    #[allow(dead_code)]
    pub is_first_strike: bool,
}

#[derive(Event)]
pub struct AttackerDeclaredEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Event)]
pub struct BlockerDeclaredEvent {
    pub blocker: Entity,
    pub attacker: Entity,
}

#[derive(Event)]
pub struct CombatBeginEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct CombatEndEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct DeclareAttackersStepBeginEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct DeclareAttackersStepEndEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct DeclareBlockersStepBeginEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct DeclareBlockersStepEndEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

#[derive(Event)]
pub struct CreatureAttacksEvent {
    #[allow(dead_code)]
    pub attacker: Entity,
    #[allow(dead_code)]
    pub defender: Entity,
}

#[derive(Event)]
pub struct CreatureBlocksEvent {
    #[allow(dead_code)]
    pub blocker: Entity,
    #[allow(dead_code)]
    pub attacker: Entity,
}

#[derive(Event)]
pub struct CreatureBlockedEvent {
    #[allow(dead_code)]
    pub attacker: Entity,
    #[allow(dead_code)]
    pub blocker: Entity,
}

#[derive(Event)]
pub struct CombatDamageCompleteEvent {
    #[allow(dead_code)]
    pub player: Entity,
}

// Combat state enums
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlockedStatus {
    Blocked,
    Unblocked,
}

#[derive(PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum BlockRestriction {
    CreatureType(CreatureType),
    Color(ManaColor),
    Power(Comparison, i32),
    Toughness(Comparison, i32),
}

#[derive(PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
}

/// Resource tracking the state of combat during a turn
#[derive(Resource, Default)]
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
    #[allow(dead_code)]
    pub players_attacked_this_turn: HashSet<Entity>,

    /// Maps players to creatures attacking them
    #[allow(dead_code)]
    pub creatures_attacking_each_player: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to players they must attack
    pub must_attack: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to players they cannot attack
    pub cannot_attack: HashMap<Entity, Vec<Entity>>,

    /// Combat restrictions - maps creatures to what cannot block them
    #[allow(dead_code)]
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

// Combat systems
pub fn initialize_combat_phase(
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
    mut combat_begin_events: EventWriter<CombatBeginEvent>,
) {
    // Clear previous combat state
    *combat_state = CombatState::default();

    // Emit combat begin event
    combat_begin_events.write(CombatBeginEvent {
        player: turn_manager.active_player,
    });
}

pub fn handle_declare_attackers_event(
    mut combat_state: ResMut<CombatState>,
    mut events: EventReader<DeclareAttackersEvent>,
    mut step_begin_events: EventWriter<DeclareAttackersStepBeginEvent>,
) {
    for event in events.read() {
        combat_state.in_declare_attackers = true;
        step_begin_events.write(DeclareAttackersStepBeginEvent {
            player: event.player,
        });
    }
}

pub fn declare_attackers_system(
    mut combat_state: ResMut<CombatState>,
    mut events: EventReader<AttackerDeclaredEvent>,
    mut creature_attacks_events: EventWriter<CreatureAttacksEvent>,
) {
    for event in events.read() {
        combat_state
            .attackers
            .insert(event.attacker, event.defender);
        combat_state
            .blocked_status
            .insert(event.attacker, BlockedStatus::Unblocked);
        creature_attacks_events.write(CreatureAttacksEvent {
            attacker: event.attacker,
            defender: event.defender,
        });
    }
}

pub fn handle_declare_blockers_event(
    mut combat_state: ResMut<CombatState>,
    mut events: EventReader<DeclareBlockersEvent>,
    mut step_begin_events: EventWriter<DeclareBlockersStepBeginEvent>,
) {
    for event in events.read() {
        combat_state.in_declare_blockers = true;
        step_begin_events.write(DeclareBlockersStepBeginEvent {
            player: event.player,
        });
    }
}

pub fn declare_blockers_system(
    mut combat_state: ResMut<CombatState>,
    mut events: EventReader<BlockerDeclaredEvent>,
    mut creature_blocks_events: EventWriter<CreatureBlocksEvent>,
    mut creature_blocked_events: EventWriter<CreatureBlockedEvent>,
) {
    for event in events.read() {
        if combat_state.attackers.contains_key(&event.attacker) {
            combat_state
                .blocked_status
                .insert(event.attacker, BlockedStatus::Blocked);
            combat_state
                .blockers
                .entry(event.attacker)
                .or_default()
                .push(event.blocker);
            creature_blocks_events.write(CreatureBlocksEvent {
                blocker: event.blocker,
                attacker: event.attacker,
            });
            creature_blocked_events.write(CreatureBlockedEvent {
                attacker: event.attacker,
                blocker: event.blocker,
            });
        }
    }
}

pub fn assign_combat_damage_system(
    _commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut events: EventReader<AssignCombatDamageEvent>,
) {
    for _event in events.read() {
        combat_state.in_combat_damage = true;
        // Handle damage assignment logic here
    }
}

pub fn process_combat_damage_system(
    _commands: Commands,
    mut combat_state: ResMut<CombatState>,
    _game_state: ResMut<GameState>,
    mut players: Query<&mut Player>,
) {
    // Clone the pending events to avoid borrow issues
    let pending_events = combat_state.pending_combat_damage.clone();

    // Track which players we've processed to avoid double-processing
    let mut processed_players = HashSet::new();

    for event in pending_events {
        // Check if target is a player
        if let Ok(mut player) = players.get_mut(event.target) {
            if processed_players.contains(&event.target) {
                continue; // Skip already processed players
            }

            // Apply damage
            player.life -= event.damage as i32;
            processed_players.insert(event.target);

            // Debug output
            info!(
                "Player {:?} took {} damage, life now {}",
                event.target, event.damage, player.life
            );

            // For commander damage, make sure it's tracked correctly
            if event.source_is_commander && event.is_combat_damage {
                info!(
                    "Tracking commander damage: {:?} -> {:?}: {}",
                    event.source, event.target, event.damage
                );
            }
        }
    }

    // Clear after processing
    combat_state.pending_combat_damage.clear();
    combat_state.in_combat_damage = false;
}

pub fn end_combat_system(
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
    mut combat_end_events: EventWriter<CombatEndEvent>,
) {
    // Clear all combat data
    combat_state.attackers.clear();
    combat_state.blockers.clear();
    combat_state.blocked_status.clear();
    combat_state.assigned_combat_damage.clear();
    combat_state.pending_combat_damage.clear();

    // In a complete implementation, we would update persistent commander damage here
    // but for now, we'll just clear the combat-specific tracking
    combat_state.commander_damage_this_combat.clear();

    // Reset combat flags
    combat_state.in_declare_attackers = false;
    combat_state.in_declare_blockers = false;
    combat_state.in_combat_damage = false;
    combat_state.combat_damage_step_number = 0;

    // Emit combat end event for triggered abilities
    let active_player = turn_manager.active_player;
    combat_end_events.write(CombatEndEvent {
        player: active_player,
    });
}

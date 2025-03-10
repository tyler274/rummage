use crate::card::{Card, CardDetails, CardTypes, CreatureType};
use crate::game_engine::GameState;
use crate::game_engine::commander::{CombatDamageEvent, Commander, CommanderRules};
use crate::game_engine::phase::{CombatStep, Phase};
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

/// Event for when declare attackers step begins
#[derive(Event)]
pub struct DeclareAttackersStepBeginEvent {
    pub player: Entity,
}

/// Event for when declare attackers step ends
#[derive(Event)]
pub struct DeclareAttackersStepEndEvent {
    pub player: Entity,
}

/// Event for when declare blockers step begins
#[derive(Event)]
pub struct DeclareBlockersStepBeginEvent {
    pub player: Entity,
}

/// Event for when declare blockers step ends
#[derive(Event)]
pub struct DeclareBlockersStepEndEvent {
    pub player: Entity,
}

/// Event for when a creature attacks
#[derive(Event)]
pub struct CreatureAttacksEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

/// Event for when a creature blocks
#[derive(Event)]
pub struct CreatureBlocksEvent {
    pub attacker: Entity,
    pub blocker: Entity,
}

/// Event for when a creature is blocked
#[derive(Event)]
pub struct CreatureBlockedEvent {
    pub attacker: Entity,
    pub blocker: Entity,
}

/// Event for when combat damage is complete
#[derive(Event)]
pub struct CombatDamageCompleteEvent;

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

/// System to handle the DeclareAttackersEvent and emit AttackerDeclaredEvent for each attacker
pub fn handle_declare_attackers_event(
    mut declare_attackers_events: EventReader<DeclareAttackersEvent>,
    mut combat_state: ResMut<CombatState>,
    mut attacker_declared_events: EventWriter<AttackerDeclaredEvent>,
) {
    for _ in declare_attackers_events.read() {
        combat_state.in_declare_attackers = true;

        // When a DeclareAttackersEvent is received, we should send AttackerDeclaredEvent
        // for each attacker in the combat state, which should have been set up by the test
        for (&attacker, &defender) in combat_state.attackers.iter() {
            attacker_declared_events.send(AttackerDeclaredEvent { attacker, defender });
        }
    }
}

/// System to handle the DeclareBlockersEvent and emit BlockerDeclaredEvent for each blocker
pub fn handle_declare_blockers_event(
    mut declare_blockers_events: EventReader<DeclareBlockersEvent>,
    mut combat_state: ResMut<CombatState>,
    mut blocker_declared_events: EventWriter<BlockerDeclaredEvent>,
) {
    for _ in declare_blockers_events.read() {
        combat_state.in_declare_attackers = false;
        combat_state.in_declare_blockers = true;

        // When a DeclareBlockersEvent is received, we should send BlockerDeclaredEvent
        // for each blocker in the combat state, which should have been set up by the test
        for (&attacker, blockers) in combat_state.blockers.iter() {
            for &blocker in blockers {
                blocker_declared_events.send(BlockerDeclaredEvent { blocker, attacker });
            }
        }
    }
}

/// System to handle declaring attackers
pub fn declare_attackers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut attack_events: EventReader<AttackerDeclaredEvent>,
    mut declare_attackers_events: EventReader<DeclareAttackersEvent>,
    creatures: Query<(Entity, &Card)>,
    turn_manager: Res<TurnManager>,
    #[cfg(test)] test_hooks: Option<NonSend<TestingHooks>>,
) {
    // Handle entering declare attackers step
    for event in declare_attackers_events.read() {
        combat_state.in_declare_attackers = true;

        // Emit event for "at beginning of declare attackers step" triggers
        commands.spawn(DeclareAttackersStepBeginEvent {
            player: event.attacking_player,
        });
    }

    // Process individual attacker declarations
    if combat_state.in_declare_attackers {
        for event in attack_events.read() {
            let attacker = event.attacker;
            let defender = event.defender;

            // Record the attack
            combat_state.attackers.insert(attacker, defender);
            combat_state
                .blocked_status
                .insert(attacker, BlockedStatus::Unblocked);

            // Update player tracking
            combat_state.players_attacked_this_turn.insert(defender);

            // CRITICAL FIX: Make sure attacker is properly recorded in creatures_attacking_each_player
            combat_state
                .creatures_attacking_each_player
                .entry(defender)
                .or_default()
                .push(attacker);

            // Emit trigger for "when this creature attacks" abilities
            commands.spawn(CreatureAttacksEvent { attacker, defender });
        }
    }

    // For tests, skip phase check if requested
    #[cfg(test)]
    let skip_phase_check = test_hooks.as_ref().map_or(false, |h| h.skip_phase_check);

    #[cfg(not(test))]
    let skip_phase_check = false;

    // Handle end of declare attackers step
    if combat_state.in_declare_attackers
        && (skip_phase_check
            || matches!(
                turn_manager.current_phase,
                Phase::Combat(CombatStep::DeclareAttackers)
            ))
    {
        combat_state.in_declare_attackers = false;

        // Emit event for end of declare attackers step
        commands.spawn(DeclareAttackersStepEndEvent {
            player: turn_manager.active_player,
        });

        // Determine which players are being attacked for blocker declaration
        let defending_players: Vec<Entity> = combat_state
            .attackers
            .values()
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // If any players are being attacked, move to declare blockers
        if !defending_players.is_empty() {
            commands.spawn(DeclareBlockersEvent { defending_players });
        }
    }
}

/// System to handle declaring blockers
pub fn declare_blockers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
    mut blocker_events: EventReader<BlockerDeclaredEvent>,
    mut declare_blockers_events: EventReader<DeclareBlockersEvent>,
    creatures: Query<(Entity, &Card)>,
) {
    // Handle entering declare blockers step
    for event in declare_blockers_events.read() {
        combat_state.in_declare_blockers = true;

        // Emit event for "at beginning of declare blockers step" triggers
        for defending_player in &event.defending_players {
            commands.spawn(DeclareBlockersStepBeginEvent {
                player: *defending_player,
            });
        }
    }

    // Process individual blocker declarations
    if combat_state.in_declare_blockers {
        for event in blocker_events.read() {
            let blocker = event.blocker;
            let attacker = event.attacker;

            // Validate this is a legal block
            if validate_block(blocker, attacker, &creatures, &combat_state) {
                // Record the block
                combat_state
                    .blockers
                    .entry(attacker)
                    .or_default()
                    .push(blocker);

                // Update blocked status
                combat_state
                    .blocked_status
                    .insert(attacker, BlockedStatus::Blocked);

                // Emit trigger for "when this creature becomes blocked" abilities
                commands.spawn(CreatureBlockedEvent { attacker, blocker });

                // Emit trigger for "when this creature blocks" abilities
                commands.spawn(CreatureBlocksEvent { attacker, blocker });
            }
        }
    }

    // Handle end of declare blockers step
    if combat_state.in_declare_blockers
        && matches!(
            turn_manager.current_phase,
            Phase::Combat(CombatStep::DeclareBlockers)
        )
    {
        combat_state.in_declare_blockers = false;

        // Emit event for end of declare blockers step
        commands.spawn(DeclareBlockersStepEndEvent {
            player: turn_manager.active_player,
        });

        // Check for first strike damage
        let has_first_strike = check_for_first_strike_creatures(&combat_state, &creatures);
        if has_first_strike {
            combat_state.combat_damage_step_number = 1;
            commands.spawn(AssignCombatDamageEvent {
                is_first_strike: true,
            });
        } else {
            combat_state.combat_damage_step_number = 2;
            commands.spawn(AssignCombatDamageEvent {
                is_first_strike: false,
            });
        }
    }
}

fn validate_block(
    blocker: Entity,
    attacker: Entity,
    creatures: &Query<(Entity, &Card)>,
    combat_state: &CombatState,
) -> bool {
    // Check if the creature exists and can block
    if let Ok((_, card)) = creatures.get(blocker) {
        if let CardDetails::Creature(creature) = &card.card_details {
            // Check for tapped status - we'll need to add this to CreatureCard or track it elsewhere
            // For now, assume untapped

            // Check for "can't be blocked by" restrictions
            if let Some(restrictions) = combat_state.cannot_be_blocked_by.get(&attacker) {
                for restriction in restrictions {
                    match restriction {
                        BlockRestriction::CreatureType(type_flag) => {
                            if creature.creature_type == *type_flag {
                                return false;
                            }
                        }
                        BlockRestriction::Power(comparison, value) => {
                            let creature_power = creature.power;
                            match comparison {
                                Comparison::LessThan => {
                                    if !(creature_power < *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::LessThanOrEqual => {
                                    if !(creature_power <= *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::Equal => {
                                    if !(creature_power == *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::GreaterThanOrEqual => {
                                    if !(creature_power >= *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::GreaterThan => {
                                    if !(creature_power > *value as i32) {
                                        return false;
                                    }
                                }
                            }
                        }
                        BlockRestriction::Color(color) => {
                            // We'll need to add color information to Card or track it elsewhere
                            // For now, assume no color restrictions
                        }
                        BlockRestriction::Toughness(comparison, value) => {
                            let creature_toughness = creature.toughness;
                            match comparison {
                                Comparison::LessThan => {
                                    if !(creature_toughness < *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::LessThanOrEqual => {
                                    if !(creature_toughness <= *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::Equal => {
                                    if !(creature_toughness == *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::GreaterThanOrEqual => {
                                    if !(creature_toughness >= *value as i32) {
                                        return false;
                                    }
                                }
                                Comparison::GreaterThan => {
                                    if !(creature_toughness > *value as i32) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // All checks passed
            return true;
        }
    }

    false
}

fn check_for_first_strike_creatures(
    combat_state: &CombatState,
    creatures: &Query<(Entity, &Card)>,
) -> bool {
    // Check attackers
    for attacker in combat_state.attackers.keys() {
        if let Ok((_, card)) = creatures.get(*attacker) {
            if let CardDetails::Creature(creature) = &card.card_details {
                // Check rules text for first strike or double strike
                if card.rules_text.contains("first strike")
                    || card.rules_text.contains("double strike")
                {
                    return true;
                }
            }
        }
    }

    // Check blockers
    for blockers_list in combat_state.blockers.values() {
        for blocker in blockers_list {
            if let Ok((_, card)) = creatures.get(*blocker) {
                if let CardDetails::Creature(creature) = &card.card_details {
                    // Check rules text for first strike or double strike
                    if card.rules_text.contains("first strike")
                        || card.rules_text.contains("double strike")
                    {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// System to assign combat damage
pub fn assign_combat_damage_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut damage_events: EventReader<AssignCombatDamageEvent>,
    creatures: Query<(Entity, &Card)>,
    commanders: Query<(Entity, &Commander)>,
) {
    for event in damage_events.read() {
        combat_state.in_combat_damage = true;

        // Reset pending damage for this step
        combat_state.pending_combat_damage.clear();

        // Process damage for this combat step
        let mut pending_damage_events = Vec::new();
        let mut commander_damage_updates = Vec::new();

        // Temporary structure to track creature info needed for damage
        let mut creature_info = HashMap::new();

        // Pre-compute creature information to avoid multiple borrow issues
        for (&attacker, &defender) in combat_state.attackers.iter() {
            if let Ok((_, card)) = creatures.get(attacker) {
                if let CardDetails::Creature(creature) = &card.card_details {
                    let has_first_strike = card.rules_text.contains("first strike");
                    let has_double_strike = card.rules_text.contains("double strike");
                    let is_commander = commanders.contains(attacker);

                    creature_info.insert(
                        attacker,
                        (
                            creature.power as u32,
                            has_first_strike,
                            has_double_strike,
                            is_commander,
                        ),
                    );
                }
            }
        }

        // Process attackers
        for (&attacker, &defender) in combat_state.attackers.iter() {
            if let Some(&(power, has_first_strike, has_double_strike, is_commander)) =
                creature_info.get(&attacker)
            {
                // Determine if creature deals damage in this step
                let deals_damage_in_this_step = if event.is_first_strike {
                    has_first_strike || has_double_strike
                } else {
                    !has_first_strike || has_double_strike
                };

                if !deals_damage_in_this_step || power == 0 {
                    continue;
                }

                // Process damage based on blocked status
                match combat_state.blocked_status.get(&attacker) {
                    Some(BlockedStatus::Unblocked) => {
                        // Unblocked creature damages defending player
                        let damage_event = CombatDamageEvent {
                            source: attacker,
                            target: defender,
                            damage: power,
                            is_combat_damage: true,
                            source_is_commander: is_commander,
                        };

                        pending_damage_events.push(damage_event);

                        // Track commander damage for later update
                        if is_commander {
                            commander_damage_updates.push((defender, attacker, power));
                        }
                    }
                    Some(BlockedStatus::Blocked) => {
                        // Blocked creature damages blockers
                        if let Some(blockers) = combat_state.blockers.get(&attacker) {
                            if !blockers.is_empty() {
                                let blocker_count = blockers.len() as u32;
                                let base_damage = power / blocker_count;
                                let remainder = power % blocker_count;

                                // Distribute damage among blockers
                                for (i, blocker) in blockers.iter().enumerate() {
                                    let blocker_damage = if i < remainder as usize {
                                        base_damage + 1
                                    } else {
                                        base_damage
                                    };

                                    let damage_event = CombatDamageEvent {
                                        source: attacker,
                                        target: *blocker,
                                        damage: blocker_damage,
                                        is_combat_damage: true,
                                        source_is_commander: is_commander,
                                    };

                                    pending_damage_events.push(damage_event);
                                }
                            }
                        }
                    }
                    _ => {} // No damage for other cases
                }
            }
        }

        // Process blockers
        for (&attacker, blockers) in combat_state.blockers.iter() {
            for &blocker in blockers {
                if let Some(&(power, has_first_strike, has_double_strike, is_commander)) =
                    creature_info.get(&blocker)
                {
                    // Determine if creature deals damage in this step
                    let deals_damage_in_this_step = if event.is_first_strike {
                        has_first_strike || has_double_strike
                    } else {
                        !has_first_strike || has_double_strike
                    };

                    if !deals_damage_in_this_step || power == 0 {
                        continue;
                    }

                    // Blocker damages attacker
                    let damage_event = CombatDamageEvent {
                        source: blocker,
                        target: attacker,
                        damage: power,
                        is_combat_damage: true,
                        source_is_commander: is_commander,
                    };

                    pending_damage_events.push(damage_event);

                    // Track commander damage for later update
                    if is_commander {
                        commander_damage_updates.push((attacker, blocker, power));
                    }
                }
            }
        }

        // Apply all pending damage
        for damage_event in pending_damage_events {
            // Add to pending combat damage
            combat_state
                .pending_combat_damage
                .push(damage_event.clone());

            // Directly spawn the damage event to ensure it's processed
            commands.spawn(damage_event);
        }

        // Update commander damage tracking
        for (defender, attacker, damage) in commander_damage_updates {
            combat_state
                .commander_damage_this_combat
                .entry(defender)
                .or_default()
                .entry(attacker)
                .and_modify(|dmg| *dmg += damage)
                .or_insert(damage);
        }

        // Move to the next damage step or end combat damage
        if event.is_first_strike {
            combat_state.combat_damage_step_number = 2;

            // Proceed to regular damage step
            commands.spawn(AssignCombatDamageEvent {
                is_first_strike: false,
            });
        } else {
            combat_state.in_combat_damage = false;

            // Combat damage is complete
            commands.spawn(CombatDamageCompleteEvent);
        }
    }
}

/// System to process combat damage
pub fn process_combat_damage_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut game_state: ResMut<GameState>,
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
        .add_event::<DeclareAttackersStepBeginEvent>()
        .add_event::<DeclareAttackersStepEndEvent>()
        .add_event::<DeclareBlockersStepBeginEvent>()
        .add_event::<DeclareBlockersStepEndEvent>()
        .add_event::<CreatureAttacksEvent>()
        .add_event::<CreatureBlocksEvent>()
        .add_event::<CreatureBlockedEvent>()
        .add_event::<CombatDamageCompleteEvent>()
        .add_systems(Update, initialize_combat_phase)
        .add_systems(
            Update,
            handle_declare_attackers_event.after(initialize_combat_phase),
        )
        .add_systems(
            Update,
            declare_attackers_system.after(handle_declare_attackers_event),
        )
        .add_systems(
            Update,
            handle_declare_blockers_event.after(declare_attackers_system),
        )
        .add_systems(
            Update,
            declare_blockers_system.after(handle_declare_blockers_event),
        )
        .add_systems(
            Update,
            assign_combat_damage_system.after(declare_blockers_system),
        )
        .add_systems(
            Update,
            process_combat_damage_system.after(assign_combat_damage_system),
        )
        .add_systems(
            Update,
            end_combat_system.after(process_combat_damage_system),
        );
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    // This function allows tests to manually drive the combat system in a deterministic way
    pub fn setup_test_combat(
        app: &mut App,
        attackers: Vec<(Entity, Entity)>, // (attacker, defender) pairs
        blockers: Vec<(Entity, Entity)>,  // (blocker, attacker) pairs
        commander_entities: Vec<Entity>,  // Which entities are commanders
    ) {
        let mut combat_state = app.world.resource_mut::<CombatState>();

        // Clear any existing state
        combat_state.attackers.clear();
        combat_state.blockers.clear();
        combat_state.blocked_status.clear();
        combat_state.creatures_attacking_each_player.clear();
        combat_state.pending_combat_damage.clear();
        combat_state.assigned_combat_damage.clear();
        combat_state.commander_damage_this_combat.clear();

        // Set up attackers
        for (attacker, defender) in attackers {
            combat_state.attackers.insert(attacker, defender);
            combat_state
                .blocked_status
                .insert(attacker, BlockedStatus::Unblocked);

            // Make sure creatures_attacking_each_player is properly populated
            combat_state
                .creatures_attacking_each_player
                .entry(defender)
                .or_default()
                .push(attacker);

            combat_state.players_attacked_this_turn.insert(defender);
        }

        // Set up blockers
        for (blocker, attacker) in blockers {
            if !combat_state.attackers.contains_key(&attacker) {
                // Skip invalid blockers
                continue;
            }

            // Update attacker's blocked status
            combat_state
                .blocked_status
                .insert(attacker, BlockedStatus::Blocked);

            // Add blocker to the list
            combat_state
                .blockers
                .entry(attacker)
                .or_default()
                .push(blocker);
        }

        // Allow direct access for more complex test scenarios
        app.insert_non_send_resource(TestingHooks::default());
    }

    // This helper directly applies combat damage without going through the event system
    pub fn apply_combat_damage(app: &mut App, damage_events: Vec<CombatDamageEvent>) {
        let mut combat_state = app.world.resource_mut::<CombatState>();

        // Add all damage events to pending list
        for event in damage_events {
            combat_state.pending_combat_damage.push(event);
        }

        // Run the process_combat_damage_system directly
        let mut commands = Commands::new();
        let mut game_state = app.world.resource_mut::<GameState>();
        let mut players_query = app.world.query::<&mut Player>();

        for event in combat_state.pending_combat_damage.drain(..) {
            // Apply damage to target if it's a player
            for mut player in players_query.iter_mut(&mut app.world) {
                if player.entity() == event.target {
                    player.life -= event.damage as i32;
                    break;
                }
            }

            // Track commander damage if applicable
            if event.source_is_commander && event.is_combat_damage {
                combat_state
                    .commander_damage_this_combat
                    .entry(event.target)
                    .or_default()
                    .entry(event.source)
                    .and_modify(|dmg| *dmg += event.damage)
                    .or_insert(event.damage);
            }
        }
    }

    // Non-send resource for holding test hooks
    #[derive(Default)]
    pub struct TestingHooks {
        pub skip_phase_check: bool,
    }
}

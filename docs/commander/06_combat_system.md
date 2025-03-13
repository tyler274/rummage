# Combat System

## Overview

The Combat System module handles the complex interactions of multiplayer combat in Commander games. It manages attacker and blocker declaration, combat damage assignment, and special combat triggers, with support for attacking any player and handling multiplayer-specific combat mechanics.

## Core Components

### Combat State Resource

```rust
#[derive(Resource)]
pub struct CombatState {
    // Current attackers and defenders
    pub attackers: HashMap<Entity, Entity>, // Creature -> Defending player
    pub blockers: HashMap<Entity, Vec<Entity>>, // Attacking creature -> Blocking creatures
    pub blocked_status: HashMap<Entity, BlockedStatus>, // Attacking creature -> Blocked status
    
    // Combat damage assignment
    pub assigned_combat_damage: HashMap<Entity, Vec<(Entity, u32)>>, // Attacker -> [(target, damage)]
    pub pending_combat_damage: Vec<CombatDamageEvent>,
    
    // Multiplayer specific tracking
    pub players_attacked_this_turn: HashSet<Entity>,
    pub creatures_attacking_each_player: HashMap<Entity, Vec<Entity>>, // Player -> Attacking creatures
    
    // Combat restrictions
    pub must_attack: HashMap<Entity, Vec<Entity>>, // Creature -> Players that must be attacked
    pub cannot_attack: HashMap<Entity, Vec<Entity>>, // Creature -> Players that cannot be attacked
    pub cannot_be_blocked_by: HashMap<Entity, Vec<BlockRestriction>>, // Creature -> Block restrictions
    
    // Commander damage tracking
    pub commander_damage_this_combat: HashMap<Entity, HashMap<Entity, u32>>, // Defender -> (Commander -> Damage)
    
    // Combat phases tracking
    pub in_declare_attackers: bool,
    pub in_declare_blockers: bool,
    pub in_combat_damage: bool,
    pub combat_damage_step_number: u8, // For first strike/regular damage steps
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
```

### Combat Events

```rust
#[derive(Event)]
pub struct DeclareAttackersEvent {
    pub attacking_player: Entity,
}

#[derive(Event)]
pub struct DeclareBlockersEvent {
    pub defending_players: Vec<Entity>,
}

#[derive(Event)]
pub struct AssignCombatDamageEvent {
    pub is_first_strike: bool,
}

#[derive(Event)]
pub struct CombatDamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub damage: u32,
    pub is_combat_damage: bool,
    pub source_is_commander: bool,
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
```

## Key Systems

### Combat Initialization

```rust
fn initialize_combat_phase(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut turn_manager: ResMut<TurnManager>,
    players: Query<Entity, With<CommanderPlayer>>,
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
        combat_state.creatures_attacking_each_player.insert(player, Vec::new());
    }
    
    // Emit combat begin event for triggered abilities
    let active_player = turn_manager.player_order[turn_manager.active_player_index];
    commands.spawn(CombatBeginEvent { player: active_player });
}
```

### Declare Attackers System

```rust
fn declare_attackers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut turn_manager: ResMut<TurnManager>,
    mut attack_events: EventReader<AttackerDeclaredEvent>,
    mut declare_attackers_events: EventReader<DeclareAttackersEvent>,
    creatures: Query<(Entity, &CreatureCard, Option<&CommanderCard>)>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    // Handle entering declare attackers step
    for event in declare_attackers_events.read() {
        combat_state.in_declare_attackers = true;
        
        // Emit event for "at beginning of declare attackers step" triggers
        commands.spawn(DeclareAttackersStepBeginEvent { 
            player: event.attacking_player 
        });
    }
    
    // Process individual attacker declarations
    if combat_state.in_declare_attackers {
        for event in attack_events.read() {
            let attacker = event.attacker;
            let defender = event.defender;
            
            // Validate this is a legal attack
            if validate_attack(attacker, defender, &creatures, &combat_state) {
                // Record the attack
                combat_state.attackers.insert(attacker, defender);
                combat_state.blocked_status.insert(attacker, BlockedStatus::Unblocked);
                
                // Update player tracking
                combat_state.players_attacked_this_turn.insert(defender);
                combat_state.creatures_attacking_each_player
                    .entry(defender)
                    .or_default()
                    .push(attacker);
                    
                // Emit trigger for "when this creature attacks" abilities
                commands.spawn(CreatureAttacksEvent {
                    attacker,
                    defender,
                });
                
                // Check if attacker is a commander for special handling
                if let Ok((_, _, Some(_commander))) = creatures.get(attacker) {
                    // Handle commander-specific attack triggers if needed
                }
            }
        }
    }
    
    // Handle end of declare attackers step
    if combat_state.in_declare_attackers && turn_manager.all_players_passed {
        combat_state.in_declare_attackers = false;
        
        // Emit event for end of declare attackers step
        let attacking_player = turn_manager.player_order[turn_manager.active_player_index];
        commands.spawn(DeclareAttackersStepEndEvent { 
            player: attacking_player 
        });
        
        // Determine which players are being attacked for blocker declaration
        let defending_players: Vec<Entity> = combat_state.attackers.values()
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
            
        // If any players are being attacked, move to declare blockers
        if !defending_players.is_empty() {
            commands.spawn(DeclareBlockersEvent { 
                defending_players 
            });
        }
    }
}

fn validate_attack(
    attacker: Entity,
    defender: Entity,
    creatures: &Query<(Entity, &CreatureCard, Option<&CommanderCard>)>,
    combat_state: &CombatState,
) -> bool {
    // Check if the creature exists and can attack
    if let Ok((_, creature, _)) = creatures.get(attacker) {
        // Check for summoning sickness, tapped status, etc.
        if creature.has_summoning_sickness || creature.is_tapped {
            return false;
        }
        
        // Check attack restrictions
        if let Some(restricted_targets) = combat_state.must_attack.get(&attacker) {
            if !restricted_targets.contains(&defender) {
                return false;
            }
        }
        
        if let Some(forbidden_targets) = combat_state.cannot_attack.get(&attacker) {
            if forbidden_targets.contains(&defender) {
                return false;
            }
        }
        
        // All checks passed
        return true;
    }
    
    false
}
```

### Declare Blockers System

```rust
fn declare_blockers_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut turn_manager: ResMut<TurnManager>,
    mut blocker_events: EventReader<BlockerDeclaredEvent>,
    mut declare_blockers_events: EventReader<DeclareBlockersEvent>,
    creatures: Query<(Entity, &CreatureCard)>,
) {
    // Handle entering declare blockers step
    for event in declare_blockers_events.read() {
        combat_state.in_declare_blockers = true;
        
        // Emit event for "at beginning of declare blockers step" triggers
        for defending_player in &event.defending_players {
            commands.spawn(DeclareBlockersStepBeginEvent { 
                player: *defending_player 
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
                combat_state.blockers
                    .entry(attacker)
                    .or_default()
                    .push(blocker);
                    
                // Update blocked status
                combat_state.blocked_status.insert(attacker, BlockedStatus::Blocked);
                
                // Emit trigger for "when this creature becomes blocked" abilities
                commands.spawn(CreatureBlockedEvent {
                    attacker,
                    blocker,
                });
                
                // Emit trigger for "when this creature blocks" abilities
                commands.spawn(CreatureBlocksEvent {
                    attacker,
                    blocker,
                });
            }
        }
    }
    
    // Handle end of declare blockers step
    if combat_state.in_declare_blockers && turn_manager.all_players_passed {
        combat_state.in_declare_blockers = false;
        
        // Emit event for end of declare blockers step
        let attacking_player = turn_manager.player_order[turn_manager.active_player_index];
        commands.spawn(DeclareBlockersStepEndEvent { 
            player: attacking_player 
        });
        
        // Check for first strike damage
        let has_first_strike = check_for_first_strike_creatures(&combat_state, &creatures);
        if has_first_strike {
            combat_state.combat_damage_step_number = 1;
            commands.spawn(AssignCombatDamageEvent { 
                is_first_strike: true 
            });
        } else {
            combat_state.combat_damage_step_number = 2;
            commands.spawn(AssignCombatDamageEvent { 
                is_first_strike: false 
            });
        }
    }
}

fn validate_block(
    blocker: Entity,
    attacker: Entity,
    creatures: &Query<(Entity, &CreatureCard)>,
    combat_state: &CombatState,
) -> bool {
    // Check if the creature exists and can block
    if let Ok((_, creature)) = creatures.get(blocker) {
        // Check for tapped status, etc.
        if creature.is_tapped {
            return false;
        }
        
        // Check for "can't be blocked by" restrictions
        if let Some(restrictions) = combat_state.cannot_be_blocked_by.get(&attacker) {
            for restriction in restrictions {
                match restriction {
                    BlockRestriction::CreatureType(type_flag) => {
                        if creature.creature_type.contains(*type_flag) {
                            return false;
                        }
                    },
                    BlockRestriction::Power(comparison, value) => {
                        let creature_power = creature.power + creature.power_modifier as i32;
                        match comparison {
                            Comparison::LessThan => if !(creature_power < *value as i32) { return false; },
                            Comparison::LessThanOrEqual => if !(creature_power <= *value as i32) { return false; },
                            Comparison::Equal => if !(creature_power == *value as i32) { return false; },
                            Comparison::GreaterThanOrEqual => if !(creature_power >= *value as i32) { return false; },
                            Comparison::GreaterThan => if !(creature_power > *value as i32) { return false; },
                        }
                    },
                    // Handle other restriction types...
                    _ => {}
                }
            }
        }
        
        // All checks passed
        return true;
    }
    
    false
}

fn check_for_first_strike_creatures(
    combat_state: &CombatState,
    creatures: &Query<(Entity, &CreatureCard)>,
) -> bool {
    // Check attackers
    for attacker in combat_state.attackers.keys() {
        if let Ok((_, creature)) = creatures.get(*attacker) {
            if creature.has_first_strike || creature.has_double_strike {
                return true;
            }
        }
    }
    
    // Check blockers
    for blockers_list in combat_state.blockers.values() {
        for blocker in blockers_list {
            if let Ok((_, creature)) = creatures.get(*blocker) {
                if creature.has_first_strike || creature.has_double_strike {
                    return true;
                }
            }
        }
    }
    
    false
}
```

### Combat Damage System

```rust
fn assign_combat_damage_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut damage_events: EventReader<AssignCombatDamageEvent>,
    creatures: Query<(Entity, &CreatureCard, Option<&CommanderCard>)>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    for event in damage_events.read() {
        combat_state.in_combat_damage = true;
        
        // Reset pending damage for this step
        combat_state.pending_combat_damage.clear();
        
        // Handle first strike damage step
        if event.is_first_strike {
            // Process attackers with first strike
            for (attacker, defender) in combat_state.attackers.iter() {
                process_attacker_damage(
                    *attacker, 
                    *defender, 
                    true, 
                    &mut combat_state, 
                    &creatures, 
                    &players,
                    &mut commands
                );
            }
            
            // Process blockers with first strike
            for (attacker, blockers) in combat_state.blockers.iter() {
                for blocker in blockers {
                    process_blocker_damage(
                        *blocker, 
                        *attacker, 
                        true, 
                        &mut combat_state, 
                        &creatures,
                        &mut commands
                    );
                }
            }
        } 
        // Handle normal damage step
        else {
            // Process all attackers without first strike or with double strike
            for (attacker, defender) in combat_state.attackers.iter() {
                process_attacker_damage(
                    *attacker, 
                    *defender, 
                    false, 
                    &mut combat_state, 
                    &creatures, 
                    &players,
                    &mut commands
                );
            }
            
            // Process all blockers without first strike or with double strike
            for (attacker, blockers) in combat_state.blockers.iter() {
                for blocker in blockers {
                    process_blocker_damage(
                        *blocker, 
                        *attacker, 
                        false, 
                        &mut combat_state, 
                        &creatures,
                        &mut commands
                    );
                }
            }
        }
        
        // Apply all pending damage
        for damage_event in combat_state.pending_combat_damage.iter() {
            commands.spawn_entity(damage_event.clone());
            
            // Track commander damage specifically
            if damage_event.source_is_commander && damage_event.is_combat_damage {
                combat_state.commander_damage_this_combat
                    .entry(damage_event.target)
                    .or_default()
                    .entry(damage_event.source)
                    .and_modify(|damage| *damage += damage_event.damage)
                    .or_insert(damage_event.damage);
            }
        }
        
        // Move to the next damage step or end combat damage
        if event.is_first_strike {
            combat_state.combat_damage_step_number = 2;
            
            // Proceed to regular damage step
            commands.spawn(AssignCombatDamageEvent { 
                is_first_strike: false 
            });
        } else {
            combat_state.in_combat_damage = false;
            
            // Combat damage is complete
            commands.spawn(CombatDamageCompleteEvent);
        }
    }
}

fn process_attacker_damage(
    attacker: Entity,
    defender: Entity,
    is_first_strike_step: bool,
    combat_state: &mut CombatState,
    creatures: &Query<(Entity, &CreatureCard, Option<&CommanderCard>)>,
    players: &Query<Entity, With<CommanderPlayer>>,
    commands: &mut Commands,
) {
    if let Ok((_, creature, commander)) = creatures.get(attacker) {
        // Check if creature deals damage in this step
        let deals_damage_in_this_step = if is_first_strike_step {
            creature.has_first_strike || creature.has_double_strike
        } else {
            !creature.has_first_strike || creature.has_double_strike
        };
        
        if !deals_damage_in_this_step {
            return;
        }
        
        let is_commander = commander.is_some();
        let damage = creature.power as u32;
        
        // If unblocked, damage goes to defending player
        if *combat_state.blocked_status.get(&attacker).unwrap() == BlockedStatus::Unblocked {
            combat_state.pending_combat_damage.push(CombatDamageEvent {
                source: attacker,
                target: defender,
                damage,
                is_combat_damage: true,
                source_is_commander: is_commander,
            });
        } 
        // If blocked, damage goes to blockers
        else if *combat_state.blocked_status.get(&attacker).unwrap() == BlockedStatus::Blocked {
            // For multiplayer we need distinct damage assignment
            // This is simplified - full implementation would need player input
            if let Some(blockers) = combat_state.blockers.get(&attacker) {
                let blocker_count = blockers.len() as u32;
                if blocker_count > 0 {
                    let base_damage = damage / blocker_count;
                    let remainder = damage % blocker_count;
                    
                    for (i, blocker) in blockers.iter().enumerate() {
                        let damage_to_assign = if i < remainder as usize {
                            base_damage + 1
                        } else {
                            base_damage
                        };
                        
                        combat_state.pending_combat_damage.push(CombatDamageEvent {
                            source: attacker,
                            target: *blocker,
                            damage: damage_to_assign,
                            is_combat_damage: true,
                            source_is_commander: is_commander,
                        });
                    }
                }
            }
        }
    }
}

fn process_blocker_damage(
    blocker: Entity,
    attacker: Entity,
    is_first_strike_step: bool,
    combat_state: &mut CombatState,
    creatures: &Query<(Entity, &CreatureCard, Option<&CommanderCard>)>,
    commands: &mut Commands,
) {
    if let Ok((_, creature, commander)) = creatures.get(blocker) {
        // Check if creature deals damage in this step
        let deals_damage_in_this_step = if is_first_strike_step {
            creature.has_first_strike || creature.has_double_strike
        } else {
            !creature.has_first_strike || creature.has_double_strike
        };
        
        if !deals_damage_in_this_step {
            return;
        }
        
        let is_commander = commander.is_some();
        let damage = creature.power as u32;
        
        // Blocker damage goes to the attacker
        combat_state.pending_combat_damage.push(CombatDamageEvent {
            source: blocker,
            target: attacker,
            damage,
            is_combat_damage: true,
            source_is_commander: is_commander,
        });
    }
}
```

## Multiplayer Combat Mechanics

### Multi-player Attack Validation

```rust
fn validate_multiplayer_attack(
    attacker: Entity,
    defender: Entity,
    active_player: Entity,
    creatures: &Query<(Entity, &CreatureCard)>,
    players: &Query<(Entity, &CommanderPlayer)>,
) -> bool {
    // Check if defending player has effects that prevent attacks
    if let Ok((_, player)) = players.get(defender) {
        if !player.can_be_attacked {
            return false;
        }
    }
    
    // Special Commander multiplayer restrictions
    if defender == active_player {
        // Can't attack yourself
        return false;
    }
    
    // Additional Commander-specific attack restrictions
    // (e.g., Goad mechanics, Vow of Duty effects, etc.)
    
    true
}
```

### Commander Damage Tracking

```rust
fn update_commander_damage_totals(
    mut commands: Commands,
    mut damage_events: EventReader<CombatDamageEvent>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    commanders: Query<(Entity, &CommanderCard)>,
    game_state: Res<CommanderGameState>,
) {
    for event in damage_events.read() {
        // Only process combat damage from commanders to players
        if event.source_is_commander && event.is_combat_damage {
            // Find the damaged player
            if let Ok((player_entity, mut player)) = players.get_mut(event.target) {
                // Find the commander's owner
                if let Ok((_, commander)) = commanders.get(event.source) {
                    let owner = commander.owner;
                    
                    // Update commander damage tracking
                    let damage_entry = player.commander_damage_received
                        .entry(owner)
                        .or_insert(0);
                    *damage_entry += event.damage;
                    
                    // Check for lethal commander damage (21 points)
                    if *damage_entry >= game_state.commander_damage_threshold {
                        commands.spawn(GameEvent::PlayerEliminated {
                            player: player_entity,
                            reason: EliminationReason::CommanderDamage(owner),
                        });
                    }
                }
            }
        }
    }
}
```

### Political Combat Effects

```rust
fn apply_political_combat_effects(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut political_events: EventReader<PoliticalCombatEvent>,
    creatures: Query<Entity, With<CreatureCard>>,
) {
    for event in political_events.read() {
        match event.effect_type {
            PoliticalCombatEffect::Goad { creature, goaded_by } => {
                // Goaded creatures must attack a player other than the goad source
                for entity in &event.affected_creatures {
                    combat_state.must_attack
                        .entry(*entity)
                        .or_default()
                        .extend(get_other_players(goaded_by));
                        
                    combat_state.cannot_attack
                        .entry(*entity)
                        .or_default()
                        .push(goaded_by);
                }
            },
            PoliticalCombatEffect::Vow { creature, protected_player } => {
                // Creatures with vows can't attack the protected player
                for entity in &event.affected_creatures {
                    combat_state.cannot_attack
                        .entry(*entity)
                        .or_default()
                        .push(protected_player);
                }
            },
            // Other political combat effects...
        }
    }
}

// Get all players except the specified one
fn get_other_players(excluded_player: Entity) -> Vec<Entity> {
    // This would need to be implemented to get all players except the excluded one
    vec![] 
}
```

## Integration Points

- **Game State Module**: Tracks combat results and state
- **Player Module**: Updates player life totals and tracks elimination
- **Turn Structure**: Coordinates phase transitions for combat
- **Command Zone**: Processes commander damage and death triggers
- **Zone Management**: Handles creature movement between zones during combat

## Testing Strategy

1. **Unit Tests**:
   - Test attacker and blocker validation
   - Verify combat damage calculation
   - Test multiplayer attack restrictions
   
2. **Integration Tests**:
   - Test full combat sequences
   - Verify commander damage tracking
   - Test political combat effects
   - Verify complex multiplayer scenarios

## Design Considerations

- Handling complex multiplayer attack dynamics
- Tracking commander damage separately from regular damage
- Supporting political and multiplayer-specific mechanics
- Efficient handling of combat with many creatures/players
- Providing clear combat feedback to the user interface 

## Edge Cases in Commander Combat

### Multiplayer Politics during Combat

The Commander format introduces unique political dynamics that affect combat. Here's how we handle these edge cases:

```rust
fn handle_political_combat_effects(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    politics: Res<PoliticsSystem>,
    declare_attackers_events: EventReader<DeclareAttackersEvent>,
    combat_phase_events: EventReader<CombatPhaseEvent>,
) {
    // Track deals and agreements affecting combat
    for event in combat_phase_events.read() {
        if matches!(event, CombatPhaseEvent::Begin) {
            // Reset per-combat political effects
            combat_state.political_restrictions.clear();
            combat_state.political_requirements.clear();
            
            // Apply all active deals that affect combat
            for deal in politics.active_deals.iter() {
                match &deal.deal_type {
                    DealType::NonAttackAgreement { players, duration } => {
                        // Add mutual non-attack agreements to combat restrictions
                        for &player in players {
                            combat_state.political_restrictions.entry(player)
                                .or_default()
                                .cannot_be_attacked_by.extend(players.iter().filter(|p| **p != player));
                        }
                    },
                    DealType::AttackSpecificPlayer { attacker, target, duration } => {
                        // Add "must attack specific player" requirement
                        combat_state.political_requirements.entry(*attacker)
                            .or_default()
                            .must_attack_player = Some(*target);
                    },
                    // Other political deal types...
                }
            }
        }
    }
}
```

### Goad Effects in Multiplayer

Goad is a Commander-specific mechanic that forces creatures to attack and restricts which players they can attack:

```rust
fn apply_goad_effects(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    politics: Res<PoliticsSystem>,
    creatures: Query<(Entity, &CreatureCard)>,
    combat_phase_events: EventReader<CombatPhaseEvent>,
) {
    for event in combat_phase_events.read() {
        if matches!(event, CombatPhaseEvent::BeginDeclareAttackers) {
            // Apply all active goad effects
            for (creature, goad_effects) in politics.goad_effects.iter() {
                if goad_effects.is_empty() {
                    continue;
                }
                
                // Create a set of players that cannot be attacked due to goad
                let mut cannot_attack_players = HashSet::new();
                for goad in goad_effects {
                    // The controller of the goad effect cannot be attacked
                    cannot_attack_players.insert(goad.source);
                }
                
                // Apply goad restrictions
                combat_state.attack_restrictions.entry(*creature)
                    .or_default()
                    .cannot_attack_players.extend(cannot_attack_players);
                    
                // Apply goad requirements (must attack if able)
                combat_state.attack_requirements.entry(*creature)
                    .or_default()
                    .forced_to_attack_if_able = true;
            }
        }
    }
}
```

### Handling Multiple Attacking Requirements

Commander introduces effects like goad, curse effects, and political deals that can create conflicting requirements:

```rust
fn resolve_attacking_requirements(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    creatures: Query<(Entity, &CreatureCard)>,
    players: Query<Entity, With<CommanderPlayer>>,
) -> HashMap<Entity, Vec<Entity>> {
    let mut valid_attacks = HashMap::new();
    
    // For each creature that could potentially attack
    for (creature, _) in creatures.iter() {
        let requirements = combat_state.attack_requirements.get(&creature).cloned()
            .unwrap_or_default();
        let restrictions = combat_state.attack_restrictions.get(&creature).cloned()
            .unwrap_or_default();
        
        // Start with all players as potential targets
        let mut potential_targets: Vec<Entity> = players.iter().collect();
        
        // Apply restrictions (cannot attack specific players)
        if !restrictions.cannot_attack_players.is_empty() {
            potential_targets.retain(|player| !restrictions.cannot_attack_players.contains(player));
        }
        
        // Apply requirements (must attack specific player if able)
        if let Some(required_player) = requirements.must_attack_player {
            if potential_targets.contains(&required_player) {
                // Can only attack the required player
                potential_targets = vec![required_player];
            }
            // If can't attack required player, can attack anyone else
        }
        
        // Apply goad requirements (attack any player other than goading player)
        if requirements.forced_to_attack_if_able {
            // Must attack someone if able
            if potential_targets.is_empty() {
                // Can't satisfy the requirement, so creature doesn't attack
                continue;
            }
        } else if !requirements.forced_to_attack_if_able {
            // Not forced to attack, so can choose not to attack
            // Add a "no attack" option
            potential_targets.push(Entity::PLACEHOLDER); // Special value for "no attack"
        }
        
        valid_attacks.insert(creature, potential_targets);
    }
    
    valid_attacks
}
```

### Handling Commander Damage when Controlling Another Player's Turn

Commander introduces mind control effects where you might control another player during combat:

```rust
fn track_damage_during_controlled_turns(
    mut commands: Commands,
    combat_state: Res<CombatState>,
    turn_control: Query<(&TurnControl, &CommanderPlayer)>,
    commanders: Query<(Entity, &Commander)>,
    damage_events: EventReader<CombatDamageEvent>,
) {
    for event in damage_events.read() {
        // Only process commander combat damage
        if !event.source_is_commander || !event.is_combat_damage {
            continue;
        }
        
        // Check if the active player is controlling another player's turn
        let active_player = combat_state.active_player;
        if let Ok((turn_control, _)) = turn_control.get(active_player) {
            if turn_control.is_controlling_other_turn {
                // The controller of the turn doesn't matter for commander damage
                // Only the owner of the commander matters
                
                // Find the actual owner of the commander
                if let Ok((_, commander)) = commanders.get(event.source) {
                    // Track damage based on the original owner
                    commands.spawn(CommanderDamageTrackedEvent {
                        source: event.source,
                        target: event.target,
                        amount: event.damage,
                        source_owner: commander.owner, // Use original owner
                        controlled_by: active_player,  // Current controller
                    });
                }
            }
        }
    }
}
```

### Multi-player Combat with Protection and Redirection

Commander games often have effects that redirect or prevent damage:

```rust
fn handle_redirected_commander_damage(
    mut commands: Commands,
    mut damage_events: EventReader<CombatDamageEvent>,
    mut redirection_events: EventReader<DamageRedirectionEvent>,
    commanders: Query<(Entity, &Commander)>,
) {
    // Track pending damage before redirection
    let mut pending_commander_damage = HashMap::new();
    
    // Record all commander damage events
    for event in damage_events.read() {
        if event.source_is_commander && event.is_combat_damage {
            pending_commander_damage.insert(
                (event.source, event.target),
                event.damage
            );
        }
    }
    
    // Apply redirection effects
    for event in redirection_events.read() {
        let original_source_target = (event.original_source, event.original_target);
        
        // Check if this is redirecting commander damage
        if let Some(damage) = pending_commander_damage.get(&original_source_target) {
            // Critical rule: Commander damage is still tracked as coming from the commander
            // even if damage is physically dealt by another source
            if let Ok((commander_entity, _)) = commanders.get(event.original_source) {
                commands.spawn(CommanderDamageRedirectedEvent {
                    original_source: event.original_source,
                    original_target: event.original_target,
                    new_target: event.new_target,
                    amount: *damage,
                    redirection_source: event.redirection_source,
                });
            }
        }
    }
}
```

### Complex Combat Math in Multiplayer

Commander games often have complex combat math with multiple attackers, blockers, and effects:

```rust
fn optimize_multiplayer_combat_damage(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    blockers: Query<(Entity, &CreatureCard)>,
    attackers: Query<(Entity, &CreatureCard)>,
) {
    // Pre-compute damage assignment for multiplayer scenarios
    // For large combat scenarios, this optimizes performance
    
    // Build a bipartite graph of possible assignments
    let mut attacker_graph: HashMap<Entity, Vec<(Entity, u32)>> = HashMap::new();
    let mut blocker_graph: HashMap<Entity, Vec<(Entity, u32)>> = HashMap::new();
    
    // For each attacking creature
    for (attacker, attacker_card) in attackers.iter() {
        if !combat_state.attackers.contains_key(&attacker) {
            continue;
        }
        
        let blockers_for_attacker = combat_state.blockers.get(&attacker)
            .cloned()
            .unwrap_or_default();
            
        // Skip if unblocked
        if blockers_for_attacker.is_empty() {
            continue;
        }
        
        // Calculate initial even distribution
        let blocker_count = blockers_for_attacker.len() as u32;
        let attacker_power = attacker_card.power as u32;
        let base_damage = attacker_power / blocker_count;
        let remainder = attacker_power % blocker_count;
        
        // Initial assignment - will be optimized later
        let mut assignments = Vec::new();
        for (i, blocker) in blockers_for_attacker.iter().enumerate() {
            let damage = if i < remainder as usize {
                base_damage + 1
            } else {
                base_damage
            };
            
            assignments.push((*blocker, damage));
        }
        
        attacker_graph.insert(attacker, assignments);
    }
    
    // Optimize damage assignment
    let optimized_assignments = optimize_damage_assignment(
        attacker_graph,
        blocker_graph,
        &combat_state
    );
    
    // Store optimized assignments
    combat_state.optimized_damage_assignments = optimized_assignments;
}

fn optimize_damage_assignment(
    attacker_graph: HashMap<Entity, Vec<(Entity, u32)>>,
    blocker_graph: HashMap<Entity, Vec<(Entity, u32)>>,
    combat_state: &CombatState
) -> HashMap<Entity, HashMap<Entity, u32>> {
    // This would implement a more sophisticated assignment algorithm
    // For multiplayer games with complex blocking scenarios
    
    // For example, prioritizing:
    // 1. Lethal damage to important blockers
    // 2. Minimizing overkill damage
    // 3. Accounting for special abilities (deathtouch, etc.)
    
    // Placeholder simplified implementation
    let mut result = HashMap::new();
    
    for (attacker, assignments) in attacker_graph {
        let mut attacker_assignments = HashMap::new();
        for (blocker, damage) in assignments {
            attacker_assignments.insert(blocker, damage);
        }
        result.insert(attacker, attacker_assignments);
    }
    
    result
}
```

### Commander Damage from Multiple Combat Steps

Commander games can have extra combat phases and modified turn structures:

```rust
fn track_commander_damage_across_combats(
    mut commands: Commands,
    mut per_combat_damage: Local<HashMap<(Entity, Entity), u32>>,
    mut per_turn_damage: Local<HashMap<(Entity, Entity), u32>>,
    combat_phase_events: EventReader<CombatPhaseEvent>,
    damage_events: EventReader<CombatDamageEvent>,
    commanders: Query<(Entity, &Commander)>,
    game_state: Res<CommanderGameState>,
) {
    // Reset tracking at the beginning of each combat
    for event in combat_phase_events.read() {
        if matches!(event, CombatPhaseEvent::Begin) {
            per_combat_damage.clear();
        }
        
        // Reset turn tracking at the beginning of a turn
        if matches!(event, CombatPhaseEvent::Begin) && 
            game_state.current_step == TurnStep::BeginningPhase {
            per_turn_damage.clear();
        }
    }
    
    // Track damage in this combat
    for event in damage_events.read() {
        if event.source_is_commander && event.is_combat_damage {
            let key = (event.source, event.target);
            
            // Update per-combat tracking
            *per_combat_damage.entry(key).or_insert(0) += event.damage;
            
            // Update per-turn tracking
            *per_turn_damage.entry(key).or_insert(0) += event.damage;
            
            // If we've reached threshold in this combat, emit event
            if *per_combat_damage.get(&key).unwrap() >= 21 {
                commands.spawn(CombatThresholdReachedEvent {
                    source: event.source,
                    target: event.target,
                    damage_type: ThresholdType::CommanderDamage,
                });
            }
        }
    }
}
```

### Commander Combat with Split Second

Split second effects can occur during combat in Commander games:

```rust
fn handle_split_second_during_combat(
    mut commands: Commands,
    split_second_effects: Query<Entity, With<SplitSecondEffect>>,
    mut damage_events: EventReader<CombatDamageEvent>,
    stack: Res<Stack>,
) {
    // Check if there's a split second spell/ability on the stack
    let split_second_active = split_second_effects.iter().count() > 0;
    
    if split_second_active {
        // Special handling for combat damage with split second
        // Damage still occurs and is tracked, but responses are prevented
        
        for event in damage_events.read() {
            if event.source_is_commander && event.is_combat_damage {
                // Track damage as normal, but mark that it occurred under split second
                commands.spawn(SplitSecondDamageEvent {
                    original_event: event.clone(),
                    allows_responses: false,
                });
            }
        }
    }
}
```

### Phasing and Commander Damage

Phasing is a complex edge case in Commander combat:

```rust
fn handle_phased_commander_damage(
    mut commands: Commands,
    phased_permanents: Query<(Entity, &PhasedStatus, &Commander)>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for event in damage_events.read() {
        // Check if this event involves a commander that's phased out
        if let Ok((entity, phased, commander)) = phased_permanents.get(event.source) {
            if phased.is_phased_out {
                // A phased out permanent can't deal damage
                // This is a sanity check - shouldn't happen in normal gameplay
                commands.spawn(RuleViolationEvent {
                    rule: "A phased out permanent can't deal combat damage",
                    entity: entity,
                });
                continue;
            }
        }
        
        // Check for commander damage to a phased permanent
        // Similar logic for phased-out targets
    }
}
```

### Time Stamps and Combat Modification Order

Combat with multiple effects modifying combat values:

```rust
fn apply_combat_modifications_in_timestamp_order(
    mut commands: Commands,
    creatures: Query<(Entity, &CreatureCard, &CombatModifications)>,
    timestamp_ordering: Res<TimestampOrdering>,
) {
    // Sort all modifications by timestamp
    let mut modifications = Vec::new();
    
    for (entity, _, combat_mods) in creatures.iter() {
        for modification in &combat_mods.modifications {
            modifications.push((
                entity,
                modification.clone(),
                timestamp_ordering.get_timestamp(modification.source)
            ));
        }
    }
    
    // Sort by timestamp
    modifications.sort_by_key(|(_, _, timestamp)| *timestamp);
    
    // Apply modifications in timestamp order
    for (entity, modification, _) in modifications {
        apply_single_combat_modification(
            entity,
            modification,
            &mut commands
        );
    }
}
```

## Testing Multiplayer Commander Combat Edge Cases

The testing framework must account for various Commander-specific combat complexities:

```rust
#[test]
fn test_multiplayer_commander_combat_edge_cases() {
    let mut app = App::new();
    app.add_plugins(CommanderCombatTestPlugin);
    
    // Test cases for complex Commander combat scenarios
    
    // 1. Combat with multiple players and goad effects
    // 2. Commander damage tracking across multiple combat phases
    // 3. Commander damage with damage doubling/tripling effects
    // 4. Interaction between commander damage and damage prevention
    // 5. Protection effects during multiplayer combat
    // 6. Complex blocking scenarios with many attackers/blockers
    // 7. Combat with phased permanents
    // 8. Split second effects during combat
    // 9. Multiple players taking extra combat phases
    // 10. Political effects influencing combat outcomes
    
    // Test execution...
}
``` 
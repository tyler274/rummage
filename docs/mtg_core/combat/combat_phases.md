# Combat Phases

## Overview

The combat phase in Magic: The Gathering is divided into five distinct steps, each with specific rules and opportunities for player interaction. This document details how these steps are implemented in Rummage's core engine.

## Beginning of Combat Step

The Beginning of Combat step marks the start of the combat phase. During this step:

1. "At the beginning of combat" triggered abilities go on the stack
2. Players receive priority, starting with the active player
3. This is the last opportunity to use effects that would prevent creatures from attacking

### Implementation

```rust
pub fn beginning_of_combat_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut phase_events: EventWriter<BeginningOfCombatEvent>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
) {
    // Generate beginning of combat event
    phase_events.send(BeginningOfCombatEvent {
        active_player: game_state.active_player,
    });
    
    // Check for triggered abilities that trigger at beginning of combat
    // Add them to the stack
    // ...
}
```

## Declare Attackers Step

During the Declare Attackers step:

1. The active player declares attackers
2. Creatures attack as a group
3. The active player taps attacking creatures (unless they have vigilance)
4. "Whenever a creature attacks" triggered abilities go on the stack
5. Players receive priority, starting with the active player

### Attacking Rules

- Only untapped creatures controlled by the active player can be declared as attackers
- Each attacking creature must attack either an opponent or a planeswalker an opponent controls
- Creatures with summoning sickness can't attack unless they have haste
- Attacking doesn't use the stack and can't be responded to directly

### Implementation

```rust
pub fn declare_attackers_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut attack_events: EventReader<DeclareAttackEvent>,
    mut creatures: Query<(Entity, &mut Creature, &Controller)>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
) {
    // Process attack declarations
    for attack_event in attack_events.iter() {
        // Validate attackers (untapped, controlled by active player, etc.)
        // Record which creatures are attacking and what they're attacking
        // Tap attacking creatures without vigilance
        // ...
    }
    
    // Generate triggered abilities for "whenever a creature attacks"
    // ...
}
```

## Declare Blockers Step

During the Declare Blockers step:

1. The defending player(s) declare blockers
2. Each blocking creature must block exactly one attacking creature
3. "Whenever a creature blocks" triggered abilities go on the stack
4. The active player declares the damage assignment order for creatures blocked by multiple creatures
5. Players receive priority, starting with the active player

### Blocking Rules

- Only untapped creatures can block
- Each creature can block only one attacker (unless it has special abilities)
- Multiple creatures can block a single attacker
- Blocking doesn't use the stack and can't be responded to directly

### Implementation

```rust
pub fn declare_blockers_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut block_events: EventReader<DeclareBlockEvent>,
    mut creatures: Query<(Entity, &Creature, &Controller)>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
) {
    // Process block declarations
    for block_event in block_events.iter() {
        // Validate blockers (untapped, etc.)
        // Record which creatures are blocking and what they're blocking
        // ...
    }
    
    // Generate triggered abilities for "whenever a creature blocks" or "becomes blocked"
    // ...
    
    // Determine damage assignment order for multiple blockers
    // ...
}
```

## Combat Damage Step

During the Combat Damage step:

1. If any creatures have first strike or double strike, a separate First Strike Combat Damage step occurs first
2. The active player assigns combat damage from their attacking creatures
3. The defending player(s) assign combat damage from their blocking creatures
4. All combat damage is dealt simultaneously
5. Players receive priority, starting with the active player

### Damage Assignment Rules

- Each attacking creature assigns damage equal to its power
- Each blocking creature assigns damage equal to its power
- Blocked creatures assign their damage to the blocking creatures
- Unblocked creatures assign their damage to the player or planeswalker they're attacking
- If multiple creatures block an attacker, the attacker's controller decides how to distribute the damage

### Implementation

```rust
pub fn combat_damage_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    combat_state: Res<CombatState>,
    mut creatures: Query<(Entity, &Creature, &mut Health)>,
    mut players: Query<(Entity, &Player, &mut Life)>,
    mut planeswalkers: Query<(Entity, &Planeswalker, &mut Loyalty)>,
) {
    // Handle first strike damage if needed
    // ...
    
    // Assign and deal combat damage
    for (attacker, targets) in combat_state.attackers.iter() {
        // Calculate damage amount
        // Apply damage to appropriate targets
        // Handle special abilities (deathtouch, lifelink, etc.)
        // ...
    }
    
    // Check for state-based actions after damage
    // ...
}
```

## End of Combat Step

During the End of Combat step:

1. "At end of combat" triggered abilities go on the stack
2. Players receive priority, starting with the active player
3. After this step, all creatures and planeswalkers are removed from combat

### Implementation

```rust
pub fn end_of_combat_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut combat_state: ResMut<CombatState>,
    mut phase_events: EventWriter<EndOfCombatEvent>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
) {
    // Generate end of combat event
    phase_events.send(EndOfCombatEvent {
        active_player: game_state.active_player,
    });
    
    // Check for triggered abilities that trigger at end of combat
    // ...
    
    // Remove all creatures from combat
    combat_state.attackers.clear();
    combat_state.blockers.clear();
    // ...
}
```

## Format-Specific Extensions

Different MTG formats may extend these basic combat phases with additional rules:

- **Commander**: Tracks commander damage separately
- **Two-Headed Giant**: Modifies how combat damage is dealt to players
- **Multiplayer Formats**: Have special rules for attacking multiple opponents

For details on format-specific combat mechanics, see the respective format documentation.

---

Next: [First Strike and Double Strike](first_strike.md) 
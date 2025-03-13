# State-Based Actions

## Overview

State-Based Actions (SBAs) are checks that the game automatically performs regularly, ensuring game rules are enforced without requiring player actions. These checks occur whenever a player would receive priority and before any player actually receives priority.

## Core Functionality

In Commander, state-based actions handle crucial game state validations:

- Player loss conditions (life total at or below 0, drawing from an empty library, poison counters)
- Commander damage tracking (21+ damage from a single commander)
- Creature destruction (zero or negative toughness, lethal damage)
- Planeswalker loyalty management
- Aura and Equipment attachment rules
- Legendary permanents uniqueness rule
- Token existence rules
- Counters on permanents (especially +1/+1 and -1/-1 counter interaction)

## Implementation

The state-based actions system is implemented as a collection of systems that run between steps and phases:

```rust
#[derive(Resource)]
pub struct StateBasedActionSystem {
    // Configuration for SBA processing
    pub enabled: bool,
    pub last_check_time: Duration,
    pub check_frequency: Duration,
    
    // Tracking for specific SBAs
    pub pending_legendary_checks: Vec<Entity>,
    pub pending_destruction: Vec<Entity>,
    pub pending_life_checks: Vec<Entity>,
}

// Systems that implement specific checks
pub fn check_player_loss_conditions(/* ... */);
pub fn check_creature_destruction(/* ... */);
pub fn check_attachment_validity(/* ... */);
pub fn check_legendary_rule(/* ... */);
pub fn check_token_existence(/* ... */);
pub fn check_counter_interactions(/* ... */);
```

## Special Commander Rules

Commander format introduces specific state-based actions:

- **Commander Damage**: When a player has been dealt 21 or more combat damage by the same commander, they lose the game
- **Commander Zone Transitions**: Commanders can be moved to the command zone instead of other zones
- **Color Identity**: Cards that would be added to a library, hand, battlefield or graveyard are checked against the commander's color identity

## Multiplayer Considerations

In multiplayer Commander:

- State-based actions are checked simultaneously for all players and permanents
- Player elimination is handled through SBAs, with appropriate triggers firing
- When a player leaves the game, all cards they own leave the game (with special rules for controlled permanents)

## Optimization

The SBA system is optimized to:

- Only check relevant game objects (using spatial partitioning where appropriate)
- Use efficient data structures to track pending actions
- Batch similar checks where possible
- Cache results when appropriate 
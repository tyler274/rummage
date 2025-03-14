# State-Based Actions in MTG

## Overview

State-Based Actions (SBAs) are one of the foundational mechanisms of Magic: The Gathering's rules system. They serve as automatic checks that the game performs regularly to ensure game rules are properly enforced without requiring explicit player actions. These checks occur whenever a player would receive priority and before any player actually receives priority.

## Core Functionality

State-based actions handle many crucial game state validations:

- **Player loss conditions**:
  - A player with 0 or less life loses the game
  - A player attempting to draw from an empty library loses the game
  - A player with 10 or more poison counters loses the game (in formats where poison counters are relevant)

- **Creature conditions**:
  - Creatures with toughness 0 or less are put into their owner's graveyard
  - Creatures with damage marked on them greater than or equal to their toughness are destroyed
  - Creatures that have been dealt damage by a source with deathtouch are destroyed

- **Permanent-specific rules**:
  - Planeswalker with no loyalty counters is put into its owner's graveyard
  - Auras that aren't attached to anything or attached illegally are put into their owner's graveyard
  - Equipment or Fortifications that are attached illegally become unattached
  - Legendary permanents with the same name are put into their owner's graveyard except the most recently played one

- **Token and counter rules**:
  - A token that has left the battlefield ceases to exist
  - If a permanent has both +1/+1 and -1/-1 counters on it, they cancel out in pairs

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

## Application Order

State-based actions are applied in a specific order, but all applicable state-based actions are performed simultaneously as a single event. If performing state-based actions creates new conditions for state-based actions, those new state-based actions will be checked in the next round of checks.

This is especially important for situations like:
- A player at 0 life with a creature dying simultaneously
- Multiple legendary permanents entering the battlefield at the same time
- Creatures with damage marked receiving -X/-X effects

## Multiplayer Considerations

In multiplayer games:

- State-based actions are checked simultaneously for all players and permanents
- When a player leaves the game, all cards they own leave the game
- Objects controlled by a player who left the game are exiled, unless they're controlled by a different player's ongoing effect

## Optimization in Rummage

The SBA system in Rummage is optimized to:

- Only check relevant game objects (using spatial partitioning where appropriate)
- Use efficient data structures to track pending actions
- Batch similar checks where possible
- Cache results when appropriate
- Minimize performance impact during complex game states

## Related Documentation

- [Turn Structure](../turn_structure/index.md): When state-based actions are checked during the turn
- [Zones](../zones/index.md): How state-based actions interact with game zones
- [Combat](../combat/index.md): Combat-specific state-based actions 
# Politics System Testing

## Overview

This document outlines the testing approach for the Multiplayer Politics system within the Commander game engine. Due to the complex nature of political interactions and their impact on game state, thorough testing is essential to ensure correct behavior and integration with other components.

## Test Categories

The Politics system testing is organized into several key categories:

### 1. Unit Tests

Unit tests focus on isolated functionality of the Politics system components:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monarch_transitions() {
        // Test monarch assignment and transitions
    }
    
    #[test]
    fn test_vote_tallying() {
        // Test vote counting and resolution
    }
    
    #[test]
    fn test_deal_validation() {
        // Test deal validation logic
    }
}
```

### 2. Integration Tests

Integration tests verify Politics system interactions with other game systems:

- Combat integration (goad effects, combat restrictions)
- Turn structure integration (monarch draw effects)
- Card effects that interact with political mechanics
- UI feedback for political events

### 3. End-to-End Tests

End-to-end tests simulate full game scenarios involving political elements:

- Four-player game with complex political interactions
- AI decision-making in political contexts
- Network synchronization of political state

## Testing The Monarch Mechanic

The Monarch mechanic requires specific test cases:

1. **Monarch Assignment**
   - Test initial monarch assignment
   - Test monarch transfer through card effects
   - Test monarch transfer through combat damage

2. **Monarch Effects**
   - Verify correct card draw during end step
   - Test interaction with replacement effects
   - Validate monarch status persistence across turns

3. **Edge Cases**
   - Test monarch behavior when player loses/leaves game
   - Test simultaneous monarch-granting effects

## Testing The Voting System

The voting system tests include:

1. **Vote Initialization**
   - Test vote creation and option definition
   - Validate vote accessibility to all players
   - Test timing restrictions on votes

2. **Vote Casting**
   - Test basic vote casting mechanics
   - Test vote weight modifiers
   - Validate vote time limits

3. **Vote Resolution**
   - Test tie-breaking logic
   - Verify correct application of voting results
   - Test effects that modify vote outcomes

## Testing The Deal System

The deal system requires comprehensive testing:

1. **Deal Creation**
   - Test deal proposal structure
   - Validate term specification
   - Test duration settings

2. **Deal Negotiation**
   - Test acceptance/rejection mechanics
   - Test counter-proposal handling
   - Validate notification system

3. **Deal Enforcement**
   - Test automatic deal monitoring
   - Validate deal violation detection
   - Test consequences application

4. **Deal History**
   - Test deal history tracking
   - Validate reputation system updates
   - Test history influence on AI decisions

## Mock Testing Approaches

For complex political scenarios, mock testing is employed:

```rust
#[test]
fn test_complex_political_scenario() {
    let mut app = App::new();
    
    // Setup test environment
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_systems(Startup, setup_test_scenario);
       
    // Mock player actions
    let players = setup_four_player_game(&mut app);
    
    // Simulate complex political scenario
    simulate_monarchy_contest(&mut app, &players);
    simulate_deal_making(&mut app, &players);
    simulate_voting_session(&mut app, &players);
    
    // Verify expected outcomes
    verify_political_state(&app, &expected_state);
}
```

## Property-Based Testing

For robust validation of political mechanics, property-based testing is used:

```rust
#[test]
fn test_voting_properties() {
    // Property: All votes must be counted
    proptest!(|(votes: Vec<(Entity, VoteChoice)>)| {
        let result = tally_votes(&votes);
        assert_eq!(result.total_votes, votes.len());
    });
    
    // Property: Winning option must have most votes
    proptest!(|(votes: Vec<(Entity, VoteChoice)>)| {
        let result = tally_votes(&votes);
        // Verify winning option has highest count
    });
}
```

## Network Testing

Political systems must be tested for correct network behavior:

1. **State Synchronization**
   - Test monarch status synchronization
   - Validate vote transmission and collection
   - Test deal state synchronization

2. **Latency Handling**
   - Test delayed political decisions
   - Validate timeout handling for votes/deals
   - Test recovery from connection issues

3. **Conflict Resolution**
   - Test resolution of conflicting political actions
   - Validate deterministic outcomes across peers

## Regression Test Suite

The Politics system maintains a regression test suite covering:

1. Known past issues and their fixes
2. Edge cases discovered during development
3. Community-reported political interaction bugs

## Performance Testing

Political mechanics are tested for performance impacts:

1. **Scaling Tests**
   - Performance with many simultaneous deals
   - Vote tallying with large player counts
   - Political state update propagation

2. **Memory Usage**
   - Test memory footprint of complex political states
   - Validate cleanup of expired political objects

## Test Fixtures

Reusable test fixtures include:

- Standard 4-player table setup
- Pre-defined political scenarios
- Mock AI political personalities
- Reference deal/vote templates

## Continuous Integration

Politics tests are integrated into CI with:

- Automated test runs on PR submission
- Coverage reports for political mechanics
- Performance comparison against baseline 
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
    
    #[test]
    fn test_alliance_formation() {
        // Test alliance creation and tracking
    }
    
    #[test]
    fn test_goad_effects() {
        // Test goad mechanics and expiration
    }
}
```

### 2. Integration Tests

Integration tests verify Politics system interactions with other game systems:

- Combat integration (goad effects, alliance-based combat restrictions)
- Turn structure integration (monarch draw effects, deal expiration)
- Card effects that interact with political mechanics
- UI feedback for political events
- Threat assessment impact on AI decision making

### 3. End-to-End Tests

End-to-end tests simulate full game scenarios involving political elements:

- Four-player game with complex political interactions
- AI decision-making in political contexts
- Network synchronization of political state
- Full game simulations with political mechanics enabled

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
   - Test prevention of monarch transfer

```rust
#[test]
fn test_monarch_combat_transfer() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn(Player).id();
    let player2 = app.world.spawn(Player).id();
    
    // Set initial monarch
    let mut monarch_state = MonarchState {
        current_monarch: Some(player1),
        last_changed: Some(0.0),
    };
    app.insert_resource(monarch_state);
    
    // Player entity should have Monarch component
    app.world.entity_mut(player1).insert(Monarch);
    
    // Create attacking creature
    let creature = app.world.spawn((
        CardName("Test Creature".to_string()),
        CardType::Creature,
        Power(3),
        Toughness(3),
        Controller(player2),
        OnBattlefield,
    )).id();
    
    // Send combat damage event
    app.world.send_event(CombatDamageEvent {
        source: creature,
        target: player1,
        amount: 3,
        is_commander_damage: false,
        attacking_player_controller: player2,
        defending_player: player1,
    });
    
    app.update();
    
    // Verify monarch transferred
    let updated_monarch = app.world.resource::<MonarchState>();
    assert_eq!(updated_monarch.current_monarch, Some(player2));
    assert!(app.world.entity(player2).contains::<Monarch>());
    assert!(!app.world.entity(player1).contains::<Monarch>());
}
```

## Testing The Voting System

The voting system tests include:

1. **Vote Initialization**
   - Test vote creation and option definition
   - Validate vote accessibility to all players
   - Test timing restrictions on votes

2. **Vote Casting**
   - Test basic vote casting mechanics
   - Test vote weight modifiers (e.g., extra votes from effects)
   - Validate vote time limits
   - Test voting for multiple options (Council's Dilemma)

3. **Vote Resolution**
   - Test tie-breaking logic
   - Verify correct application of voting results
   - Test effects that modify vote outcomes
   - Test scaling effects based on vote counts

```rust
#[test]
fn test_voting_resolution() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn(Player).id();
    let player2 = app.world.spawn(Player).id();
    let player3 = app.world.spawn(Player).id();
    
    // Create voting card
    let voting_card = app.world.spawn((
        CardName("Council's Judgment".to_string()),
        VotingMechanic {
            voting_type: VotingType::WillOfCouncil,
            options: vec!["Option A".to_string(), "Option B".to_string()],
            resolution: VoteResolutionMethod::MostVotes,
        },
        Controller(player1),
    )).id();
    
    // Cast votes
    let votes = HashMap::from([
        (player1, "Option A".to_string()),
        (player2, "Option B".to_string()),
        (player3, "Option A".to_string()),
    ]);
    
    // Send vote completion event
    app.world.send_event(VoteCompletionEvent {
        source_card: voting_card,
        votes,
        timestamp: 1.0,
    });
    
    app.update();
    
    // Verify correct effect was applied for winning option "Option A"
    // Further assertions based on the expected outcome
}
```

## Testing The Deal System

The deal system requires comprehensive testing:

1. **Deal Creation**
   - Test deal proposal structure
   - Validate term specification
   - Test duration settings
   - Test deal limits per player

2. **Deal Negotiation**
   - Test acceptance/rejection mechanics
   - Test counter-proposal handling
   - Validate notification system
   - Test multi-player deals

3. **Deal Enforcement**
   - Test automatic deal monitoring
   - Validate deal violation detection
   - Test consequences application
   - Test enforcement of complex terms

4. **Deal History**
   - Test deal history tracking
   - Validate reputation system updates
   - Test history influence on AI decisions
   - Test broken deal statistics

```rust
#[test]
fn test_deal_lifecycle() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players with politics components
    let player1 = app.world.spawn((
        Player,
        PlayerName("Player 1".to_string()),
        PoliticsComponent::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player,
        PlayerName("Player 2".to_string()),
        PoliticsComponent::default(),
    )).id();
    
    // Create a deal proposal
    let deal_terms = vec![
        DealTerm::NonAggression {
            target: player2,
            duration: 2,
        },
    ];
    
    app.world.send_event(DealProposalEvent {
        proposer: player1,
        targets: vec![player2],
        terms: deal_terms,
        expiration: Some(DealExpiration::Turns(2)),
        timestamp: 0.0,
    });
    
    app.update();
    
    // Verify deal proposal created
    let politics1 = app.world.get::<PoliticsComponent>(player1).unwrap();
    assert_eq!(politics1.active_deals.len(), 1);
    assert_eq!(politics1.active_deals[0].status, DealStatus::Proposed);
    
    // Accept the deal
    let deal_id = politics1.active_deals[0].id;
    app.world.send_event(DealResponseEvent {
        deal_id,
        response_type: DealResponseType::Accept,
        player: player2,
        timestamp: 1.0,
    });
    
    app.update();
    
    // Verify deal is now active
    let politics1_updated = app.world.get::<PoliticsComponent>(player1).unwrap();
    assert_eq!(politics1_updated.active_deals[0].status, DealStatus::Active);
    
    // Test deal expiration
    advance_game_turns(&mut app, 2);
    app.update();
    
    // Verify deal has expired
    let politics1_final = app.world.get::<PoliticsComponent>(player1).unwrap();
    assert_eq!(politics1_final.active_deals[0].status, DealStatus::Expired);
}
```

## Testing Alliance Mechanics

The alliance system requires specific test cases:

1. **Alliance Formation**
   - Test alliance creation
   - Test alliance strength levels
   - Validate multi-player alliance formation

2. **Alliance Effects**
   - Test combat restrictions based on alliances
   - Test targeting restrictions for allied players
   - Validate benefits between allied players

3. **Alliance Dissolution**
   - Test voluntary alliance breaking
   - Test automatic alliance expiration
   - Validate alliance history tracking

```rust
#[test]
fn test_alliance_combat_restrictions() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn((
        Player,
        PoliticsComponent::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player,
        PoliticsComponent::default(),
    )).id();
    
    // Create strong alliance
    let mut politics1 = app.world.get_mut::<PoliticsComponent>(player1).unwrap();
    politics1.alliances.insert(player2, AllianceStrength::Strong);
    
    // Create creature
    let creature = app.world.spawn((
        CardType::Creature,
        Controller(player1),
        OnBattlefield,
    )).id();
    
    // Attempt to attack allied player
    app.world.send_event(DeclareAttackerEvent {
        attacker: creature,
        defender: player2,
        controller: player1,
    });
    
    app.update();
    
    // Check for attack modification event
    let attack_mods = app.world.resource::<Events<AttackModifierEvent>>()
        .get_reader()
        .read(&app.world.resource::<Events<AttackModifierEvent>>())
        .collect::<Vec<_>>();
    
    assert!(!attack_mods.is_empty());
    assert_eq!(attack_mods[0].attacker, creature);
    assert_eq!(attack_mods[0].defender, player2);
    
    // Verify correct modification type (should be DissuadeAttack for strong alliance)
    match &attack_mods[0].modification {
        AttackModification::DissuadeAttack { reason, penalty } => {
            assert!(reason.contains("alliance"));
            assert!(*penalty > 0.0);
        },
        _ => panic!("Expected DissuadeAttack modification"),
    }
}
```

## Testing Goad Mechanics

Goad mechanics require specific testing:

1. **Goad Application**
   - Test applying goad to creatures
   - Test goad duration tracking
   - Test multiple simultaneous goad effects

2. **Goad Attack Requirements**
   - Test forced attack requirement
   - Test restriction on attacking goader
   - Validate legal target determination

3. **Goad Edge Cases**
   - Test goad with no legal attack targets
   - Test goad when creature cannot attack
   - Test interaction with other attack restrictions

```rust
#[test]
fn test_goad_mechanics() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create 3 players
    let player1 = app.world.spawn(Player).id();
    let player2 = app.world.spawn(Player).id();
    let player3 = app.world.spawn(Player).id();
    
    // Create a creature
    let creature = app.world.spawn((
        CardName("Test Creature".to_string()),
        CardType::Creature,
        Power(3),
        Toughness(3),
        Controller(player1),
        OnBattlefield,
        CanAttack(true),
    )).id();
    
    // Goad the creature
    app.world.entity_mut(creature).insert(Goaded {
        goaded_by: player2,
        expires_at: ExpiryTiming::EndOfTurn,
    });
    
    // Set up combat phase
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareAttackers);
    app.world.resource_mut::<TurnManager>().active_player = player1;
    
    app.update();
    
    // Get attack requirements
    let attack_requirements = app.world.resource::<Events<AttackRequirementEvent>>()
        .get_reader()
        .read(&app.world.resource::<Events<AttackRequirementEvent>>())
        .collect::<Vec<_>>();
    
    // Verify goad requirements are enforced
    assert!(!attack_requirements.is_empty());
    assert_eq!(attack_requirements[0].creature, creature);
    assert!(attack_requirements[0].must_attack);
    assert_eq!(attack_requirements[0].cannot_attack, vec![player2]);
    
    // Advance turn to test goad expiration
    advance_turn(&mut app);
    
    // Verify goad has expired
    assert!(!app.world.entity(creature).contains::<Goaded>());
}
```

## Testing Threat Assessment

The threat assessment system requires verification:

1. **Threat Calculation**
   - Test basic threat score calculation
   - Test contribution of different threat factors
   - Validate dynamic threat updates

2. **Threat Visualization**
   - Test threat UI display
   - Test threat history tracking
   - Validate threat factor breakdown

3. **AI Integration**
   - Test AI use of threat assessment
   - Validate threat-based targeting
   - Test strategic adjustments based on threat

```rust
#[test]
fn test_threat_assessment() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn(Player).id();
    let player2 = app.world.spawn(Player).id();
    
    // Initialize threat assessment
    app.insert_resource(ThreatAssessment {
        player_threats: HashMap::new(),
        threat_factors: HashMap::new(),
        threat_history: HashMap::new(),
    });
    
    // Create board state for player1
    for _ in 0..5 {
        app.world.spawn((
            CardType::Creature,
            Controller(player1),
            OnBattlefield,
        ));
    }
    
    // Create commander for player1
    app.world.spawn((
        CardType::Creature,
        Commander,
        Controller(player1),
        OnBattlefield,
    ));
    
    // Create smaller board for player2
    for _ in 0..2 {
        app.world.spawn((
            CardType::Creature,
            Controller(player2),
            OnBattlefield,
        ));
    }
    
    // Run threat assessment system
    app.add_systems(Update, update_threat_assessment);
    app.update();
    
    // Check threat levels
    let threat = app.world.resource::<ThreatAssessment>();
    
    // Player1 should have higher threat than player2
    let player1_threat = threat.player_threats.get(&player1).unwrap();
    let player2_threat = threat.player_threats.get(&player2).unwrap();
    
    assert!(player1_threat.0 > player2_threat.0);
    
    // Verify threat factors were recorded
    assert!(!threat.threat_factors.get(&player1).unwrap().is_empty());
    
    // Check commander presence factor
    let commander_factor = threat.threat_factors.get(&player1).unwrap()
        .iter()
        .find(|f| matches!(f.factor_type, ThreatFactorType::CommanderPresence));
    
    assert!(commander_factor.is_some());
}
```

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
    simulate_alliance_formation(&mut app, &players);
    simulate_threat_based_targeting(&mut app, &players);
    
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
    
    // Property: Goad must always allow at least one legal attack target
    proptest!(|(players: Vec<Entity>, goaded_by: Entity)| {
        let legal_targets = determine_legal_attack_targets(players.clone(), goaded_by);
        // Verify at least one legal target if there are enough players
        if players.len() > 2 {
            assert!(!legal_targets.is_empty());
        }
    });
}
```

## Network Testing

Political systems must be tested for correct network behavior:

1. **State Synchronization**
   - Test monarch status synchronization
   - Validate vote transmission and collection
   - Test deal state synchronization
   - Test alliance state replication

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
   - Threat assessment with complex board states

2. **Memory Usage**
   - Test memory footprint of complex political states
   - Validate cleanup of expired political objects

## Test Fixtures

Reusable test fixtures include:

- Standard 4-player table setup
- Pre-defined political scenarios
- Mock AI political personalities
- Reference deal/vote templates
- Standard threat assessment profiles

## Continuous Integration

Politics tests are integrated into CI with:

- Automated test runs on PR submission
- Coverage reports for political mechanics
- Performance comparison against baseline
- Parallel testing of different political scenarios

## Related Resources

- [Multiplayer Politics](multiplayer_politics.md) - Core politics system documentation
- [Politics Tests](politics_tests/) - Specific test case implementations
- [Commander-Specific Cards](special_cards.md) - Cards with political mechanics 
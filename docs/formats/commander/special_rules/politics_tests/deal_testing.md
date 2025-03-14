# Deal System Testing

## Overview

This document details the testing approach for the Deal system within Rummage's multiplayer Commander implementation. Deals are a key political mechanic allowing players to make formal agreements with benefits and consequences, adding strategic depth to multiplayer interaction.

## Deal Creation Tests

Tests for the creation and initialization of deals:

```rust
#[test]
fn test_deal_creation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a simple deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[1],
                duration: DealDuration::Turns(2),
            },
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![
            DealReward::DrawCards { count: 1 },
        ],
        penalties: vec![
            DealPenalty::LifeLoss { amount: 5 },
        ],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Verify deal was created and is in pending state
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal = deal_registry.get_deal(deal_id).unwrap();
    
    assert_eq!(deal.proposer, players[0]);
    assert_eq!(deal.target, players[1]);
    assert_eq!(deal.terms.len(), 2);
    assert_eq!(deal.status, DealStatus::Pending);
}

#[test]
fn test_deal_term_validation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal with invalid terms (can't restrict a player not in the deal)
    let invalid_deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[2], // Invalid: players[2] not part of deal
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Verify deal was rejected
    let deal_registry = app.world.resource::<DealRegistry>();
    let invalid_deal = deal_registry.get_deal(invalid_deal_id).unwrap();
    
    assert_eq!(invalid_deal.status, DealStatus::Rejected);
    assert_eq!(
        invalid_deal.rejection_reason, 
        Some(DealRejectionReason::InvalidTerms)
    );
    
    // Create a valid deal
    let valid_deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[1], // Valid: players[1] is target
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Verify valid deal is pending
    let valid_deal = deal_registry.get_deal(valid_deal_id).unwrap();
    assert_eq!(valid_deal.status, DealStatus::Pending);
}

#[test]
fn test_deal_duration_validation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal with excessively long duration
    let long_deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(20), // Too long
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(20), // Exceeds maximum allowed duration
    });
    app.update();
    
    // Verify deal was rejected
    let deal_registry = app.world.resource::<DealRegistry>();
    let long_deal = deal_registry.get_deal(long_deal_id).unwrap();
    
    assert_eq!(long_deal.status, DealStatus::Rejected);
    assert_eq!(
        long_deal.rejection_reason, 
        Some(DealRejectionReason::ExcessiveDuration)
    );
}
```

## Deal Negotiation Tests

Testing the deal negotiation mechanics:

```rust
#[test]
fn test_deal_acceptance() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![
            DealReward::DrawCards { count: 1 },
        ],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Target accepts the deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Verify deal is now active
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal = deal_registry.get_deal(deal_id).unwrap();
    
    assert_eq!(deal.status, DealStatus::Active);
    
    // Verify reward was given
    let player1_hand_size = get_player_hand_size(&app, players[1]);
    assert_eq!(player1_hand_size, 8); // Assuming starting hand size of 7 + 1 from reward
}

#[test]
fn test_deal_rejection() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Target rejects the deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Reject,
    });
    app.update();
    
    // Verify deal is now rejected
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal = deal_registry.get_deal(deal_id).unwrap();
    
    assert_eq!(deal.status, DealStatus::Rejected);
    assert_eq!(deal.rejection_reason, Some(DealRejectionReason::TargetRejected));
}

#[test]
fn test_deal_counter_proposal() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[1],
                duration: DealDuration::Turns(3),
            },
        ],
        rewards: vec![
            DealReward::DrawCards { count: 1 },
        ],
        penalties: vec![],
        duration: DealDuration::Turns(3),
    });
    app.update();
    
    // Target makes a counter-proposal
    let counter_proposal = DealCounterProposal {
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[1],
                duration: DealDuration::Turns(2), // Shorter duration
            },
        ],
        rewards: vec![
            DealReward::DrawCards { count: 2 }, // More cards
        ],
        penalties: vec![],
        duration: DealDuration::Turns(2), // Shorter duration
    };
    
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::CounterProposal(counter_proposal),
    });
    app.update();
    
    // Verify original deal is now countered
    let deal_registry = app.world.resource::<DealRegistry>();
    let original_deal = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(original_deal.status, DealStatus::Countered);
    
    // Find the counter proposal
    let counter_deals: Vec<_> = deal_registry.deals_iter()
        .filter(|d| d.status == DealStatus::Pending && d.proposer == players[1] && d.target == players[0])
        .collect();
    
    assert_eq!(counter_deals.len(), 1);
    let counter_deal = &counter_deals[0];
    
    // Verify counter deal has the modified terms
    assert_eq!(counter_deal.rewards.len(), 1);
    if let DealReward::DrawCards { count } = counter_deal.rewards[0] {
        assert_eq!(count, 2);
    } else {
        panic!("Expected DrawCards reward");
    }
    
    // Proposer accepts counter-proposal
    app.world.send_event(RespondToDealEvent {
        responder: players[0],
        deal_id: counter_deal.id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Verify counter deal is now active
    let updated_counter_deal = deal_registry.get_deal(counter_deal.id).unwrap();
    assert_eq!(updated_counter_deal.status, DealStatus::Active);
}
```

## Deal Enforcement Tests

Testing the enforcement and violation of deals:

```rust
#[test]
fn test_deal_enforcement_attack_restriction() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(CombatPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create player creatures
    let attacker = app.world.spawn((
        Creature::default(),
        Permanent { controller: players[1], ..Default::default() },
    )).id();
    
    // Create a deal with attack restriction
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::AttackRestriction {
                restricted_player: players[0], // Target can't attack proposer
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![
            DealPenalty::LifeLoss { amount: 3 },
        ],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Target accepts the deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Capture life total before violation
    let initial_life = app.world.get::<Player>(players[1]).unwrap().life_total;
    
    // Target attempts to attack proposer (violating deal)
    app.world.send_event(DeclareAttackerEvent {
        attacker,
        defender: players[0],
    });
    app.update();
    
    // Verify deal violation was detected
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal = deal_registry.get_deal(deal_id).unwrap();
    
    assert!(deal.violations.contains(&DealViolation {
        violator: players[1],
        violation_type: ViolationType::AttackRestriction,
        turn: app.world.resource::<TurnState>().turn_number,
    }));
    
    // Verify penalty was applied
    let new_life = app.world.get::<Player>(players[1]).unwrap().life_total;
    assert_eq!(new_life, initial_life - 3);
}

#[test]
fn test_deal_expiration() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(TurnStructurePlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal with short duration
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(1),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(1), // 1 turn duration
    });
    app.update();
    
    // Target accepts the deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Verify deal is active
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_before = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_before.status, DealStatus::Active);
    
    // Advance turn
    advance_turn(&mut app);
    app.update();
    
    // Verify deal is now expired
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_after = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_after.status, DealStatus::Expired);
}

#[test]
fn test_deal_auto_termination_on_player_loss() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create and activate a deal
    let deal_id = create_and_accept_deal(&mut app, players[0], players[1]);
    
    // Verify deal is active
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_before = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_before.status, DealStatus::Active);
    
    // Player leaves game
    app.world.send_event(PlayerEliminatedEvent { player: players[0] });
    app.update();
    
    // Verify deal is now terminated
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_after = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_after.status, DealStatus::Terminated);
    assert_eq!(
        deal_after.termination_reason, 
        Some(DealTerminationReason::PlayerEliminated)
    );
}
```

## Deal History and Reputation Tests

Testing deal history tracking and reputation systems:

```rust
#[test]
fn test_deal_history_tracking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create and activate multiple deals
    let deal1_id = create_and_accept_deal(&mut app, players[0], players[1]);
    let deal2_id = create_and_accept_deal(&mut app, players[1], players[2]);
    let deal3_id = create_and_accept_deal(&mut app, players[0], players[2]);
    
    // Player 2 violates deal with player 0
    simulate_deal_violation(&mut app, deal3_id, players[2]);
    
    // Get deal history
    let history = app.world.resource::<DealHistory>();
    
    // Check player 0's history
    let player0_history = history.get_player_history(players[0]);
    assert_eq!(player0_history.deals_proposed, 2);
    assert_eq!(player0_history.deals_honored, 2); // Both active or not violated yet
    
    // Check player 2's history
    let player2_history = history.get_player_history(players[2]);
    assert_eq!(player2_history.deals_proposed, 0);
    assert_eq!(player2_history.deals_accepted, 2);
    assert_eq!(player2_history.deals_violated, 1);
}

#[test]
fn test_reputation_system() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create and accept several deals
    let deal1_id = create_and_accept_deal(&mut app, players[0], players[1]);
    let deal2_id = create_and_accept_deal(&mut app, players[0], players[2]);
    let deal3_id = create_and_accept_deal(&mut app, players[0], players[3]);
    
    // Player 3 violates deal
    simulate_deal_violation(&mut app, deal3_id, players[3]);
    
    // Check reputations
    let reputation_system = app.world.resource::<ReputationSystem>();
    
    let player0_rep = reputation_system.get_reputation(players[0]);
    let player1_rep = reputation_system.get_reputation(players[1]);
    let player3_rep = reputation_system.get_reputation(players[3]);
    
    // Player 0 should have good reputation (keeps deals)
    assert!(player0_rep.score > 0.0);
    
    // Player 1 should have neutral/positive reputation (honors deals)
    assert!(player1_rep.score >= 0.0);
    
    // Player 3 should have negative reputation (violated deal)
    assert!(player3_rep.score < 0.0);
    
    // Test reputation effects on deal proposals
    // Player 3 (bad reputation) tries to make a deal
    let deal4_id = app.world.send_event(CreateDealEvent {
        proposer: players[3],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Target is more likely to reject due to proposer's reputation
    let ai_decision = app.world.resource::<AiDealSystem>()
        .evaluate_deal_proposal(players[1], deal4_id);
    
    // AI should factor in reputation (exact values depend on implementation)
    assert!(ai_decision.trust_factor < 0.5);
}
```

## Integration Tests

Testing deal system integration with other game systems:

```rust
#[test]
fn test_deal_integration_with_combat() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(CombatPlugin);
       
    // Setup players and creatures
    let players = setup_four_player_game(&mut app);
    
    let creature1 = spawn_test_creature(&mut app, 2, 2, players[0]);
    let creature2 = spawn_test_creature(&mut app, 3, 3, players[1]);
    
    // Create non-aggression pact
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Accept deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Try to declare attack between deal participants
    app.world.send_event(DeclareAttackerEvent {
        attacker: creature1,
        defender: players[1],
    });
    app.update();
    
    // Verify combat restriction was enforced
    let combat_state = app.world.resource::<CombatState>();
    assert!(!combat_state.is_attacking(creature1));
    
    // But can still attack other players
    app.world.send_event(DeclareAttackerEvent {
        attacker: creature1,
        defender: players[2], // Not part of deal
    });
    app.update();
    
    // Verify attack was allowed
    let combat_state = app.world.resource::<CombatState>();
    assert!(combat_state.is_attacking(creature1));
    assert_eq!(combat_state.get_defender(creature1), Some(players[2]));
}

#[test]
fn test_deal_integration_with_card_effects() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create "deal breaker" card
    let deal_breaker = app.world.spawn((
        Card::default(),
        DealBreakerEffect,
    )).id();
    
    // Create and accept a deal
    let deal_id = create_and_accept_deal(&mut app, players[0], players[1]);
    
    // Verify deal is active
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_before = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_before.status, DealStatus::Active);
    
    // Cast deal breaker card (e.g., "Council's Judgment")
    app.world.send_event(CastCardEvent {
        caster: players[2],
        card: deal_breaker,
        targets: vec![EntityTarget::Deal(deal_id)],
    });
    app.update();
    
    // Resolve card effect
    app.world.send_event(ResolveCardEffectEvent {
        card: deal_breaker,
    });
    app.update();
    
    // Verify deal was terminated
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal_after = deal_registry.get_deal(deal_id).unwrap();
    assert_eq!(deal_after.status, DealStatus::Terminated);
    assert_eq!(
        deal_after.termination_reason, 
        Some(DealTerminationReason::CardEffect)
    );
}
```

## UI and Notification Tests

Testing UI and notification aspects of the deal system:

```rust
#[test]
fn test_deal_ui_representation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(UiPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create a deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer: players[0],
        target: players[1],
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Verify deal UI elements were created
    let deal_ui_elements = app.world.query_filtered::<Entity, With<DealUiElement>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!deal_ui_elements.is_empty());
    
    // Check for proposal notification
    let notifications = app.world.query_filtered::<&Notification, With<DealProposalNotification>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!notifications.is_empty());
    assert!(notifications[0].message.contains("proposed a deal"));
    
    // Accept deal
    app.world.send_event(RespondToDealEvent {
        responder: players[1],
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    // Check for acceptance notification
    let acceptance_notifications = app.world.query_filtered::<&Notification, With<DealAcceptanceNotification>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!acceptance_notifications.is_empty());
    assert!(acceptance_notifications[0].message.contains("accepted"));
    
    // Verify active deal indicator visible
    let active_deal_indicators = app.world.query_filtered::<Entity, (With<ActiveDealIndicator>, With<Parent>)>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!active_deal_indicators.is_empty());
}

#[test]
fn test_deal_violation_notifications() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(UiPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create and accept a deal
    let deal_id = create_and_accept_deal(&mut app, players[0], players[1]);
    
    // Simulate deal violation
    simulate_deal_violation(&mut app, deal_id, players[1]);
    
    // Check for violation notification
    let violation_notifications = app.world.query_filtered::<&Notification, With<DealViolationNotification>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!violation_notifications.is_empty());
    assert!(violation_notifications[0].message.contains("violated"));
}
```

## Helper Functions

```rust
/// Creates a standard deal and accepts it
fn create_and_accept_deal(app: &mut App, proposer: Entity, target: Entity) -> DealId {
    // Create deal
    let deal_id = app.world.send_event(CreateDealEvent {
        proposer,
        target,
        terms: vec![
            DealTerm::NonAggressionPact {
                duration: DealDuration::Turns(2),
            },
        ],
        rewards: vec![],
        penalties: vec![],
        duration: DealDuration::Turns(2),
    });
    app.update();
    
    // Accept deal
    app.world.send_event(RespondToDealEvent {
        responder: target,
        deal_id,
        response: DealResponse::Accept,
    });
    app.update();
    
    deal_id
}

/// Simulates violating a deal
fn simulate_deal_violation(app: &mut App, deal_id: DealId, violator: Entity) {
    // Get deal
    let deal_registry = app.world.resource::<DealRegistry>();
    let deal = deal_registry.get_deal(deal_id).unwrap();
    
    // Determine violation type based on terms
    let violation_type = if let Some(DealTerm::NonAggressionPact { .. }) = deal.terms.first() {
        ViolationType::NonAggressionViolation
    } else if let Some(DealTerm::AttackRestriction { .. }) = deal.terms.first() {
        ViolationType::AttackRestriction
    } else {
        ViolationType::Generic
    };
    
    // Trigger violation
    app.world.send_event(DealViolationEvent {
        deal_id,
        violator,
        violation_type,
    });
    app.update();
}

/// Creates a test creature
fn spawn_test_creature(app: &mut App, power: i32, toughness: i32, controller: Entity) -> Entity {
    app.world.spawn((
        Creature {
            power,
            toughness,
            ..Default::default()
        },
        Permanent {
            controller,
            ..Default::default()
        },
    )).id()
}
```

## Conclusion

The Deal System adds significant depth to political interactions in Commander. These tests ensure that deals are properly created, negotiated, enforced, and terminated under all relevant circumstances, while accurately tracking player reputation based on deal history. 
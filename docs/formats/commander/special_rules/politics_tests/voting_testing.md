# Voting System Testing

## Overview

This document outlines the comprehensive testing approach for the voting system in Rummage's Commander format implementation. The voting mechanic appears on various cards like Council's Judgment, Expropriate, and Capital Punishment, and represents a key political interaction point in multiplayer games.

## Vote Initialization Tests

Tests for the initialization of voting events:

```rust
#[test]
fn test_vote_initialization() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create vote options
    let option_a = "Exile target permanent";
    let option_b = "Each player sacrifices a creature";
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec![option_a.to_string(), option_b.to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Verify vote was created
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.options.len(), 2);
    assert_eq!(vote_state.initiator, players[0]);
    assert_eq!(vote_state.status, VoteStatus::Active);
    assert_eq!(vote_state.eligible_voters.len(), 4);
}

#[test]
fn test_vote_with_timing_restrictions() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(TurnStructurePlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Set current player and phase
    app.world.resource_mut::<TurnState>().current_player = players[0];
    app.world.resource_mut::<TurnState>().current_phase = Phase::Main1;
    
    // Initialize vote with sorcery timing
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::SorcerySpeed,
    });
    app.update();
    
    // Verify vote is allowed during main phase of initiator's turn
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.status, VoteStatus::Active);
    
    // Change to different player's turn
    app.world.resource_mut::<TurnState>().current_player = players[1];
    
    // Try to initialize another vote with sorcery timing
    let vote_id2 = app.world.send_event(InitializeVoteEvent {
        initiator: players[1],
        options: vec!["Option X".to_string(), "Option Y".to_string()],
        timing_restriction: VoteTiming::SorcerySpeed,
    });
    app.update();
    
    // Verify second vote is active (as it's now player1's turn)
    let vote_state2 = app.world.resource::<VoteRegistry>().get_vote(vote_id2).unwrap();
    assert_eq!(vote_state2.status, VoteStatus::Active);
    
    // Change to non-main phase
    app.world.resource_mut::<TurnState>().current_phase = Phase::Combat;
    
    // Try to initialize another vote with sorcery timing
    let vote_id3 = app.world.send_event(InitializeVoteEvent {
        initiator: players[1],
        options: vec!["Option C".to_string(), "Option D".to_string()],
        timing_restriction: VoteTiming::SorcerySpeed,
    });
    app.update();
    
    // Verify third vote is rejected due to timing
    let vote_state3 = app.world.resource::<VoteRegistry>().get_vote(vote_id3).unwrap();
    assert_eq!(vote_state3.status, VoteStatus::Rejected);
    assert_eq!(vote_state3.rejection_reason, Some(VoteRejectionReason::InvalidTiming));
}
```

## Vote Casting Tests

Tests for the vote casting mechanics:

```rust
#[test]
fn test_basic_vote_casting() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Players cast votes
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0, // Option A
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 0, // Option A
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 1, // Option B
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[3],
        vote_id,
        option_index: 1, // Option B
    });
    
    app.update();
    
    // Verify votes were recorded
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.votes.len(), 4);
    assert_eq!(vote_state.vote_counts[0], 2); // Option A has 2 votes
    assert_eq!(vote_state.vote_counts[1], 2); // Option B has 2 votes
}

#[test]
fn test_vote_weight_modifiers() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Add a vote weight modifier to player0
    app.world.entity_mut(players[0]).insert(VoteWeightModifier {
        multiplier: 2.0,
        expiration: VoteWeightExpiration::Permanent,
    });
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Players cast votes
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0, // Option A - should count as 2 votes
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 1, // Option B
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 1, // Option B
    });
    
    app.update();
    
    // Verify vote weights were applied correctly
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.vote_counts[0], 2); // Option A has 2 votes (from weighted player)
    assert_eq!(vote_state.vote_counts[1], 2); // Option B has 2 votes (1 each from two players)
}

#[test]
fn test_vote_time_limits() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(TimePlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote with time limit
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
        time_limit: Some(std::time::Duration::from_secs(30)),
    });
    app.update();
    
    // Players cast some votes
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 1,
    });
    
    app.update();
    
    // Fast forward time past the limit
    advance_time(&mut app, std::time::Duration::from_secs(31));
    app.update();
    
    // Verify vote was automatically closed
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.status, VoteStatus::Closed);
    
    // Try to cast another vote
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 0,
    });
    app.update();
    
    // Verify late vote was not counted
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.votes.len(), 2); // Still only 2 votes
}
```

## Vote Resolution Tests

Tests for resolving votes and applying their effects:

```rust
#[test]
fn test_vote_resolution_with_clear_winner() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Players cast votes with clear winner
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[3],
        vote_id,
        option_index: 1,
    });
    
    app.update();
    
    // Resolve vote
    app.world.send_event(ResolveVoteEvent { vote_id });
    app.update();
    
    // Verify correct option won
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.status, VoteStatus::Resolved);
    assert_eq!(vote_state.winning_option, Some(0)); // Option A won
}

#[test]
fn test_vote_tie_breaking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote with tie-breaker
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
        tie_breaker: VoteTieBreaker::Initiator,
    });
    app.update();
    
    // Players cast votes resulting in tie
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0, // Initiator votes for Option A
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 1,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[3],
        vote_id,
        option_index: 1,
    });
    
    app.update();
    
    // Resolve vote
    app.world.send_event(ResolveVoteEvent { vote_id });
    app.update();
    
    // Verify initiator's choice won the tie
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.status, VoteStatus::Resolved);
    assert_eq!(vote_state.winning_option, Some(0)); // Option A won due to initiator tie-break
    
    // Test different tie-breaker: Random
    let vote_id2 = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option X".to_string(), "Option Y".to_string()],
        timing_restriction: VoteTiming::OnResolve,
        tie_breaker: VoteTieBreaker::Random,
    });
    app.update();
    
    // Players cast votes resulting in tie
    for player in &players {
        app.world.send_event(CastVoteEvent {
            voter: *player,
            vote_id: vote_id2,
            option_index: player.index() % 2, // Evenly split votes
        });
    }
    app.update();
    
    // Set up deterministic RNG for testing
    app.insert_resource(TestRng::with_seed(12345));
    
    // Resolve vote
    app.world.send_event(ResolveVoteEvent { vote_id: vote_id2 });
    app.update();
    
    // Verify a random winner was selected
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id2).unwrap();
    assert_eq!(vote_state.status, VoteStatus::Resolved);
    assert!(vote_state.winning_option.is_some()); // Some option was chosen randomly
}

#[test]
fn test_vote_effect_application() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create test permanent to potentially exile
    let target_permanent = app.world.spawn((
        Permanent { controller: players[1], ..Default::default() },
    )).id();
    
    // Create vote with exile effect
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Exile permanent".to_string(), "Draw cards".to_string()],
        timing_restriction: VoteTiming::OnResolve,
        effects: vec![
            VoteEffect::ExilePermanent { target: target_permanent },
            VoteEffect::DrawCards { player: players[0], count: 2 },
        ],
    });
    app.update();
    
    // Players vote to exile the permanent
    for player in &players {
        app.world.send_event(CastVoteEvent {
            voter: *player,
            vote_id,
            option_index: 0, // Everyone votes to exile
        });
    }
    app.update();
    
    // Resolve vote
    app.world.send_event(ResolveVoteEvent { vote_id });
    app.update();
    
    // Verify permanent was exiled
    let zone = app.world.get::<Zone>(target_permanent).unwrap();
    assert_eq!(*zone, Zone::Exile);
}
```

## Edge Case Tests

Testing unusual voting scenarios:

```rust
#[test]
fn test_vote_abstention() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
        allow_abstention: true,
    });
    app.update();
    
    // Two players vote, two abstain
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 1,
    });
    
    // Players 2 and 3 abstain by not voting
    
    // Close vote manually
    app.world.send_event(ResolveVoteEvent { vote_id });
    app.update();
    
    // Verify vote was counted correctly with abstentions
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.votes.len(), 2); // Only 2 votes cast
    assert_eq!(vote_state.eligible_voters.len(), 4); // But 4 eligible voters
    assert_eq!(vote_state.abstention_count, 2); // 2 abstentions
}

#[test]
fn test_vote_with_modifiers() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Create card that affects votes
    let vote_modifier_card = app.world.spawn((
        Card::default(),
        VoteModifier {
            effect_type: VoteModifierType::DoubleVotesForOption { option_index: 0 },
            controller: players[0],
        },
    )).id();
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Players cast votes
    app.world.send_event(CastVoteEvent {
        voter: players[0],
        vote_id,
        option_index: 0, // This vote should be doubled
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[1],
        vote_id,
        option_index: 0,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[2],
        vote_id,
        option_index: 1,
    });
    
    app.world.send_event(CastVoteEvent {
        voter: players[3],
        vote_id,
        option_index: 1,
    });
    
    app.update();
    
    // Resolve vote
    app.world.send_event(ResolveVoteEvent { vote_id });
    app.update();
    
    // Verify vote modifier was applied
    let vote_state = app.world.resource::<VoteRegistry>().get_vote(vote_id).unwrap();
    assert_eq!(vote_state.modified_vote_counts[0], 3); // 2 votes, but player0's vote doubled
    assert_eq!(vote_state.modified_vote_counts[1], 2); // 2 normal votes
    assert_eq!(vote_state.winning_option, Some(0));
}
```

## UI and Network Tests

Testing the voting UI and network synchronization:

```rust
#[test]
fn test_vote_ui_representation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(UiPlugin);
       
    // Setup players
    let players = setup_four_player_game(&mut app);
    
    // Initialize vote
    let vote_id = app.world.send_event(InitializeVoteEvent {
        initiator: players[0],
        options: vec!["Option A".to_string(), "Option B".to_string()],
        timing_restriction: VoteTiming::OnResolve,
    });
    app.update();
    
    // Verify vote UI elements were created
    let vote_ui_elements = app.world.query_filtered::<Entity, With<VoteUiElement>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert!(!vote_ui_elements.is_empty());
    
    // Verify options are displayed
    let vote_options = app.world.query_filtered::<&Text, With<VoteOptionText>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    
    assert_eq!(vote_options.len(), 2);
    assert!(vote_options[0].sections[0].value.contains("Option A"));
    assert!(vote_options[1].sections[0].value.contains("Option B"));
}

#[test]
fn test_vote_network_synchronization() {
    let mut app = TestNetworkApp::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(NetworkPlugin);
       
    // Setup networked game with host and client
    let (host_id, client_id) = app.setup_network_game(2);
    
    // Host initializes vote
    app.send_host_message(InitializeVoteMessage {
        initiator: host_id,
        options: vec!["Option A".to_string(), "Option B".to_string()],
        vote_id: VoteId::new(),
    });
    app.update();
    
    // Verify vote was synchronized to client
    let client_votes = app.get_client_resource::<VoteRegistry>().active_votes();
    assert_eq!(client_votes.len(), 1);
    
    // Client casts vote
    app.send_client_message(CastVoteMessage {
        voter: client_id,
        vote_id: client_votes[0].id,
        option_index: 1,
    });
    app.update();
    
    // Verify vote was recorded on host
    let host_vote = app.get_host_resource::<VoteRegistry>().get_vote(client_votes[0].id).unwrap();
    assert_eq!(host_vote.votes.len(), 1);
    assert_eq!(host_vote.votes[0].voter, client_id);
    assert_eq!(host_vote.votes[0].option_index, 1);
}
```

## Conclusion

The voting system is a complex but essential political mechanic in Commander format. These tests ensure that votes are properly initialized, cast, tallied, and resolved under all circumstances, ensuring a fair and predictable voting experience for players. 
use bevy::prelude::*;
use rummage::game_engine::GameState;
use rummage::game_engine::combat::CombatState;
use rummage::game_engine::politics::{
    ActionType, CombatRestriction, Deal, DealBrokenEvent, DealDuration, DealProposedEvent,
    DealResponseEvent, DealStatus, DealTerm, GoadEffect, MonarchChangedEvent, PoliticsSystem, Vote,
    VoteCastEvent, VoteChoice, VoteCompletedEvent, VoteStartedEvent, deal_system, goad_system,
    monarch_system, voting_system,
};
use rummage::game_engine::turns::TurnManager;
use rummage::player::Player;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

// Helper function to set up a test app with the necessary resources and systems
fn setup_test_app() -> App {
    let mut app = App::new();

    // Add events
    app.add_event::<MonarchChangedEvent>()
        .add_event::<VoteStartedEvent>()
        .add_event::<VoteCastEvent>()
        .add_event::<VoteCompletedEvent>()
        .add_event::<DealProposedEvent>()
        .add_event::<DealResponseEvent>()
        .add_event::<DealBrokenEvent>();

    // Add resources
    app.insert_resource(PoliticsSystem::default())
        .insert_resource(GameState::default())
        .insert_resource(CombatState::default())
        .insert_resource(TurnManager::default());

    // Add systems
    app.add_systems(
        Update,
        (monarch_system, voting_system, goad_system, deal_system),
    );

    app
}

#[test]
fn test_monarch_system() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Send monarchy change event
    app.world_mut()
        .resource_mut::<Events<MonarchChangedEvent>>()
        .send(MonarchChangedEvent {
            new_monarch: player1,
            previous_monarch: None,
            source: None,
        });

    // Update the app to run systems
    app.update();

    // Verify player1 is the monarch
    let politics = app.world().resource::<PoliticsSystem>();
    assert_eq!(politics.monarch, Some(player1));

    // Change monarch to player2
    app.world_mut()
        .resource_mut::<Events<MonarchChangedEvent>>()
        .send(MonarchChangedEvent {
            new_monarch: player2,
            previous_monarch: Some(player1),
            source: None,
        });

    // Update the app to run systems
    app.update();

    // Verify player2 is now the monarch
    let politics = app.world().resource::<PoliticsSystem>();
    assert_eq!(politics.monarch, Some(player2));
}

#[test]
fn test_voting_system() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();
    let player3 = app.world_mut().spawn(Player::default()).id();

    // Create a vote
    let vote_id = Uuid::new_v4();
    let vote = Vote {
        id: vote_id,
        title: "Choose a player to attack".to_string(),
        source: Entity::PLACEHOLDER,
        controller: player1,
        choices: vec![
            VoteChoice {
                id: 1,
                text: "Player 1".to_string(),
                target: Some(player1),
            },
            VoteChoice {
                id: 2,
                text: "Player 2".to_string(),
                target: Some(player2),
            },
            VoteChoice {
                id: 3,
                text: "Player 3".to_string(),
                target: Some(player3),
            },
        ],
        eligible_voters: vec![player1, player2, player3],
        requires_all_players: false,
        timer: None,
        created_at: Instant::now(),
    };

    // Start a vote
    app.world_mut()
        .resource_mut::<Events<VoteStartedEvent>>()
        .send(VoteStartedEvent { vote: vote.clone() });

    // Update the app to run systems
    app.update();

    // Verify vote is active
    let politics = app.world().resource::<PoliticsSystem>();
    assert!(politics.active_vote.is_some());
    if let Some(active_vote) = &politics.active_vote {
        assert_eq!(active_vote.id, vote_id);
    }

    // Cast votes
    let choice1 = VoteChoice {
        id: 2,
        text: "Player 2".to_string(),
        target: Some(player2),
    };
    app.world_mut()
        .resource_mut::<Events<VoteCastEvent>>()
        .send(VoteCastEvent {
            vote_id,
            player: player1,
            choice: choice1,
        });

    let choice2 = VoteChoice {
        id: 2,
        text: "Player 2".to_string(),
        target: Some(player2),
    };
    app.world_mut()
        .resource_mut::<Events<VoteCastEvent>>()
        .send(VoteCastEvent {
            vote_id,
            player: player2,
            choice: choice2,
        });

    let choice3 = VoteChoice {
        id: 3,
        text: "Player 3".to_string(),
        target: Some(player3),
    };
    app.world_mut()
        .resource_mut::<Events<VoteCastEvent>>()
        .send(VoteCastEvent {
            vote_id,
            player: player3,
            choice: choice3,
        });

    // Update the app to run systems
    app.update();

    // Verify vote has completed
    let politics = app.world().resource::<PoliticsSystem>();
    assert!(politics.active_vote.is_none());

    // Check for vote completed event
    let has_vote_completed = {
        let vote_completed_events = app.world().resource::<Events<VoteCompletedEvent>>();
        let mut reader = vote_completed_events.get_reader();
        let mut completed = false;

        for event in reader.read(vote_completed_events) {
            if event.vote_id == vote_id {
                completed = true;
                // Verify the winning choice
                assert_eq!(event.winning_choice.id, 2);
                assert_eq!(event.vote_count, 2);
            }
        }
        completed
    };

    assert!(has_vote_completed, "Vote should have completed");
}

#[test]
fn test_goad_system() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Create a creature
    let creature = app.world_mut().spawn_empty().id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];
    turn_manager.turn_number = 1;

    // Add a goad effect
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        let goad_effect = GoadEffect {
            target: creature,
            source: player2, // Player2 goaded the creature
            duration: 2,     // Lasts for 2 turns
            created_at: 1,   // Created at turn 1
        };

        politics.goad_effects.insert(creature, vec![goad_effect]);
    }

    // Run goad system
    app.update();

    // Verify combat restrictions were applied
    let combat_state = app.world().resource::<CombatState>();

    // The creature should be forced to attack
    assert!(combat_state.must_attack.contains_key(&creature));

    // The creature can't attack player2 (who goaded it)
    assert!(combat_state.cannot_attack.contains_key(&creature));
    if let Some(restrictions) = combat_state.cannot_attack.get(&creature) {
        assert!(restrictions.contains(&player2));
    }

    // Advance turn number past goad duration
    {
        let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
        turn_manager.turn_number = 4; // Beyond the goad duration
    }

    // Run goad system again
    app.update();

    // Verify goad effect was removed
    let politics = app.world().resource::<PoliticsSystem>();
    assert!(!politics.goad_effects.contains_key(&creature));
}

#[test]
fn test_deal_system() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Create a deal
    let deal_id = Uuid::new_v4();
    let deal = Deal {
        id: deal_id,
        proposer: player1,
        target: player2,
        terms: vec![DealTerm::DoNotAttack(2)], // Don't attack for 2 turns
        duration: DealDuration::Turns(2),
        status: DealStatus::Proposed,
        created_at: Instant::now(),
    };

    // Propose a deal
    app.world_mut()
        .resource_mut::<Events<DealProposedEvent>>()
        .send(DealProposedEvent { deal: deal.clone() });

    // Update the app to run systems
    app.update();

    // Verify deal was added to pending deals
    let politics = app.world().resource::<PoliticsSystem>();
    assert_eq!(politics.pending_deals.len(), 1);
    assert_eq!(politics.pending_deals[0].id, deal_id);

    // Accept the deal
    app.world_mut()
        .resource_mut::<Events<DealResponseEvent>>()
        .send(DealResponseEvent {
            deal_id,
            accepted: true,
            responder: player2,
        });

    // Update the app to run systems
    app.update();

    // Verify deal moved from pending to active
    let politics = app.world().resource::<PoliticsSystem>();
    assert_eq!(politics.pending_deals.len(), 0);
    assert_eq!(politics.active_deals.len(), 1);
    assert_eq!(politics.active_deals[0].id, deal_id);
    assert!(matches!(
        politics.active_deals[0].status,
        DealStatus::Accepted
    ));

    // Break the deal
    app.world_mut()
        .resource_mut::<Events<DealBrokenEvent>>()
        .send(DealBrokenEvent {
            deal_id,
            breaker: player1,
            reason: "Attacked the protected player".to_string(),
        });

    // Update the app to run systems
    app.update();

    // Verify deal was removed from active deals
    let politics = app.world().resource::<PoliticsSystem>();
    assert_eq!(politics.active_deals.len(), 0);
}

#[test]
fn test_vote_decisiveness() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();
    let player3 = app.world_mut().spawn(Player::default()).id();
    let player4 = app.world_mut().spawn(Player::default()).id();

    // Create choices
    let choice1 = VoteChoice {
        id: 1,
        text: "Choice 1".to_string(),
        target: None,
    };
    let choice2 = VoteChoice {
        id: 2,
        text: "Choice 2".to_string(),
        target: None,
    };

    // Create a vote
    let vote_id = Uuid::new_v4();
    let vote = Vote {
        id: vote_id,
        title: "Test Vote".to_string(),
        source: Entity::PLACEHOLDER,
        controller: player1,
        choices: vec![choice1.clone(), choice2.clone()],
        eligible_voters: vec![player1, player2, player3, player4],
        requires_all_players: false,
        timer: None,
        created_at: Instant::now(),
    };

    // Setup politics system
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.active_vote = Some(vote);

        // Set vote weights (all equal for this test)
        politics.vote_weights.insert(player1, 1);
        politics.vote_weights.insert(player2, 1);
        politics.vote_weights.insert(player3, 1);
        politics.vote_weights.insert(player4, 1);
    }

    // Test scenario 1: Not decisive yet (2 vs 0, with 2 remaining voters)
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.votes_cast.insert(player1, choice1.clone());
        politics.votes_cast.insert(player2, choice1.clone());

        // Not decisive yet because remaining voters could still tie
        assert!(!politics.is_vote_decisive());
    }

    // Test scenario 2: Decisive (3 vs 0, with 1 remaining voter)
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.votes_cast.insert(player3, choice1.clone());

        // Now decisive because the remaining voter can't change the outcome
        assert!(politics.is_vote_decisive());
    }

    // Clear votes for next test
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.votes_cast.clear();
    }

    // Test scenario 3: Split vote, not decisive
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.votes_cast.insert(player1, choice1.clone());
        politics.votes_cast.insert(player2, choice2.clone());

        // Not decisive because remaining voters could swing either way
        assert!(!politics.is_vote_decisive());
    }

    // Test scenario 4: Weighted votes make it decisive
    {
        let mut politics = app.world_mut().resource_mut::<PoliticsSystem>();
        politics.votes_cast.clear();

        // Player 1 gets extra weight
        politics.vote_weights.insert(player1, 3);

        // Only player 1 has voted so far
        politics.votes_cast.insert(player1, choice1.clone());

        // Decisive because player1's vote (3) outweighs all other possible votes (3)
        assert!(politics.is_vote_decisive());
    }
}

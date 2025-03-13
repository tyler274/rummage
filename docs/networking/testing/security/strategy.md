# Security Testing for MTG Commander Online

This document outlines the security testing strategy for our MTG Commander online implementation, with a primary focus on protecting hidden information and preventing cheating in a distributed multiplayer environment.

## Table of Contents

1. [Security Objectives](#security-objectives)
2. [Threat Model](#threat-model)
3. [Hidden Information Protection](#hidden-information-protection)
4. [Client Validation and Server Authority](#client-validation-and-server-authority)
5. [Anti-Cheat Testing](#anti-cheat-testing)
6. [Penetration Testing Methodology](#penetration-testing-methodology)
7. [Fuzzing and Malformed Input Testing](#fuzzing-and-malformed-input-testing)
8. [Session and Authentication Testing](#session-and-authentication-testing)
9. [Continuous Security Testing](#continuous-security-testing)

## Security Objectives

The primary security objectives for our MTG Commander online implementation are:

1. **Information Confidentiality**: Ensuring players only have access to information they're entitled to see
2. **Game State Integrity**: Preventing unauthorized modification of game state
3. **Rules Enforcement**: Ensuring all actions follow MTG rules, even against malicious clients
4. **Input Validation**: Protecting against malicious inputs that could crash or exploit the game
5. **Fairness**: Preventing any player from gaining unfair advantages through technical means

## Threat Model

Our threat model considers the following potential adversaries and attack vectors:

### Adversaries

1. **Curious Players**: Regular players who might attempt to gain unfair advantages by viewing hidden information
2. **Cheaters**: Players actively attempting to manipulate the game to win unfairly
3. **Griefers**: Players attempting to disrupt gameplay for others without necessarily seeking to win
4. **Reverse Engineers**: Technical users analyzing the client to understand and potentially exploit the protocol

### Attack Vectors

1. **Client Modification**: Altering the client to expose hidden information or enable illegal actions
2. **Network Traffic Analysis**: Analyzing network traffic to reveal hidden information
3. **Protocol Exploitation**: Sending malformed or unauthorized messages to the server
4. **Memory Examination**: Using external tools to examine client memory for hidden information
5. **Timing Attacks**: Using timing differences to infer hidden information

## Hidden Information Protection

MTG has significant hidden information that must be protected:

- Cards in hand
- Library contents and order
- Facedown cards
- Opponent's choices for effects like scry

### Testing Hidden Card Information

```rust
#[test]
fn test_hand_card_information_protection() {
    let mut app = setup_security_test_app(4);
    
    // Setup player 1 with known cards in hand
    let player1_hand = vec![101, 102, 103];
    app.world.resource_mut::<TestController>()
        .setup_player_hand(1, player1_hand.clone());
    
    // Allow state to replicate
    app.update_n_times(10);
    
    // Verify each client's visibility
    for client_id in 1..=4 {
        let client_view = extract_client_game_state(&app, client_id);
        
        if client_id == 1 {
            // Player 1 should see all details of their cards
            for card_id in &player1_hand {
                let card = client_view.get_card(*card_id);
                assert!(card.is_some());
                assert_eq!(card.unwrap().visibility_level, VisibilityLevel::FullInformation);
            }
        } else {
            // Other players should only see card backs
            let opponent_info = client_view.get_opponent_info(1);
            assert_eq!(opponent_info.hand_size, player1_hand.len());
            
            // Should not have full information about the cards
            for card_id in &player1_hand {
                let card = client_view.get_card(*card_id);
                assert!(
                    card.is_none() || card.unwrap().visibility_level == VisibilityLevel::CardBack,
                    "Client {} can see card {} when they shouldn't", 
                    client_id, 
                    card_id
                );
            }
        }
    }
}
```

### Testing Revealed Card Information

```rust
#[test]
fn test_revealed_card_information() {
    let mut app = setup_security_test_app(4);
    
    // Setup player 1 with known cards in hand
    let player1_hand = vec![101, 102, 103];
    app.world.resource_mut::<TestController>()
        .setup_player_hand(1, player1_hand.clone());
    
    // Reveal a specific card to all players
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::RevealCard {
            player_id: 1,
            card_id: 102,
            reveal_to: RevealTarget::AllPlayers
        });
    
    app.update_n_times(10);
    
    // Verify correct card is revealed to all players
    for client_id in 1..=4 {
        let client_view = extract_client_game_state(&app, client_id);
        
        // All players should now see the revealed card
        let revealed_card = client_view.get_card(102);
        assert!(revealed_card.is_some());
        assert_eq!(
            revealed_card.unwrap().visibility_level, 
            VisibilityLevel::Revealed
        );
        
        // Other cards in hand should still be hidden
        if client_id != 1 {
            for &card_id in &[101, 103] {
                let card = client_view.get_card(card_id);
                assert!(
                    card.is_none() || card.unwrap().visibility_level == VisibilityLevel::CardBack,
                    "Client {} can see unrevealed card {} when they shouldn't", 
                    client_id, 
                    card_id
                );
            }
        }
    }
}
```

### Testing Library Information Protection

```rust
#[test]
fn test_library_information_protection() {
    let mut app = setup_security_test_app(4);
    
    // Setup player 1 with known library
    let library_cards = (201..240).collect::<Vec<usize>>();
    app.world.resource_mut::<TestController>()
        .setup_player_library(1, library_cards.clone());
    
    app.update_n_times(10);
    
    // Verify library information protection
    for client_id in 1..=4 {
        let client_view = extract_client_game_state(&app, client_id);
        
        // All players should only see library size, not contents
        let player1_info = if client_id == 1 {
            client_view.get_player_info()
        } else {
            client_view.get_opponent_info(1)
        };
        
        assert_eq!(player1_info.library_size, library_cards.len());
        
        // No player should see library card details (even the owner)
        for &card_id in &library_cards {
            let card = client_view.get_card(card_id);
            
            // Card might be visible as existing, but contents should be hidden
            if card.is_some() {
                assert!(
                    card.unwrap().visibility_level == VisibilityLevel::CardBack ||
                    card.unwrap().zone == Zone::Library,
                    "Client {} can see library card {} contents when they shouldn't",
                    client_id,
                    card_id
                );
            }
        }
    }
}
```

## Client Validation and Server Authority

The server must maintain authority over game state and validate all client actions.

### Testing Illegal Action Prevention

```rust
#[test]
fn test_illegal_action_prevention() {
    let mut app = setup_security_test_app(4);
    
    // Setup game state with player 1 having some cards but not others
    app.world.resource_mut::<TestController>()
        .setup_player_hand(1, vec![101, 102]);
    
    app.update_n_times(10);
    
    // Test case 1: Try to play a card not in hand
    let illegal_action = SecurityTestAction::PlayNonExistentCard {
        player_id: 1,
        card_id: 999 // Card that doesn't exist
    };
    
    let result = app.world.resource_mut::<SecurityTestController>()
        .execute_illegal_action(illegal_action);
    
    app.update_n_times(10);
    
    // Verify action was rejected
    assert!(matches!(result, ActionResult::Rejected(rejection) if 
        rejection.reason == RejectionReason::CardNotFound));
    
    // Test case 2: Try to play opponent's card
    app.world.resource_mut::<TestController>()
        .setup_player_hand(2, vec![201]);
    
    app.update_n_times(10);
    
    let illegal_action = SecurityTestAction::PlayOpponentCard {
        player_id: 1,
        card_id: 201, // Card in player 2's hand
        target_player: 2
    };
    
    let result = app.world.resource_mut::<SecurityTestController>()
        .execute_illegal_action(illegal_action);
    
    app.update_n_times(10);
    
    // Verify action was rejected
    assert!(matches!(result, ActionResult::Rejected(rejection) if 
        rejection.reason == RejectionReason::NotYourCard));
    
    // Verify game state remains unchanged
    let game_state = extract_server_game_state(&app);
    assert!(game_state.players[0].hand.contains(&101));
    assert!(game_state.players[0].hand.contains(&102));
    assert!(game_state.players[1].hand.contains(&201));
}
```

### Testing Play Sequence Enforcement

```rust
#[test]
fn test_out_of_sequence_play_prevention() {
    let mut app = setup_security_test_app(4);
    
    // Setup game state where it's player 1's turn, main phase
    app.world.resource_mut::<TestController>()
        .setup_game_phase(1, Phase::Main1);
    
    app.update_n_times(10);
    
    // Try to perform an action when it's not your turn
    let illegal_action = SecurityTestAction::ActOutOfTurn {
        player_id: 2, // Not the active player
        action_type: ActionType::PlayLand,
        card_id: 201
    };
    
    let result = app.world.resource_mut::<SecurityTestController>()
        .execute_illegal_action(illegal_action);
    
    app.update_n_times(10);
    
    // Verify action was rejected
    assert!(matches!(result, ActionResult::Rejected(rejection) if 
        rejection.reason == RejectionReason::NotYourTurn));
        
    // Try to declare attackers during main phase
    let illegal_action = SecurityTestAction::ActOutOfPhase {
        player_id: 1, // Correct player
        action_type: ActionType::DeclareAttackers,
    };
    
    let result = app.world.resource_mut::<SecurityTestController>()
        .execute_illegal_action(illegal_action);
    
    app.update_n_times(10);
    
    // Verify action was rejected
    assert!(matches!(result, ActionResult::Rejected(rejection) if 
        rejection.reason == RejectionReason::InvalidPhase));
}
```

## Anti-Cheat Testing

Tests focused specifically on detecting and preventing common cheating methods.

### Network Traffic Analysis

```rust
#[test]
fn test_network_traffic_does_not_leak_information() {
    let mut app = setup_security_test_app(4);
    
    // Setup player hands and library
    for player_id in 1..=4 {
        app.world.resource_mut::<TestController>()
            .setup_player_hand(player_id, vec![100 + player_id*10, 101 + player_id*10, 102 + player_id*10]);
            
        app.world.resource_mut::<TestController>()
            .setup_player_library(player_id, (200 + player_id*100..250 + player_id*100).collect());
    }
    
    // Start network traffic analyzer
    let mut traffic_analyzer = NetworkTrafficAnalyzer::new();
    app.insert_resource(traffic_analyzer.clone());
    app.add_systems(Update, analyze_network_traffic);
    
    // Run game actions that should generate network traffic
    let actions = [
        GameAction::DrawCard(1),
        GameAction::PlayLand(1, 110),
        GameAction::PassPriority(1),
        GameAction::PassTurn(1),
    ];
    
    for action in &actions {
        app.world.resource_mut::<TestController>()
            .execute_action(action.clone());
        app.update_n_times(5);
    }
    
    // Get traffic analysis results
    let traffic_results = app.world.resource::<NetworkTrafficAnalyzer>().get_results();
    
    // Verify no hidden information is leaked
    for &player_id in &[2, 3, 4] {
        // Check player hand cards aren't leaked to others
        for card_id in 100 + player_id*10..103 + player_id*10 {
            assert!(!traffic_results.contains_card_information(1, card_id),
                "Player 1's network traffic contains card {} from player {}'s hand",
                card_id, player_id);
        }
        
        // Check library card contents aren't leaked
        for card_id in 200 + player_id*100..250 + player_id*100 {
            assert!(!traffic_results.contains_card_information(1, card_id),
                "Player 1's network traffic contains card {} from player {}'s library",
                card_id, player_id);
        }
    }
}
```

### Memory Examination Protection

```rust
#[test]
fn test_client_memory_protection() {
    // This would be a manual test in practice, documented here for completeness
    
    // Steps:
    // 1. Set up a game with known hidden information
    // 2. Attach memory examination tools to client process
    // 3. Scan memory for card identifiers or other game data
    // 4. Verify sensitive information is obfuscated or encrypted in memory
    
    // Implementation would vary based on platform and tooling
}
```

## Penetration Testing Methodology

Structured approach to identifying security vulnerabilities:

```rust
fn perform_security_penetration_test() {
    // 1. Information Gathering
    let game_info = analyze_game_protocol();
    
    // 2. Threat Modeling
    let attack_vectors = identify_attack_vectors(game_info);
    
    // 3. Vulnerability Analysis
    let vulnerabilities = scan_for_vulnerabilities(attack_vectors);
    
    // 4. Exploitation
    for vulnerability in vulnerabilities {
        let exploit_result = attempt_exploit(vulnerability);
        
        if exploit_result.successful {
            record_security_issue(exploit_result);
        }
    }
    
    // 5. Post Exploitation
    analyze_exploit_impact();
    
    // 6. Reporting
    generate_security_report();
}
```

### Example Penetration Test Scenario

```rust
#[test]
fn test_client_message_tampering() {
    let mut app = setup_security_test_app(4);
    
    // Setup initial game state
    app.world.resource_mut::<TestController>()
        .setup_standard_game_start();
    
    // Create a message tampering simulator
    let mut tampering_simulator = MessageTamperingSimulator::new();
    app.insert_resource(tampering_simulator);
    
    // Register message tampering scenarios
    app.world.resource_mut::<MessageTamperingSimulator>()
        .register_tampering_scenario(TamperingScenario::ModifyCardId {
            original_id: 101,
            modified_id: 999
        })
        .register_tampering_scenario(TamperingScenario::InjectAction {
            action: GameAction::DrawCard(1),
            injection_point: InjectionPoint::AfterPassPriority
        })
        .register_tampering_scenario(TamperingScenario::ModifyTargetPlayer {
            original_player: 2,
            modified_player: 1
        });
    
    // Run game with message tampering active
    let result = run_game_with_tampering(&mut app);
    
    // Verify server detected and rejected tampering
    assert_eq!(result.detected_tampering_count, 3);
    assert_eq!(result.successful_tampering_count, 0);
    
    // Verify game state integrity maintained
    assert!(result.game_state_integrity_maintained);
}
```

## Fuzzing and Malformed Input Testing

Using automated fuzzing to identify input validation issues:

```rust
#[test]
fn test_protocol_fuzzing() {
    let mut app = setup_security_test_app(4);
    
    // Create fuzzer
    let fuzzer = ProtocolFuzzer::new()
        .with_seed(12345)
        .with_message_types(vec![
            MessageType::Action,
            MessageType::StateUpdate,
            MessageType::SyncRequest
        ])
        .with_mutation_rate(0.2);
    
    // Register fuzzer with app
    app.insert_resource(fuzzer);
    app.add_systems(Update, run_protocol_fuzzing);
    
    // Run for specified number of iterations
    for _ in 0..1000 {
        app.update();
    }
    
    // Check results
    let fuzzer = app.world.resource::<ProtocolFuzzer>();
    let results = fuzzer.get_results();
    
    // Verify no crashes occurred
    assert_eq!(results.server_crashes, 0);
    assert_eq!(results.client_crashes, 0);
    
    // Verify all malformed inputs were rejected
    assert_eq!(
        results.malformed_messages_sent,
        results.malformed_messages_rejected,
        "{} malformed messages were incorrectly accepted",
        results.malformed_messages_sent - results.malformed_messages_rejected
    );
}
```

## Session and Authentication Testing

Verify players can only control their own actions:

```rust
#[test]
fn test_session_integrity() {
    let mut app = setup_security_test_app(4);
    
    // Setup standard game
    app.world.resource_mut::<TestController>()
        .setup_standard_game_start();
    
    // Attempt to spoof another player's session
    let spoofing_result = app.world.resource_mut::<SecurityTestController>()
        .attempt_session_spoofing(SpoofingAttempt {
            actual_player: 1,
            spoofed_player: 2,
            action: GameAction::PlayLand(2, 201)
        });
    
    // Verify spoofing was detected and prevented
    assert!(!spoofing_result.successful);
    assert_eq!(spoofing_result.detection_reason, DetectionReason::SessionValidationFailed);
    
    // Verify game state remained unchanged
    let game_state = extract_server_game_state(&app);
    assert!(!game_state.turn_history.contains_action_by_player(2, ActionType::PlayLand));
}
```

## Continuous Security Testing

Integrated into the CI/CD pipeline:

```yaml
# .github/workflows/security-tests.yml
name: Security Tests

on:
  push:
    branches: [ main, develop ]
  schedule:
    - cron: '0 0 * * *'  # Run daily

jobs:
  security-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run information hiding tests
      run: cargo test --verbose networking::security::information_hiding
    
    - name: Run protocol validation tests
      run: cargo test --verbose networking::security::protocol_validation
    
    - name: Run anti-cheat tests
      run: cargo test --verbose networking::security::anti_cheat
    
    - name: Run fuzzing tests (longer runtime)
      run: cargo test --verbose networking::security::fuzzing -- --ignored
```

---

By implementing these comprehensive security tests, we can ensure our MTG Commander online implementation protects the integrity of the game, prevents cheating, and maintains the proper hidden information characteristics essential to Magic: The Gathering. These tests provide confidence that our networking implementation is not only functional but also secure against potential exploitation attempts. 
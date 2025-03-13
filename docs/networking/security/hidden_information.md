# Hidden Information Management in MTG Commander

This document outlines the approach to managing hidden information in the MTG Commander game engine's multiplayer implementation.

## Overview

Magic: The Gathering relies heavily on hidden information as a core gameplay mechanic. Cards in players' hands, the order of libraries, and face-down cards are all examples of information that must be hidden from some or all players. Properly securing this information is critical to maintaining game integrity.

## Types of Hidden Information

1. **Player Hands**: Cards in a player's hand should be visible only to that player
2. **Libraries**: The order and content of libraries should be hidden from all players
3. **Face-down Cards**: Cards played face-down should have their identity hidden
4. **Revealed Cards**: Cards revealed to specific players should only be visible to those players
5. **Search Results**: When a player searches their library, only they should see the results

## Implementation Approach

### Server-Side Information Management

The server maintains the authoritative game state and controls what information is sent to each client:

```rust
// System to update visible information for each client
fn update_client_visible_information(
    mut server: ResMut<RepliconServer>,
    game_state: Res<GameState>,
    players: Query<(Entity, &Player, &Hand, &Library)>,
    connected_clients: Res<ConnectedClients>,
) {
    // For each connected client
    for client_id in connected_clients.clients.keys() {
        // Find the player entity for this client
        let player_entity = players
            .iter()
            .find(|(_, player, _, _)| player.client_id == *client_id)
            .map(|(entity, _, _, _)| entity);
        
        // Prepare visible game state for this client
        let mut visible_state = VisibleGameState {
            // Common visible information
            battlefield: game_state.battlefield.clone(),
            graveyards: game_state.graveyards.clone(),
            exiled_cards: game_state.exiled_cards.clone(),
            
            // Player-specific information
            player_hands: HashMap::new(),
            library_counts: HashMap::new(),
            library_tops: HashMap::new(),
        };
        
        // Add information about each player's hand and library
        for (entity, player, hand, library) in players.iter() {
            // If this is the current player, show their full hand
            if Some(entity) == player_entity {
                visible_state.player_hands.insert(player.id, hand.cards.clone());
            } else {
                // For other players, only show card count
                visible_state.player_hands.insert(player.id, vec![CardBack; hand.cards.len()]);
            }
            
            // For all players, only show library count, not contents
            visible_state.library_counts.insert(player.id, library.cards.len());
            
            // Show top card if it's been revealed
            if library.top_revealed {
                visible_state.library_tops.insert(player.id, library.cards.first().cloned());
            }
        }
        
        // Send the visible state to this client
        server.send_message(*client_id, ClientGameState { state: visible_state });
    }
}
```

### Client-Side Information Handling

The client displays only the information it receives from the server:

```rust
// Client system to update UI based on visible information
fn update_game_ui(
    visible_state: Res<VisibleGameState>,
    mut ui_state: ResMut<UiState>,
    local_player_id: Res<LocalPlayerId>,
) {
    // Update battlefield display
    ui_state.battlefield_cards = visible_state.battlefield.clone();
    
    // Update hand display (only shows full information for local player)
    if let Some(hand) = visible_state.player_hands.get(&local_player_id.0) {
        ui_state.hand_cards = hand.clone();
    }
    
    // Update opponent hand displays (only shows card backs)
    for (player_id, hand) in visible_state.player_hands.iter() {
        if *player_id != local_player_id.0 {
            ui_state.opponent_hand_counts.insert(*player_id, hand.len());
        }
    }
    
    // Update library displays (only shows counts)
    for (player_id, count) in visible_state.library_counts.iter() {
        ui_state.library_counts.insert(*player_id, *count);
    }
    
    // Update revealed top cards if any
    for (player_id, card) in visible_state.library_tops.iter() {
        if let Some(card) = card {
            ui_state.revealed_tops.insert(*player_id, card.clone());
        }
    }
}
```

## Security Measures

### Encryption

All network traffic containing hidden information is encrypted:

```rust
// Example of encrypting sensitive game data
fn encrypt_game_message(
    message: &GameMessage,
    encryption_key: &EncryptionKey,
) -> EncryptedMessage {
    // Serialize the message
    let serialized = bincode::serialize(message).expect("Failed to serialize message");
    
    // Encrypt the serialized data
    let nonce = generate_nonce();
    let cipher = ChaCha20Poly1305::new(encryption_key.as_ref().into());
    let ciphertext = cipher
        .encrypt(&nonce, serialized.as_ref())
        .expect("Encryption failed");
    
    EncryptedMessage {
        ciphertext,
        nonce,
    }
}
```

### Server Authority

The server is the single source of truth for all game state:

- Clients never directly modify hidden information
- All actions that would reveal information are processed by the server
- The server controls exactly what information each client can see

### Validation

The server validates all client actions to ensure they don't attempt to access hidden information:

```rust
// Validate a player's attempt to look at cards
fn validate_look_at_cards_action(
    mut commands: Commands,
    mut server: ResMut<RepliconServer>,
    mut look_events: EventReader<LookAtCardsEvent>,
    game_state: Res<GameState>,
    players: Query<(Entity, &Player)>,
) {
    for event in look_events.read() {
        let client_id = event.client_id;
        let target_zone = event.zone;
        let target_player_id = event.player_id;
        
        // Check if this action is allowed by a game effect
        let is_allowed = game_state
            .active_effects
            .iter()
            .any(|effect| effect.allows_looking_at(target_zone, target_player_id, client_id));
        
        if !is_allowed {
            // Log potential information leak attempt
            warn!("Potential information leak attempt: Client {} tried to look at {:?} of player {} without permission", 
                client_id, target_zone, target_player_id);
            
            // Reject the action
            server.send_message(client_id, ActionRejected { 
                reason: "Not allowed to look at those cards".to_string() 
            });
            
            continue;
        }
        
        // Process the legitimate look action
        // ...
    }
}
```

## Testing Hidden Information Security

Testing the hidden information system involves:

1. **Penetration Testing**: Attempting to access hidden information through various attack vectors
2. **Protocol Analysis**: Examining network traffic to ensure hidden information isn't leaked
3. **Edge Case Testing**: Testing unusual game states that might reveal hidden information

For detailed testing procedures, see the [Security Testing Strategy](../testing/security/strategy.md).

## Future Enhancements

Planned improvements to hidden information management include:

- Enhanced encryption for particularly sensitive game actions
- Improved obfuscation of client-side game state
- Additional validation for complex card interactions involving hidden information
- Support for spectator mode with appropriate information hiding

---

This documentation will be updated as hidden information management evolves.

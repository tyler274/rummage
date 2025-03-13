# Game Synchronization in MTG Commander

This document outlines the approach to synchronizing game state across multiple clients in the MTG Commander game engine's multiplayer implementation.

## Overview

Synchronization is a critical aspect of multiplayer games, ensuring that all players have a consistent view of the game state. In a turn-based card game like Magic: The Gathering, synchronization must handle complex state changes, hidden information, and the potential for network disruptions.

## Synchronization Challenges

MTG Commander presents several unique synchronization challenges:

1. **Complex State**: The game state includes numerous interrelated components
2. **Hidden Information**: Some information must be hidden from certain players
3. **Turn-Based Structure**: Actions occur in a specific order with priority passing
4. **Triggered Abilities**: Game events can trigger abilities at various times
5. **Network Conditions**: Players may have varying network quality and latency

## Synchronization Approach

### Server-Authoritative Model

The game uses a server-authoritative model where the server maintains the definitive game state:

```rust
// Server plugin setup
fn setup_server_plugin(app: &mut App) {
    app.add_plugins(ServerPlugin {
        tick_policy: TickPolicy::EveryFrame,
        ..Default::default()
    })
    .add_systems(Update, (
        process_player_actions,
        validate_game_state,
        send_state_updates,
    ).chain().in_set(ServerSet::Authority));
}
```

### Command-Based Synchronization

Player actions are sent as commands rather than direct state changes:

```rust
// Client-side system to send player actions
fn send_player_action(
    mut client: ResMut<RepliconClient>,
    mut action_events: EventReader<PlayerActionEvent>,
    client_id: Res<ClientId>,
) {
    for action in action_events.read() {
        // Convert local action to network command
        let command = match action {
            PlayerActionEvent::PlayCard { card_id, targets } => {
                PlayerCommand::PlayCard {
                    client_id: *client_id,
                    card_id: *card_id,
                    targets: targets.clone(),
                }
            },
            // Other action types...
        };
        
        // Send command to server
        client.send_message(command);
    }
}

// Server-side system to process commands
fn process_player_commands(
    mut commands: Commands,
    mut server: ResMut<RepliconServer>,
    mut command_events: EventReader<PlayerCommand>,
    game_state: Res<GameState>,
    // Other resources and queries...
) {
    for command in command_events.read() {
        // Validate command
        if !validate_command(command, &game_state) {
            // Send rejection to client
            server.send_message(
                command.client_id(), 
                CommandRejected { 
                    reason: "Invalid command".to_string() 
                }
            );
            continue;
        }
        
        // Apply command to game state
        apply_command(command, &mut commands, &game_state);
        
        // Notify all clients about the action
        server.broadcast_message(
            CommandAccepted { 
                client_id: command.client_id(),
                command_type: command.command_type(),
            }
        );
    }
}
```

### Incremental State Updates

The system sends incremental updates rather than full state snapshots:

```rust
// System to send state updates to clients
fn send_state_updates(
    mut server: ResMut<RepliconServer>,
    game_state: Res<GameState>,
    changed_entities: Query<Entity, Changed<Card>>,
    connected_clients: Res<ConnectedClients>,
) {
    // Collect entities that have changed
    let changed = changed_entities.iter().collect::<Vec<_>>();
    
    if !changed.is_empty() {
        // For each client, send relevant updates
        for client_id in connected_clients.clients.keys() {
            let visible_changes = filter_visible_changes(&changed, *client_id, &game_state);
            
            if !visible_changes.is_empty() {
                server.send_message(
                    *client_id,
                    StateUpdate { 
                        entities: visible_changes,
                        turn: game_state.turn_number,
                        phase: game_state.current_phase,
                    }
                );
            }
        }
    }
}
```

### Tick-Based Synchronization

The game uses a tick-based system to ensure actions are processed in order:

```rust
// Server tick configuration
fn configure_server_tick(app: &mut App) {
    app.insert_resource(ServerTick::default())
        .add_systems(
            First, 
            increment_server_tick
                .run_if(server_running)
        );
}

// System to increment the server tick
fn increment_server_tick(mut tick: ResMut<ServerTick>) {
    tick.0 += 1;
}

// Process actions in tick order
fn process_actions_in_order(
    tick: Res<ServerTick>,
    mut action_queue: ResMut<ActionQueue>,
    // Other resources...
) {
    // Sort actions by tick
    action_queue.sort_by_key(|action| action.tick);
    
    // Process actions up to current tick
    while let Some(action) = action_queue.peek() {
        if action.tick > tick.0 {
            break;
        }
        
        let action = action_queue.pop().unwrap();
        // Process the action
        // ...
    }
}
```

## Handling Network Issues

### Disconnection Handling

The system gracefully handles client disconnections:

```rust
// System to handle client disconnections
fn handle_client_disconnect(
    mut server_events: EventReader<ServerEvent>,
    mut game_state: ResMut<GameState>,
    players: Query<(Entity, &Player)>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        if let ServerEvent::ClientDisconnected { client_id } = event {
            // Find the player entity for this client
            if let Some((player_entity, player)) = players
                .iter()
                .find(|(_, p)| p.client_id == *client_id)
            {
                // Mark player as disconnected
                commands.entity(player_entity).insert(Disconnected {
                    timestamp: Utc::now(),
                });
                
                // Update game state
                game_state.disconnected_players.push(player.id);
                
                // If this was the active player, pass priority
                if game_state.active_player_id == player.id {
                    advance_to_next_player(&mut game_state);
                }
            }
        }
    }
}
```

### Reconnection and State Recovery

The system supports client reconnection and state recovery:

```rust
// System to handle client reconnection
fn handle_client_reconnection(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RepliconServer>,
    game_state: Res<GameState>,
    players: Query<(Entity, &Player, Option<&Disconnected>)>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        if let ServerEvent::ClientConnected { client_id } = event {
            // Check if this is a reconnecting player
            if let Some((player_entity, player, Some(disconnected))) = players
                .iter()
                .find(|(_, p, _)| p.client_id == *client_id)
            {
                // Remove disconnected marker
                commands.entity(player_entity).remove::<Disconnected>();
                
                // Send full game state to reconnected client
                let full_state = create_full_state_for_client(*client_id, &game_state);
                server.send_message(*client_id, FullGameState { state: full_state });
                
                // Notify other players about reconnection
                server.broadcast_message_except(
                    *client_id,
                    PlayerReconnected { player_id: player.id }
                );
            }
        }
    }
}
```

## Testing Synchronization

Testing the synchronization system involves:

1. **Latency Simulation**: Testing with artificial network delays
2. **Packet Loss Simulation**: Testing with dropped packets
3. **Reconnection Testing**: Testing disconnection and reconnection scenarios
4. **Stress Testing**: Testing with many simultaneous actions

For detailed testing procedures, see the [Integration Testing Strategy](../../testing/integration/strategy.md).

## Future Enhancements

Planned improvements to synchronization include:

- Predictive client-side actions for better responsiveness
- Enhanced state compression for better network performance
- More sophisticated reconnection handling for long disconnections
- Support for spectator mode with appropriate synchronization

---

This documentation will be updated as synchronization mechanisms evolve.

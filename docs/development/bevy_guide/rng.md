# Random Number Generation with bevy_rand

This guide explains how to implement deterministic random number generation in Rummage using `bevy_rand` and `bevy_prng`, with a focus on entity-attached RNGs for networked state synchronization.

## Table of Contents

1. [Overview](#overview)
2. [Setup and Configuration](#setup-and-configuration)
3. [Entity-Attached RNGs](#entity-attached-rngs)
4. [Networked State Synchronization](#networked-state-synchronization)
5. [Testing and Debugging](#testing-and-debugging)
6. [Implementation Patterns](#implementation-patterns)
7. [Performance Considerations](#performance-considerations)

## Overview

In Rummage, deterministic random number generation is critical for:

1. **Networked Gameplay**: Ensuring all clients produce identical results when processing the same game actions
2. **Replay Functionality**: Allowing game sessions to be accurately replayed
3. **Testing**: Creating reproducible test scenarios

The `bevy_rand` ecosystem provides the tools we need to implement deterministic RNG that can be:
- Attached to specific entities
- Serialized/deserialized for network transmission
- Rolled back and restored for state reconciliation

## Setup and Configuration

### Dependencies

In your `Cargo.toml`, include the following dependencies:

```toml
[dependencies]
bevy_rand = { git = "https://github.com/Bluefinger/bevy_rand", branch = "main", features = ["wyrand", "experimental"] }
bevy_prng = { git = "https://github.com/Bluefinger/bevy_rand", branch = "main", features = ["wyrand"] }
```

The `wyrand` feature specifies the WyRand algorithm, which provides a good balance of performance and quality. The `experimental` feature enables `bevy_rand`'s entity-attached RNG functionality.

### Plugin Registration

Set up the RNG system in your app:

```rust
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;

fn main() {
    App::new()
        // Add the entropy plugin with WyRand algorithm
        .add_plugins(EntropyPlugin::<WyRand>::default())
        // Your other plugins...
        .run();
}
```

### Seeding for Determinism

For deterministic behavior, seed the global RNG at the start of your game:

```rust
fn setup_deterministic_rng(mut global_entropy: ResMut<GlobalEntropy<WyRand>>) {
    // Use a fixed seed for testing or derive from game parameters
    let seed = 12345u64;
    global_entropy.seed_from_u64(seed);
}
```

For multiplayer games, the server should generate the seed and communicate it to clients during game initialization.

## Entity-Attached RNGs

### Core Concept

Entity-attached RNGs allow different game entities (players, decks, etc.) to have their own independent but deterministic random number generators. This is critical for:

1. **Isolation**: Each entity's randomization is independent of others
2. **Reproducibility**: Given the same initial state, entities will produce the same sequence of random numbers
3. **State Management**: Entity RNG state can be saved, restored, and synchronized

### Creating Entity-Attached RNGs

```rust
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;

// Component to hold an entity's RNG
#[derive(Component)]
struct EntityRng(Entropy<WyRand>);

// System to set up RNGs for entities that need them
fn setup_entity_rngs(
    mut commands: Commands,
    entities: Query<Entity, (With<Player>, Without<EntityRng>)>,
    mut global_entropy: ResMut<GlobalEntropy<WyRand>>,
) {
    for entity in &entities {
        // Create a new RNG forked from the global entropy source
        let entity_rng = global_entropy.fork_rng();
        
        // Attach it to the entity
        commands.entity(entity).insert(EntityRng(entity_rng));
    }
}
```

### Using Entity-Attached RNGs

To use an entity's RNG:

```rust
fn shuffle_player_deck(
    mut players: Query<(&Player, &mut EntityRng)>,
    mut decks: Query<&mut Deck>,
) {
    for (player, mut entity_rng) in &mut players {
        if let Ok(mut deck) = decks.get_mut(player.deck_entity) {
            // Use the player's RNG to shuffle their deck
            deck.shuffle_with_rng(&mut entity_rng.0);
        }
    }
}
```

## Networked State Synchronization

### Serializing RNG State

For network transmission, RNG state must be serialized:

```rust
// Resource to track RNG state for network sync
#[derive(Resource)]
struct NetworkedRngState {
    // Global RNG state
    global_state: Vec<u8>,
    // Player entity RNG states mapped by entity ID
    entity_states: HashMap<Entity, Vec<u8>>,
    // Last sync timestamp
    last_sync: f32,
}

// System to capture RNG states for replication
fn capture_rng_states(
    global_entropy: Res<GlobalEntropy<WyRand>>,
    entity_rngs: Query<(Entity, &EntityRng)>,
    mut networked_state: ResMut<NetworkedRngState>,
    time: Res<Time>,
) {
    // Only sync periodically to reduce network traffic
    if time.elapsed_seconds() - networked_state.last_sync < 5.0 {
        return;
    }
    
    // Capture global RNG state
    if let Ok(serialized) = global_entropy.try_serialize_state() {
        networked_state.global_state = serialized;
    }
    
    // Capture entity RNG states
    for (entity, entity_rng) in &entity_rngs {
        if let Ok(serialized) = entity_rng.0.try_serialize_state() {
            networked_state.entity_states.insert(entity, serialized);
        }
    }
    
    networked_state.last_sync = time.elapsed_seconds();
}
```

### Transmitting RNG State

Use Bevy Replicon to efficiently sync RNG state between server and clients:

```rust
use bevy_replicon::prelude::*;

// Server-authoritative replication
#[derive(Component, Serialize, Deserialize, Clone)]
struct ReplicatedRngState {
    state: Vec<u8>,
    last_updated: f32,
}

// System to update replication components
fn update_rng_replication(
    mut commands: Commands,
    players: Query<(Entity, &EntityRng)>,
    time: Res<Time>,
) {
    for (entity, entity_rng) in &players {
        if let Ok(serialized) = entity_rng.0.try_serialize_state() {
            commands.entity(entity).insert(ReplicatedRngState {
                state: serialized,
                last_updated: time.elapsed_seconds(),
            });
        }
    }
}
```

### Restoring RNG State on Clients

```rust
// System to apply RNG state updates from server
fn apply_rng_state_updates(
    mut players: Query<(Entity, &ReplicatedRngState, &mut EntityRng)>,
    mut applied_states: Local<HashMap<Entity, f32>>,
) {
    for (entity, replicated_state, mut entity_rng) in &mut players {
        // Check if this is a newer state than what we've already applied
        if !applied_states.contains_key(&entity) || 
           applied_states[&entity] < replicated_state.last_updated {
            
            // Apply the updated state
            if let Ok(()) = entity_rng.0.deserialize_state(&replicated_state.state) {
                applied_states.insert(entity, replicated_state.last_updated);
            }
        }
    }
}
```

## Testing and Debugging

### Verifying Determinism

To verify RNG determinism, create a test that:
1. Seeds multiple RNGs with the same seed
2. Generates a sequence of random values from each
3. Compares the sequences for equality

```rust
#[test]
fn test_rng_determinism() {
    // Create two separate RNGs with the same seed
    let seed = 12345u64;
    let mut rng1 = WyRand::seed_from_u64(seed);
    let mut rng2 = WyRand::seed_from_u64(seed);
    
    // Generate sequences from both RNGs
    let sequence1: Vec<u32> = (0..100).map(|_| rng1.gen_range(0..1000)).collect();
    let sequence2: Vec<u32> = (0..100).map(|_| rng2.gen_range(0..1000)).collect();
    
    // Verify sequences are identical
    assert_eq!(sequence1, sequence2, "RNG sequences should be identical with the same seed");
}
```

### Debugging Network Desynchronization

When RNG state gets out of sync across the network:

1. **Add Logging**: Log RNG states and the random values they generate
   ```rust
   info!("Entity {}: RNG state hash: {:?}, Next value: {}", 
       entity, hash_rng_state(&entity_rng.0), entity_rng.0.gen_range(0..100));
   ```

2. **State Comparison**: Compare serialized RNG states between server and clients
   ```rust
   fn debug_rng_states(
       server_states: &HashMap<Entity, Vec<u8>>,
       client_states: &HashMap<Entity, Vec<u8>>,
   ) {
       for (entity, server_state) in server_states {
           if let Some(client_state) = client_states.get(entity) {
               if server_state != client_state {
                   warn!("RNG state mismatch for entity {}", entity);
               }
           }
       }
   }
   ```

3. **Event Logging**: Track every action that uses RNG to pinpoint where divergence occurs

## Implementation Patterns

### Deck Shuffling

For card games like MTG, deck shuffling must be consistent across the network:

```rust
// Component for a deck of cards
#[derive(Component)]
struct Deck {
    cards: Vec<Entity>,
}

// System to shuffle a deck using entity-attached RNG
fn shuffle_deck(
    mut decks: Query<&mut Deck>,
    deck_owner: Query<&DeckOwner>,
    mut players: Query<&mut EntityRng>,
    mut shuffle_events: EventReader<ShuffleDeckEvent>,
) {
    for event in shuffle_events.iter() {
        // Get the deck
        if let Ok(mut deck) = decks.get_mut(event.deck_entity) {
            // Find the deck owner
            if let Ok(owner) = deck_owner.get(event.deck_entity) {
                // Get the owner's RNG
                if let Ok(mut entity_rng) = players.get_mut(owner.0) {
                    // Use Fisher-Yates shuffle with the owner's RNG
                    let mut cards = deck.cards.clone();
                    for i in (1..cards.len()).rev() {
                        let j = entity_rng.0.gen_range(0..=i);
                        cards.swap(i, j);
                    }
                    deck.cards = cards;
                }
            }
        }
    }
}
```

### Random Card Selection

For abilities that select random targets:

```rust
fn random_target_selection(
    mut commands: Commands,
    mut ability_events: EventReader<AbilityActivatedEvent>,
    players: Query<&EntityRng>,
    targets: Query<Entity, With<Targetable>>,
) {
    for event in ability_events.iter() {
        if event.ability_type == AbilityType::RandomTarget {
            // Get the entity's RNG
            if let Ok(entity_rng) = players.get(event.player_entity) {
                let target_entities: Vec<Entity> = targets.iter().collect();
                
                if !target_entities.is_empty() {
                    // Select a random target using the entity's RNG
                    let random_index = entity_rng.0.gen_range(0..target_entities.len());
                    let selected_target = target_entities[random_index];
                    
                    // Apply the ability effect to the selected target
                    commands.entity(selected_target).insert(AbilityEffect {
                        source: event.player_entity,
                        effect_type: event.effect_type,
                    });
                }
            }
        }
    }
}
```

## Performance Considerations

### Minimizing RNG Operations

Random number generation can be computationally expensive:

1. **Cache Random Results**: Generate batches of random values when possible
   ```rust
   // Generate and cache random values
   for _ in 0..10 {
       let value = entity_rng.0.gen_range(0..100);
       cached_values.push(value);
   }
   ```

2. **Optimize RNG Distribution**: Use the most efficient distribution for your needs
   ```rust
   // For uniform integer distributions, use gen_range
   let value = rng.gen_range(0..100);
   
   // For weighted choices, use a weight-optimized approach
   let choices = vec![(option1, 10), (option2, 5), (option3, 1)];
   let total_weight: u32 = choices.iter().map(|(_, w)| w).sum();
   let mut rng_value = rng.gen_range(0..total_weight);
   
   for (option, weight) in choices {
       if rng_value < *weight {
           selected = option;
           break;
       }
       rng_value -= *weight;
   }
   ```

3. **Schedule RNG Operations**: Spread intensive RNG work across frames

### State Synchronization Frequency

Synchronize RNG state efficiently:

1. **Event-Driven Updates**: Sync after significant random events rather than on a timer
2. **Delta Compression**: Only send changes to RNG state
3. **Prioritize Critical Entities**: Sync more frequently for gameplay-critical entities

---

By following these guidelines, you can create a robust, deterministic random number generation system that works reliably across network boundaries in your Bevy application. 
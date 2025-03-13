# Testing Replicon Integration with RNG State Management

This document outlines specific test cases and methodologies for verifying the correct integration of bevy_replicon with our RNG state management system.

## Table of Contents

1. [Introduction](#introduction)
2. [Test Environment Setup](#test-environment-setup)
3. [Unit Tests](#unit-tests)
4. [Integration Tests](#integration-tests)
5. [End-to-End Tests](#end-to-end-tests)
6. [Performance Tests](#performance-tests)
7. [Debugging Failures](#debugging-failures)

## Introduction

Testing the integration of bevy_replicon with RNG state management presents unique challenges:

1. Network conditions are variable and unpredictable
2. Randomized operations must be deterministic across network boundaries
3. Rollbacks must preserve the exact RNG state
4. Any deviations in RNG state can lead to unpredictable game outcomes

Our testing approach focuses on verifying determinism under various network conditions and ensuring proper recovery after disruptions.

## Test Environment Setup

### Local Network Testing Harness

```rust
/// Struct for testing replicon and RNG integration 
pub struct RepliconRngTestHarness {
    /// Server application
    pub server_app: App,
    /// Client applications (can have multiple)
    pub client_apps: Vec<App>,
    /// Network conditions simulator
    pub network_conditions: NetworkConditionSimulator,
    /// Test seed for deterministic behavior
    pub test_seed: u64,
}

impl RepliconRngTestHarness {
    /// Create a new test harness with the specified number of clients
    pub fn new(num_clients: usize) -> Self {
        let mut server_app = App::new();
        let mut client_apps = Vec::with_capacity(num_clients);
        
        // Setup server
        server_app.add_plugins(MinimalPlugins)
                  .add_plugins(DefaultRngPlugin)
                  .add_plugins(RepliconServerPlugin::default())
                  .add_plugin(RepliconRngRollbackPlugin);
        
        // Setup RNG with specific seed for repeatability
        let test_seed = 12345u64;
        server_app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
        
        // Setup clients
        for _ in 0..num_clients {
            let mut client_app = App::new();
            client_app.add_plugins(MinimalPlugins)
                     .add_plugins(DefaultRngPlugin)
                     .add_plugins(RepliconClientPlugin::default())
                     .add_plugin(RepliconRngRollbackPlugin);
                     
            // Each client gets the same seed
            client_app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
            
            client_apps.push(client_app);
        }
        
        Self {
            server_app,
            client_apps,
            network_conditions: NetworkConditionSimulator::default(),
            test_seed,
        }
    }
    
    /// Connect all clients to the server
    pub fn connect_all_clients(&mut self) {
        // Setup server to listen
        let server_port = 8080;
        self.server_app.world.resource_mut::<RepliconServer>()
            .start_endpoint(ServerEndpoint::new(server_port));
        
        // Connect clients
        for (i, client_app) in self.client_apps.iter_mut().enumerate() {
            client_app.world.resource_mut::<RepliconClient>()
                .connect_endpoint(ClientEndpoint::new("127.0.0.1", server_port));
        }
        
        // Update a few times to establish connections
        for _ in 0..10 {
            self.server_app.update();
            for client_app in &mut self.client_apps {
                client_app.update();
            }
        }
    }
    
    /// Simulate network disruption for a specific client
    pub fn simulate_disruption(&mut self, client_idx: usize, duration_ms: u64) {
        self.network_conditions.disconnect_client(client_idx, duration_ms);
    }
    
    /// Run a test with network conditions
    pub fn run_with_conditions<F>(&mut self, update_count: usize, test_fn: F) 
    where F: Fn(&mut Self, usize) {
        for i in 0..update_count {
            // Apply network conditions
            self.network_conditions.update(&mut self.client_apps);
            
            // Run server update
            self.server_app.update();
            
            // Run client updates
            for client_app in &mut self.client_apps {
                client_app.update();
            }
            
            // Call test function
            test_fn(self, i);
        }
    }
}

/// Simulates different network conditions
pub struct NetworkConditionSimulator {
    /// Client disconnection timers
    pub disconnection_timers: HashMap<usize, u64>,
    /// Packet loss percentages
    pub packet_loss_rates: HashMap<usize, f32>,
    /// Latency values
    pub latencies: HashMap<usize, u64>,
}

impl NetworkConditionSimulator {
    /// Disconnect a client for a duration
    pub fn disconnect_client(&mut self, client_idx: usize, duration_ms: u64) {
        self.disconnection_timers.insert(client_idx, duration_ms);
    }
    
    /// Apply network conditions to clients
    pub fn update(&mut self, client_apps: &mut [App]) {
        // Update disconnection timers and reconnect if needed
        let mut reconnect = Vec::new();
        for (client_idx, timer) in &mut self.disconnection_timers {
            if *timer <= 16 {
                reconnect.push(*client_idx);
            } else {
                *timer -= 16; // Assuming 60 FPS
            }
        }
        
        for client_idx in reconnect {
            self.disconnection_timers.remove(&client_idx);
        }
    }
}
```

## Unit Tests

### Testing RNG State Serialization and Deserialization

```rust
#[test]
fn test_rng_state_serialization() {
    // Create a test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(DefaultRngPlugin)
       .init_resource::<RngReplicationState>();
    
    // Setup global RNG with specific seed
    let test_seed = 12345u64;
    app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
    
    // Generate some random values and store them
    let original_values: Vec<u32> = {
        let mut rng = app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.gen::<u32>()).collect()
    };
    
    // Capture the RNG state
    let mut rng_state = app.world.resource_mut::<RngReplicationState>();
    let global_rng = app.world.resource::<GlobalEntropy<WyRand>>();
    rng_state.global_state = global_rng.try_serialize_state().unwrap();
    
    // Create a new app with fresh RNG
    let mut new_app = App::new();
    new_app.add_plugins(MinimalPlugins)
           .add_plugins(DefaultRngPlugin);
    
    // Apply the saved state
    let mut new_global_rng = new_app.world.resource_mut::<GlobalEntropy<WyRand>>();
    new_global_rng.deserialize_state(&rng_state.global_state).unwrap();
    
    // Generate values from the new RNG
    let new_values: Vec<u32> = {
        let mut rng = new_app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.gen::<u32>()).collect()
    };
    
    // Values should be different from the original sequence
    // because we captured the state after generating the original values
    assert_ne!(original_values, new_values);
    
    // Reset both RNGs to the same seed and generate sequences
    app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
    new_app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
    
    let reset_values1: Vec<u32> = {
        let mut rng = app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.gen::<u32>()).collect()
    };
    
    let reset_values2: Vec<u32> = {
        let mut rng = new_app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.gen::<u32>()).collect()
    };
    
    // Values should now be identical
    assert_eq!(reset_values1, reset_values2);
}
```

### Testing Checkpoint Creation and Restoration

```rust
#[test]
fn test_checkpoint_creation_and_restoration() {
    // Create a test app with the plugin
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(DefaultRngPlugin)
       .add_plugin(RepliconRngRollbackPlugin)
       .init_resource::<SequenceTracker>();
    
    // Seed RNG
    let test_seed = 12345u64;
    app.world.resource_mut::<GlobalEntropy<WyRand>>().seed_from_u64(test_seed);
    
    // Generate some initial values
    let initial_values: Vec<u32> = {
        let mut rng = app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..5).map(|_| rng.gen::<u32>()).collect()
    };
    
    // Create a checkpoint
    let checkpoint_sequence = 1;
    let mut checkpoints = app.world.resource_mut::<RollbackCheckpoints>();
    let rng_state = app.world.resource::<RngReplicationState>();
    let global_rng = app.world.resource::<GlobalEntropy<WyRand>>();
    
    let checkpoint = RollbackCheckpoint {
        sequence_id: checkpoint_sequence,
        timestamp: 0.0,
        global_rng_state: global_rng.try_serialize_state().unwrap(),
        player_rng_states: HashMap::new(),
        replicated_entities: Vec::new(),
    };
    
    checkpoints.checkpoints.insert(checkpoint_sequence, checkpoint);
    
    // Generate more values after checkpoint
    let post_checkpoint_values: Vec<u32> = {
        let mut rng = app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..5).map(|_| rng.gen::<u32>()).collect()
    };
    
    // Restore from checkpoint
    let checkpoint = checkpoints.checkpoints.get(&checkpoint_sequence).unwrap();
    app.world.resource_mut::<GlobalEntropy<WyRand>>()
        .deserialize_state(&checkpoint.global_rng_state).unwrap();
    
    // Generate values after restoration
    let restored_values: Vec<u32> = {
        let mut rng = app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..5).map(|_| rng.gen::<u32>()).collect()
    };
    
    // The restored values should match the post-checkpoint values
    assert_eq!(post_checkpoint_values, restored_values);
}
```

## Integration Tests

### Testing RNG Synchronization Between Server and Client

```rust
#[test]
fn test_server_client_rng_sync() {
    // Create test harness with 1 client
    let mut harness = RepliconRngTestHarness::new(1);
    harness.connect_all_clients();
    
    // Test variables
    let mut server_values = Vec::new();
    let mut client_values = Vec::new();
    
    // Run with updates
    harness.run_with_conditions(50, |harness, i| {
        if i == 10 {
            // Record server RNG values at update 10
            let mut rng = harness.server_app.world.resource_mut::<GlobalEntropy<WyRand>>();
            server_values = (0..5).map(|_| rng.gen::<u32>()).collect();
        }
        
        if i == 20 {
            // Record client RNG values at update 20
            // By now, RNG state should have been synced
            let mut rng = harness.client_apps[0].world.resource_mut::<GlobalEntropy<WyRand>>();
            client_values = (0..5).map(|_| rng.gen::<u32>()).collect();
            
            // Server will have advanced, get fresh set of values
            let mut rng = harness.server_app.world.resource_mut::<GlobalEntropy<WyRand>>();
            server_values = (0..5).map(|_| rng.gen::<u32>()).collect();
        }
    });
    
    // Client values should match server values from update 20
    assert_eq!(client_values, server_values);
}
```

### Testing Rollback Due to Network Disruption

```rust
#[test]
fn test_rollback_after_disruption() {
    // Create test harness with 2 clients
    let mut harness = RepliconRngTestHarness::new(2);
    harness.connect_all_clients();
    
    // Setup game entities
    // ...
    
    // Run test with network disruption
    let mut pre_disruption_rng_values = Vec::new();
    let mut post_disruption_rng_values = Vec::new();
    let mut post_rollback_rng_values = Vec::new();
    
    harness.run_with_conditions(100, |harness, i| {
        if i == 20 {
            // Record RNG values before disruption
            let rng = harness.server_app.world.resource::<GlobalEntropy<WyRand>>();
            pre_disruption_rng_values = generate_test_random_values(rng, 10);
            
            // Simulate network disruption for client 0
            harness.simulate_disruption(0, 500); // 500ms disruption
        }
        
        if i == 40 {
            // Record RNG values after disruption
            let rng = harness.server_app.world.resource::<GlobalEntropy<WyRand>>();
            post_disruption_rng_values = generate_test_random_values(rng, 10);
        }
        
        if i == 60 {
            // By now rollback should have happened
            // Record RNG values after rollback
            let rng = harness.server_app.world.resource::<GlobalEntropy<WyRand>>();
            post_rollback_rng_values = generate_test_random_values(rng, 10);
            
            // Check that client 0 and client 1 have the same RNG state
            let rng0 = harness.client_apps[0].world.resource::<GlobalEntropy<WyRand>>();
            let rng1 = harness.client_apps[1].world.resource::<GlobalEntropy<WyRand>>();
            
            let client0_values = generate_test_random_values(rng0, 10);
            let client1_values = generate_test_random_values(rng1, 10);
            
            assert_eq!(client0_values, client1_values, "Clients should have same RNG state after rollback");
        }
    });
    
    // Verify behavior
    assert_ne!(pre_disruption_rng_values, post_disruption_rng_values, 
               "RNG values should change during normal operation");
    assert_eq!(post_rollback_rng_values, post_disruption_rng_values, 
               "After rollback, RNG sequences should match the checkpoint state");
}

/// Helper function to generate random values for testing
fn generate_test_random_values(rng: &GlobalEntropy<WyRand>, count: usize) -> Vec<u32> {
    let mut rng_clone = rng.clone();
    (0..count).map(|_| rng_clone.gen::<u32>()).collect()
}
```

## End-to-End Tests

### Testing Card Shuffling During Network Disruption

```rust
#[test]
fn test_card_shuffle_during_disruption() {
    // Setup test environment with card library
    let mut harness = RepliconRngTestHarness::new(2);
    harness.connect_all_clients();
    
    // Create players and libraries
    let server_player1 = setup_test_player(&mut harness.server_app.world, 1);
    let server_player2 = setup_test_player(&mut harness.server_app.world, 2);
    
    // Create identical card libraries
    let cards = (1..53).collect::<Vec<i32>>();
    let server_library1 = create_test_library(&mut harness.server_app.world, server_player1, cards.clone());
    let server_library2 = create_test_library(&mut harness.server_app.world, server_player2, cards.clone());
    
    // Initialize client players and libraries
    // ...
    
    // Shuffle results
    let mut server_shuffle_result1 = Vec::new();
    let mut server_shuffle_result2 = Vec::new();
    let mut client1_shuffle_result = Vec::new();
    let mut client2_shuffle_result = Vec::new();
    
    // Run test with network disruption during card shuffle
    harness.run_with_conditions(200, |harness, i| {
        if i == 50 {
            // Player 1 shuffles their library
            harness.server_app.world.send_event(ShuffleLibraryEvent { 
                library_entity: server_library1 
            });
        }
        
        if i == 60 {
            // Capture shuffle result
            server_shuffle_result1 = get_library_order(&harness.server_app.world, server_library1);
            
            // Cause network disruption
            harness.simulate_disruption(0, 1000);
        }
        
        if i == 80 {
            // Player 2 shuffles during disruption
            harness.server_app.world.send_event(ShuffleLibraryEvent { 
                library_entity: server_library2 
            });
        }
        
        if i == 100 {
            // Capture server-side shuffle results
            server_shuffle_result2 = get_library_order(&harness.server_app.world, server_library2);
        }
        
        if i == 150 {
            // By now, rollback and resynchronization should have occurred
            // Capture client-side shuffle results
            client1_shuffle_result = get_client_library_order(&harness.client_apps[0].world, 1);
            client2_shuffle_result = get_client_library_order(&harness.client_apps[1].world, 2);
        }
    });
    
    // Verify all libraries have the same shuffle result
    assert_eq!(server_shuffle_result1, client1_shuffle_result, 
               "Client 1 should have same shuffle result as server");
    assert_eq!(server_shuffle_result2, client2_shuffle_result, 
               "Client 2 should have same shuffle result as server");
}

/// Helper function to get library card order
fn get_library_order(world: &World, library_entity: Entity) -> Vec<i32> {
    if let Some(library) = world.get::<Library>(library_entity) {
        library.cards.clone()
    } else {
        Vec::new()
    }
}

/// Helper function to get client-side library order
fn get_client_library_order(client_world: &World, player_id: i32) -> Vec<i32> {
    // Find player by ID
    let player_entity = find_player_by_id(client_world, player_id);
    if player_entity.is_none() { return Vec::new(); }
    
    // Find library entity
    let library_entity = find_library_for_player(client_world, player_entity.unwrap());
    if library_entity.is_none() { return Vec::new(); }
    
    // Get library cards
    get_library_order(client_world, library_entity.unwrap())
}
```

## Performance Tests

### Testing RNG State Replication Bandwidth

```rust
#[test]
fn test_rng_replication_bandwidth() {
    // Create a test harness with multiple clients
    let mut harness = RepliconRngTestHarness::new(4);
    harness.connect_all_clients();
    
    // Setup bandwidth tracking
    let mut bandwidth_tracker = BandwidthTracker::new();
    
    // Run test with bandwidth monitoring
    harness.run_with_conditions(100, |harness, i| {
        if i % 10 == 0 {
            // Record bandwidth usage every 10 updates
            let server = harness.server_app.world.resource::<RepliconServer>();
            bandwidth_tracker.record_bandwidth(server.get_bandwidth_stats());
        }
    });
    
    // Analyze bandwidth results
    let results = bandwidth_tracker.analyze();
    
    // Ensure RNG state replication is within reasonable bounds
    assert!(results.avg_bandwidth_per_client < 1024, 
            "Average bandwidth should be less than 1KB per client");
    
    // Print results
    println!("Bandwidth results:");
    println!("  Average per client: {} bytes", results.avg_bandwidth_per_client);
    println!("  Peak: {} bytes", results.peak_bandwidth);
    println!("  Total: {} bytes", results.total_bandwidth);
}

/// Helper struct for tracking bandwidth usage
struct BandwidthTracker {
    samples: Vec<BandwidthSample>,
}

struct BandwidthSample {
    timestamp: f32,
    bytes_sent: usize,
    client_count: usize,
}

struct BandwidthResults {
    avg_bandwidth_per_client: f32,
    peak_bandwidth: usize,
    total_bandwidth: usize,
}

impl BandwidthTracker {
    fn new() -> Self {
        Self { samples: Vec::new() }
    }
    
    fn record_bandwidth(&mut self, stats: BandwidthStats) {
        self.samples.push(BandwidthSample {
            timestamp: stats.timestamp,
            bytes_sent: stats.bytes_sent,
            client_count: stats.client_count,
        });
    }
    
    fn analyze(&self) -> BandwidthResults {
        if self.samples.is_empty() {
            return BandwidthResults {
                avg_bandwidth_per_client: 0.0,
                peak_bandwidth: 0,
                total_bandwidth: 0,
            };
        }
        
        let total_bytes: usize = self.samples.iter().map(|s| s.bytes_sent).sum();
        let peak_bytes = self.samples.iter().map(|s| s.bytes_sent).max().unwrap_or(0);
        
        let client_samples: usize = self.samples.iter().map(|s| s.client_count).sum();
        let avg_per_client = if client_samples > 0 {
            total_bytes as f32 / client_samples as f32
        } else {
            0.0
        };
        
        BandwidthResults {
            avg_bandwidth_per_client: avg_per_client,
            peak_bandwidth: peak_bytes,
            total_bandwidth: total_bytes,
        }
    }
}

/// Mock struct to represent network bandwidth statistics
struct BandwidthStats {
    timestamp: f32,
    bytes_sent: usize,
    client_count: usize,
}
```

## Debugging Failures

When tests fail, collect diagnostic information to aid debugging:

```rust
fn diagnose_rng_state_mismatch(
    server_rng: &GlobalEntropy<WyRand>,
    client_rng: &GlobalEntropy<WyRand>,
) -> String {
    // Serialize both RNG states
    let server_state = server_rng.try_serialize_state().unwrap_or_default();
    let client_state = client_rng.try_serialize_state().unwrap_or_default();
    
    // Generate test values from both
    let mut server_rng_clone = server_rng.clone();
    let mut client_rng_clone = client_rng.clone();
    
    let server_values: Vec<u32> = (0..5).map(|_| server_rng_clone.gen::<u32>()).collect();
    let client_values: Vec<u32> = (0..5).map(|_| client_rng_clone.gen::<u32>()).collect();
    
    let mut report = String::new();
    report.push_str("RNG State Mismatch Diagnostic:\n");
    report.push_str(&format!("Server state: {:?}\n", server_state));
    report.push_str(&format!("Client state: {:?}\n", client_state));
    report.push_str(&format!("Server values: {:?}\n", server_values));
    report.push_str(&format!("Client values: {:?}\n", client_values));
    
    report
}
```

---

These tests validate that our bevy_replicon integration with RNG state management works correctly under various conditions, ensuring deterministic behavior in our networked MTG Commander game.

Remember to run these tests:
1. Regularly during development
2. After any changes to networking code
3. After any changes to RNG-dependent game logic
4. As part of the CI/CD pipeline 
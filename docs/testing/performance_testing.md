# Performance Testing

Performance testing in Rummage ensures the game remains responsive and efficient, even in complex game states with many cards and interactions. This documentation covers approaches to measuring and optimizing performance.

## Overview

Performance testing in Rummage focuses on:
- Frame rate stability during gameplay
- Memory usage optimization
- CPU utilization efficiency
- Load scalability with increasing game complexity

## Testing Approach

Performance testing uses several methodologies:

1. **Benchmark Tests**: Measure execution time of critical systems
2. **Load Tests**: Evaluate performance under increasing entity counts
3. **Profiling**: Identify bottlenecks using system profiling tools
4. **Resource Monitoring**: Track memory and CPU utilization

## Benchmark Tests

Benchmark tests measure the performance of specific systems or operations:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_state_based_actions(c: &mut Criterion) {
    c.bench_function("state based actions - 100 permanents", |b| {
        b.iter_batched(
            || {
                // Setup test environment with 100 permanents
                let mut app = App::new();
                app.add_plugins(MinimalPlugins)
                   .add_systems(Update, check_state_based_actions);
                
                // Create 100 permanents with various states
                for i in 0..100 {
                    if i % 3 == 0 {
                        // Create a 0/0 creature that should die
                        app.world.spawn((
                            CardMarker,
                            Creature { power: 0, toughness: 0 },
                            InPlay,
                        ));
                    } else if i % 3 == 1 {
                        // Create a damaged creature
                        app.world.spawn((
                            CardMarker,
                            Creature { power: 2, toughness: 2 },
                            InPlay,
                            DamageMarkers { amount: 1 },
                        ));
                    } else {
                        // Create an enchantment
                        app.world.spawn((
                            CardMarker,
                            EnchantmentType,
                            InPlay,
                        ));
                    }
                }
                
                app
            },
            |mut app| {
                // Execute state-based actions
                app.world.send_event(CheckStateBasedActionsEvent);
                black_box(app.update());
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark_state_based_actions);
criterion_main!(benches);
```

## Load Testing

Load tests evaluate how the system performs as load increases:

```rust
#[test]
fn test_battlefield_scaling() {
    // Test with different permanent counts
    let permanent_counts = [10, 100, 500, 1000, 5000];
    
    let mut results = Vec::new();
    
    for count in permanent_counts {
        // Setup test environment
        let mut app = App::new();
        app.add_plugins(GameTestPlugins);
        
        // Setup battlefield with 'count' permanent cards
        setup_battlefield_with_permanents(&mut app, count);
        
        // Measure performance over multiple updates
        let start_time = std::time::Instant::now();
        const ITERATIONS: usize = 100;
        
        for _ in 0..ITERATIONS {
            app.update();
        }
        
        let elapsed = start_time.elapsed();
        let avg_update_time = elapsed.as_secs_f64() / ITERATIONS as f64;
        
        results.push((count, avg_update_time));
        
        // Ensure performance meets expectations
        assert!(
            avg_update_time < 0.016, // Target: 60 FPS (16ms per frame)
            "Performance degraded with {} permanents: {:.2}ms average update time",
            count,
            avg_update_time * 1000.0
        );
    }
    
    // Log results for analysis
    for (count, time) in results {
        println!("Permanents: {}, Avg Update Time: {:.2}ms", count, time * 1000.0);
    }
}
```

## Memory Usage Testing

Memory tests verify efficient memory utilization:

```rust
#[test]
fn test_memory_usage() {
    // Setup test
    let mut app = App::new();
    app.add_plugins(GameTestPlugins);
    
    // Track initial memory usage
    let initial_memory = get_current_memory_usage();
    
    // Load a comprehensive test game
    setup_complex_game(&mut app);
    
    // Track memory after setup
    let post_setup_memory = get_current_memory_usage();
    
    // Run game for several turns
    for _ in 0..10 {
        simulate_game_turn(&mut app);
    }
    
    // Track memory after gameplay
    let post_gameplay_memory = get_current_memory_usage();
    
    // Verify memory usage is within acceptable limits
    let setup_memory_increase = post_setup_memory - initial_memory;
    println!("Memory increase after setup: {} MB", setup_memory_increase / (1024 * 1024));
    
    let gameplay_memory_increase = post_gameplay_memory - post_setup_memory;
    println!("Memory increase during gameplay: {} MB", gameplay_memory_increase / (1024 * 1024));
    
    // Check for memory leaks (gameplay shouldn't continuously increase memory)
    assert!(
        gameplay_memory_increase < 50 * 1024 * 1024, // 50 MB limit
        "Excessive memory growth during gameplay: {} MB",
        gameplay_memory_increase / (1024 * 1024)
    );
    
    // Cleanup should release memory
    drop(app);
    
    // Optionally force garbage collection if language supports it
    // ...
    
    let final_memory = get_current_memory_usage();
    let memory_not_released = final_memory - initial_memory;
    
    assert!(
        memory_not_released < 5 * 1024 * 1024, // 5 MB limit
        "Potential memory leak detected: {} MB not released",
        memory_not_released / (1024 * 1024)
    );
}

fn get_current_memory_usage() -> usize {
    // Platform-specific implementation
    #[cfg(target_os = "linux")]
    {
        use std::fs::File;
        use std::io::Read;
        
        let mut status = String::new();
        File::open("/proc/self/status")
            .unwrap()
            .read_to_string(&mut status)
            .unwrap();
        
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                return parts[1].parse::<usize>().unwrap() * 1024;
            }
        }
        0
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // Use platform-specific APIs on other platforms
        // For this example, we'll return a dummy value
        0
    }
}
```

## Profiling Integration

Integrate profiling tools to identify bottlenecks:

```rust
#[test]
fn profile_complex_game_state() {
    // Initialize profiling
    #[cfg(feature = "profiling")]
    let _profiling_guard = puffin::ProfilerGuard::new();
    
    // Setup test
    let mut app = App::new();
    
    #[cfg(feature = "profiling")]
    app.add_plugin(puffin_egui::PuffinPlugin);
    
    app.add_plugins(GameTestPlugins);
    
    // Setup complex game state
    setup_complex_game(&mut app);
    
    // Run game with profiling enabled
    #[cfg(feature = "profiling")]
    puffin::profile_scope!("game_simulation");
    
    for turn in 0..5 {
        #[cfg(feature = "profiling")]
        puffin::profile_scope!("turn", format!("Turn {}", turn));
        
        // Process a full turn
        simulate_game_turn(&mut app);
    }
    
    // Export profiling data
    #[cfg(feature = "profiling")]
    puffin::set_scopes_on(false); // Stop profiling
}
```

## Frame Rate Analysis

Test frame rate stability during complex game states:

```rust
#[test]
fn test_frame_rate_stability() {
    // Setup test
    let mut app = App::new();
    app.add_plugins(GamePlugins);
    
    // Create diagnostics resource to track FPS
    app.insert_resource(FrameTimeDiagnostics::default());
    
    // Setup a complex game
    setup_complex_game(&mut app);
    
    // Track frame times over a series of updates
    let mut frame_times = Vec::new();
    
    for _ in 0..300 {
        let start = std::time::Instant::now();
        app.update();
        frame_times.push(start.elapsed().as_secs_f64());
    }
    
    // Calculate statistics
    let avg_frame_time = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
    let fps = 1.0 / avg_frame_time;
    
    // Find worst frame times (99th percentile)
    frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let percentile_99 = frame_times[(frame_times.len() as f64 * 0.99) as usize];
    let worst_fps = 1.0 / percentile_99;
    
    println!("Average FPS: {:.2}", fps);
    println!("99th percentile FPS: {:.2}", worst_fps);
    
    // Assert performance requirements
    assert!(fps >= 60.0, "Average FPS below target: {:.2}", fps);
    assert!(worst_fps >= 30.0, "Worst case FPS too low: {:.2}", worst_fps);
}
```

## CPU Utilization Testing

Monitor CPU usage to identify inefficient algorithms:

```rust
#[test]
fn test_cpu_utilization() {
    // Setup test
    let mut app = App::new();
    app.add_plugins(GameTestPlugins);
    
    // Setup monitoring
    let cpu_monitor = CpuUsageMonitor::new();
    
    // Setup complex game state
    setup_complex_game(&mut app);
    
    // Run for a fixed duration
    let start_time = std::time::Instant::now();
    let duration = std::time::Duration::from_secs(10);
    
    while start_time.elapsed() < duration {
        app.update();
    }
    
    // Get CPU usage statistics
    let usage_stats = cpu_monitor.get_stats();
    
    println!("Average CPU usage: {:.2}%", usage_stats.average);
    println!("Peak CPU usage: {:.2}%", usage_stats.peak);
    
    // Assert reasonable CPU usage
    assert!(
        usage_stats.average < 50.0,
        "Average CPU usage too high: {:.2}%",
        usage_stats.average
    );
}

struct CpuUsageMonitor {
    // Platform-specific implementation
}

impl CpuUsageMonitor {
    fn new() -> Self {
        // Initialize monitoring
        Self { /* ... */ }
    }
    
    fn get_stats(&self) -> CpuUsageStats {
        // Platform-specific implementation to get CPU usage
        CpuUsageStats {
            average: 0.0,
            peak: 0.0,
        }
    }
}

struct CpuUsageStats {
    average: f64,
    peak: f64,
}
```

## Best Practices

For effective performance testing in Rummage:

1. **Establish Baselines**: Define performance targets for different hardware profiles
2. **Automate Tests**: Run performance tests as part of CI/CD pipeline
3. **Track Metrics Over Time**: Monitor performance trends across versions
4. **Test Edge Cases**: Verify performance in complex game states
5. **Profile Regularly**: Use profiling tools to identify bottlenecks
6. **Optimize Critical Paths**: Focus optimization on frequently executed code paths
7. **Balance Memory and Speed**: Find the right tradeoffs for performance

## Performance Testing Tools

Rummage uses several tools for performance testing:

- **Criterion**: Rust benchmarking framework for microbenchmarks
- **Flame Graphs**: Visualization of performance hotspots
- **Memory Profilers**: Track memory allocation and usage
- **Bevy Diagnostics**: Built-in performance metrics
- **Custom Telemetry**: Application-specific performance tracking

## Common Performance Bottlenecks

Performance testing has identified these common bottlenecks:

1. **Entity Iteration**: Inefficient queries over large entity sets
2. **Pathological Card Interactions**: Certain card combinations triggering excessive system runs
3. **UI Rendering**: Complex battlefield visualization with many entities
4. **Physics Simulation**: Card movement and stacking physics
5. **State Synchronization**: Network overhead for replicated state

Each area has targeted optimization strategies documented in the implementation sections.

## Related Documentation

For more information on performance in Rummage, see:

- [Bevy ECS Optimization](../development/bevy_guide/ecs.md)
- [Rendering Performance](../game_gui/cards/card_rendering.md#performance)
- [Network Optimization](../networking/gameplay/latency_compensation.md) 
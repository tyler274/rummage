# In-Game Voice Chat System

This document provides detailed information about the voice chat component of Rummage's in-game communication system.

## Table of Contents

1. [Overview](#overview)
2. [UI Components](#ui-components)
3. [Core Features](#core-features)
4. [Audio Controls](#audio-controls)
5. [Integration With Game State](#integration-with-game-state)
6. [Implementation Details](#implementation-details)
7. [Testing](#testing)

## Overview

The voice chat system provides real-time audio communication between players during Commander games. It enhances the social experience of playing Magic: The Gathering remotely by adding a layer of direct communication that complements the text chat system.

Key design principles:

- **Low Latency**: Prioritizes minimal delay for natural conversation
- **Clarity**: Emphasizes audio quality for clear communication
- **Accessibility**: Provides alternative options for those who can't use voice
- **Non-Intrusive**: Integrates with gameplay without disruption
- **Resource Efficient**: Minimizes performance impact

## UI Components

The voice chat interface consists of several components that integrate with the main chat UI:

### Voice Activation Controls

```
┌─────────────────────────────┐
│ [🎤] Push to Talk [⚙️] [📊] │
└─────────────────────────────┘
```

- **Microphone Button**: Toggle for enabling/disabling voice input
- **Mode Selector**: Switch between Push-to-Talk and Voice Activation
- **Settings Button**: Quick access to voice settings
- **Voice Level Indicator**: Shows current microphone input level

### Speaker Status Indicator

```
┌──────────────────────────────────┐
│ Player1 [🔊] Player2 [🔊] [🔇]   │
└──────────────────────────────────┘
```

- **Player List**: Shows all players in the game
- **Speaker Icon**: Animated icon showing who is currently speaking
- **Volume Controls**: Per-player volume adjustment
- **Mute Button**: Quick toggle to mute all voice chat

### Voice Settings Panel

```
┌───────────────────────────────────────┐
│ Voice Chat Settings             [X]   │
├───────────────────────────────────────┤
│ Input Device: [Microphone▼]           │
│ Output Device: [Speakers▼]            │
│                                       │
│ Input Volume: [==========] 80%        │
│ Output Volume: [========--] 60%       │
│                                       │
│ Voice Activation Level: [===-------]  │
│                                       │
│ [✓] Noise Suppression                 │
│ [✓] Echo Cancellation                 │
│ [ ] Automatically Adjust Levels       │
│                                       │
│ Push-to-Talk Key: [Space]             │
│                                       │
│ [Reset to Defaults]     [Apply]       │
└───────────────────────────────────────┘
```

- **Device Selection**: Input and output device configuration
- **Volume Controls**: Master volume adjustments
- **Activation Settings**: Voice detection sensitivity
- **Audio Processing**: Noise suppression and echo cancellation options
- **Key Bindings**: Configure Push-to-Talk keys

### Voice Activity Indicators

Visual cues integrated into player avatars:

- **Glowing Border**: Indicates a player is speaking
- **Volume Level**: Shows relative volume of speaking player
- **Mute Indicator**: Shows when a player is muted or has muted themselves

## Core Features

### Voice Activation Modes

The system supports multiple ways to activate voice transmission:

- **Push-to-Talk**: Requires holding a key to transmit voice
- **Voice Activation**: Automatically transmits when speech is detected
- **Always On**: Continuously transmits audio (with optional noise gate)
- **Priority Speaker**: Option for game host to override other speakers

### Audio Quality Settings

Configurable audio quality options:

- **Quality Presets**: Low, Medium, High, and Ultra profiles
- **Bandwidth Control**: Automatic adjustment based on network conditions
- **Sample Rate**: Options from 16 kHz to 48 kHz
- **Bit Depth**: 16-bit or 24-bit audio

### Channel Management

Support for multiple audio channels:

- **Global Voice**: Heard by all players in the game
- **Team Voice**: Private channel for team members in team games
- **Private Call**: One-on-one communication between specific players
- **Spectator Channel**: Communication with non-playing observers

## Audio Controls

### Input Controls

Options for managing voice input:

- **Microphone Selection**: Choose between available input devices
- **Input Gain**: Adjust microphone sensitivity
- **Noise Gate**: Filter out background noise below threshold
- **Push-to-Talk Delay**: Set release timing to avoid cutting off words
- **Automatic Gain Control**: Maintain consistent input volume

### Output Controls

Options for managing voice output:

- **Speaker Selection**: Choose between available output devices
- **Master Volume**: Overall voice chat volume control
- **Per-Player Volume**: Individual volume controls for each player
- **Audio Panning**: Spatial positioning of voices (disabled by default)
- **Ducking**: Option to reduce game sound when others are speaking

### Audio Processing

Features for improving voice quality:

- **Noise Suppression**: Reduce background noise
- **Echo Cancellation**: Prevent feedback and echo
- **Voice Clarity Enhancement**: Emphasis on speech frequencies
- **Automatic Level Adjustment**: Balance volume between different players

## Integration With Game State

The voice chat system adapts to the current game state:

### Game Phase Integration

- **Planning Phase**: Normal voice operation
- **Active Phase**: Option to highlight speaking player's cards
- **End Phase**: Notification sounds for voice chat

### Player Status Integration

- **Disconnected Players**: Automatic muting with indicator
- **AFK Detection**: Automatic muting after period of inactivity
- **Priority Status**: Visual indicator when speaking player has priority

### Contextual Features

- **Positional Audio**: Optional feature to position voices based on player's virtual position
- **Effect Filters**: Special voice effects for certain game actions (disabled by default)
- **Auto-Mute**: Option to mute voice during cinematic moments

## Implementation Details

The voice chat is implemented using Bevy's ECS architecture with additional audio processing libraries:

### Data Structures

```rust
/// Voice chat configuration
#[derive(Resource)]
struct VoiceChatConfig {
    enabled: bool,
    input_device_id: Option<String>,
    output_device_id: Option<String>,
    input_volume: f32,
    output_volume: f32,
    activation_mode: VoiceActivationMode,
    activation_threshold: f32,
    push_to_talk_key: KeyCode,
    noise_suppression_level: NoiseSuppressionLevel,
    echo_cancellation: bool,
    auto_gain_control: bool,
}

/// Voice activation modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VoiceActivationMode {
    PushToTalk,
    VoiceActivated,
    AlwaysOn,
    Disabled,
}

/// Noise suppression levels
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoiseSuppressionLevel {
    Off,
    Low,
    Medium,
    High,
    Maximum,
}

/// Voice activity status for a player
#[derive(Component)]
struct VoiceActivityStatus {
    player_id: PlayerId,
    is_speaking: bool,
    is_muted: bool,
    is_muted_by_local: bool,
    volume_level: f32,
    peak_level: f32,
}

/// Voice chat audio buffer
#[derive(Resource)]
struct VoiceAudioBuffer {
    input_buffer: Vec<f32>,
    output_buffer: HashMap<PlayerId, Vec<f32>>,
    buffer_size: usize,
    sample_rate: u32,
}

/// Voice packet for network transmission
#[derive(Event)]
struct VoicePacketEvent {
    player_id: PlayerId,
    audio_data: Vec<u8>,
    sequence_number: u32,
    timestamp: f64,
    channel: VoiceChannel,
}
```

### Components

```rust
/// Component for the voice chat UI controller
#[derive(Component)]
struct VoiceChatController {
    is_settings_open: bool,
    is_expanded: bool,
    active_channel: VoiceChannel,
}

/// Component for voice input indicators
#[derive(Component)]
struct VoiceInputIndicator {
    level: f32,
    speaking_confidence: f32,
    is_active: bool,
}

/// Component for player voice indicators
#[derive(Component)]
struct PlayerVoiceIndicator {
    player_id: PlayerId,
    is_speaking: bool,
}
```

### Audio Capture System

```rust
/// System to capture and process microphone input
fn capture_microphone_input(
    mut audio_buffer: ResMut<VoiceAudioBuffer>,
    voice_config: Res<VoiceChatConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio_input: Res<AudioInputDevice>,
    mut voice_events: EventWriter<VoicePacketEvent>,
    time: Res<Time>,
    local_player: Res<LocalPlayer>,
) {
    // Skip if voice chat is disabled
    if !voice_config.enabled {
        return;
    }
    
    // Check activation mode
    let should_capture = match voice_config.activation_mode {
        VoiceActivationMode::PushToTalk => {
            keyboard_input.pressed(voice_config.push_to_talk_key)
        },
        VoiceActivationMode::VoiceActivated => {
            // Detect voice using energy threshold
            let energy = calculate_audio_energy(&audio_input);
            energy > voice_config.activation_threshold
        },
        VoiceActivationMode::AlwaysOn => true,
        VoiceActivationMode::Disabled => false,
    };
    
    if should_capture {
        // Capture audio from microphone
        let input_samples = audio_input.capture_samples(audio_buffer.buffer_size);
        
        // Apply audio processing (noise suppression, etc.)
        let processed_samples = process_audio_samples(
            &input_samples, 
            voice_config.noise_suppression_level,
            voice_config.echo_cancellation,
        );
        
        // Compress audio for network transmission
        let compressed_data = compress_audio_data(&processed_samples);
        
        // Send voice packet
        voice_events.send(VoicePacketEvent {
            player_id: local_player.id,
            audio_data: compressed_data,
            sequence_number: next_sequence_number(),
            timestamp: time.elapsed_seconds(),
            channel: voice_config.active_channel,
        });
    }
}
```

### Audio Playback System

```rust
/// System to play received voice audio
fn play_voice_audio(
    mut audio_buffer: ResMut<VoiceAudioBuffer>,
    voice_config: Res<VoiceChatConfig>,
    voice_status: Query<&VoiceActivityStatus>,
    mut voice_events: EventReader<VoicePacketEvent>,
    audio_output: Res<AudioOutputDevice>,
    mut player_indicators: Query<(&mut PlayerVoiceIndicator, &mut BackgroundColor)>,
) {
    // Process incoming voice packets
    for packet in voice_events.read() {
        // Skip packets if voice is disabled
        if !voice_config.enabled {
            continue;
        }
        
        // Skip packets from muted players
        if let Ok(status) = voice_status.get_component::<VoiceActivityStatus>(packet.player_id) {
            if status.is_muted || status.is_muted_by_local {
                continue;
            }
        }
        
        // Decompress audio data
        let decompressed_data = decompress_audio_data(&packet.audio_data);
        
        // Apply volume adjustment
        let volume_adjusted = adjust_volume(
            &decompressed_data,
            voice_config.output_volume * get_player_volume(packet.player_id),
        );
        
        // Add to output buffer
        audio_buffer.output_buffer.insert(packet.player_id, volume_adjusted);
        
        // Update speaking indicators
        for (mut indicator, mut background) in &mut player_indicators {
            if indicator.player_id == packet.player_id {
                indicator.is_speaking = true;
                // Change background to indicate speaking
                *background = BackgroundColor(Color::rgba(0.2, 0.8, 0.2, 0.5));
            }
        }
    }
    
    // Mix and play output buffer
    let mixed_output = mix_audio_channels(&audio_buffer.output_buffer);
    audio_output.play_samples(&mixed_output);
    
    // Reset output buffer
    audio_buffer.output_buffer.clear();
    
    // Reset speaking indicators after delay
    // This would typically be done with a timer system
    // Simplified for documentation purposes
}
```

### Voice Chat UI Update System

```rust
/// System to update voice chat UI
fn update_voice_chat_ui(
    mut voice_controller_query: Query<(&mut VoiceChatController, &Children)>,
    mut indicator_query: Query<(&mut VoiceInputIndicator, &mut Node)>,
    voice_status_query: Query<&VoiceActivityStatus>,
    voice_config: Res<VoiceChatConfig>,
    audio_input: Res<AudioInputDevice>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut controller, children) in &mut voice_controller_query {
        // Toggle settings panel
        if keyboard_input.just_pressed(KeyCode::F7) {
            controller.is_settings_open = !controller.is_settings_open;
        }
        
        // Update input indicators
        for (mut indicator, mut node) in &mut indicator_query {
            // Update microphone level indicator
            let current_level = if voice_config.enabled {
                calculate_audio_level(audio_input.get_level())
            } else {
                0.0
            };
            
            indicator.level = smooth_level_transition(indicator.level, current_level, 0.1);
            
            // Update indicator visual size based on level
            let indicator_height = Val::Px(20.0 + indicator.level * 30.0);
            if node.height != indicator_height {
                node.height = indicator_height;
            }
            
            // Update active state
            indicator.is_active = match voice_config.activation_mode {
                VoiceActivationMode::PushToTalk => {
                    keyboard_input.pressed(voice_config.push_to_talk_key)
                },
                VoiceActivationMode::VoiceActivated => {
                    indicator.level > voice_config.activation_threshold
                },
                VoiceActivationMode::AlwaysOn => true,
                VoiceActivationMode::Disabled => false,
            };
        }
    }
}
```

## Testing

The voice chat component requires thorough testing:

### Unit Tests

```rust
#[test]
fn test_voice_activation_detection() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, voice_systems::detect_voice_activity);
    
    // Setup test resources
    app.insert_resource(VoiceChatConfig {
        activation_mode: VoiceActivationMode::VoiceActivated,
        activation_threshold: 0.05,
        // ...
    });
    
    // Create mock audio data
    let silent_audio = vec![0.01f32; 1024];
    let speaking_audio = vec![0.2f32; 1024];
    
    // Test silent audio
    app.world.insert_resource(MockAudioInput {
        samples: silent_audio.clone(),
    });
    
    app.update();
    
    let is_active = app.world.resource::<VoiceActivityState>().is_active;
    assert!(!is_active, "Should not detect voice in silent audio");
    
    // Test speaking audio
    app.world.insert_resource(MockAudioInput {
        samples: speaking_audio.clone(),
    });
    
    app.update();
    
    let is_active = app.world.resource::<VoiceActivityState>().is_active;
    assert!(is_active, "Should detect voice in speaking audio");
}
```

### Integration Tests

```rust
#[test]
fn test_voice_chat_network_integration() {
    // Create test app with networking mockup
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(MockNetworkPlugin)
       .add_systems(Update, (
           voice_systems::capture_microphone_input,
           voice_systems::transmit_voice_packets,
           voice_systems::receive_voice_packets,
           voice_systems::play_voice_audio,
       ));
    
    // Setup test game with multiple players
    let (local_id, remote_id) = setup_test_players(&mut app);
    
    // Setup mock audio input with test samples
    let test_audio = generate_test_audio_samples();
    app.insert_resource(MockAudioInput { samples: test_audio });
    
    // Setup voice activation
    app.insert_resource(VoiceChatConfig {
        enabled: true,
        activation_mode: VoiceActivationMode::AlwaysOn,
        // ...
    });
    
    // Run capture and transmission
    app.update();
    
    // Verify packets were sent
    let sent_packets = app.world.resource::<MockNetwork>().sent_packets.clone();
    assert!(!sent_packets.is_empty(), "Voice packets should be sent");
    
    // Simulate receiving packets from the remote player
    let mut mock_network = app.world.resource_mut::<MockNetwork>();
    mock_network.simulate_receive(VoicePacketEvent {
        player_id: remote_id,
        audio_data: mock_network.sent_packets[0].audio_data.clone(),
        sequence_number: 1,
        timestamp: 0.0,
        channel: VoiceChannel::Global,
    });
    
    // Run receive and playback
    app.update();
    
    // Verify audio was queued for playback
    let output_device = app.world.resource::<MockAudioOutput>();
    assert!(!output_device.played_samples.is_empty(), "Voice audio should be played");
    
    // Verify UI indicators were updated
    let indicators = app.world.query::<&PlayerVoiceIndicator>()
        .iter(&app.world)
        .find(|i| i.player_id == remote_id && i.is_speaking);
    
    assert!(indicators.is_some(), "Remote player should be marked as speaking");
}
```

### Performance Tests

```rust
#[test]
fn test_voice_chat_performance() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(DiagnosticsPlugin);
    
    // Setup voice chat with max players
    setup_voice_chat_with_players(&mut app, 6);
    
    // Add performance measurement systems
    app.add_systems(Update, measure_performance);
    
    // Generate test audio for all players
    let test_audio = generate_multi_player_audio();
    app.insert_resource(MockMultiPlayerAudio { samples: test_audio });
    
    // Run system for multiple frames
    let mut cpu_usage = Vec::new();
    let mut memory_usage = Vec::new();
    
    for _ in 0..100 {
        let start = std::time::Instant::now();
        app.update();
        
        let frame_time = start.elapsed();
        cpu_usage.push(frame_time);
        
        // Measure memory usage
        let mem_usage = app.world.resource::<MemoryDiagnostics>().current_usage;
        memory_usage.push(mem_usage);
    }
    
    // Calculate average CPU and memory usage
    let avg_cpu = cpu_usage.iter().sum::<std::time::Duration>() / cpu_usage.len() as u32;
    let avg_memory = memory_usage.iter().sum::<usize>() / memory_usage.len();
    
    // Check against performance targets
    assert!(avg_cpu.as_millis() < 5, "Voice processing should use less than 5ms per frame");
    assert!(avg_memory < 10 * 1024 * 1024, "Voice chat should use less than 10MB memory");
}
```

### Network Tests

```rust
#[test]
fn test_voice_chat_bandwidth_usage() {
    // Create test app with network diagnostics
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(NetworkDiagnosticsPlugin);
    
    // Setup voice chat
    app.insert_resource(VoiceChatConfig {
        enabled: true,
        activation_mode: VoiceActivationMode::AlwaysOn,
        // ...
    });
    
    // Setup mock audio with constant speaking
    app.insert_resource(MockAudioInput {
        samples: generate_speaking_audio(),
    });
    
    // Run for multiple frames
    let network_usage = Vec::new();
    for _ in 0..100 {
        app.update();
        
        let diagnostics = app.world.resource::<NetworkDiagnostics>();
        network_usage.push(diagnostics.bytes_sent + diagnostics.bytes_received);
    }
    
    // Calculate average bandwidth
    let avg_bandwidth = network_usage.iter().sum::<usize>() / network_usage.len();
    
    // Voice chat should use reasonable bandwidth (30 KB/s maximum)
    assert!(avg_bandwidth < 30 * 1024, "Voice chat should use less than 30 KB/s bandwidth");
}
``` 
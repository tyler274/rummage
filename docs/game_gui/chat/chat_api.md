# Chat System API for Plugins

This document provides detailed information about the chat system API that plugin developers can use to integrate with Rummage's in-game communication system.

## Table of Contents

1. [Overview](#overview)
2. [Core API Components](#core-api-components)
3. [Text Chat Integration](#text-chat-integration)
4. [Voice Chat Integration](#voice-chat-integration)
5. [Events and Messages](#events-and-messages)
6. [UI Customization](#ui-customization)
7. [Examples](#examples)
8. [Best Practices](#best-practices)

## Overview

The chat API allows plugin developers to integrate with the existing chat system, enabling custom chat commands, specialized message formats, voice chat extensions, and UI customizations. This API is designed to be flexible while maintaining compatibility with the core chat functionality.

Key API features:

- Send and receive chat messages through events
- Register custom chat commands
- Add custom UI elements to the chat interface
- Hook into voice activity detection
- Add specialized message types
- Access chat history

## Core API Components

### Chat Plugin

The main entry point for plugins integrating with the chat system:

```rust
/// Plugin for chat system integration
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChatMessageEvent>()
           .add_event::<VoicePacketEvent>()
           .add_event::<ChatCommandEvent>()
           .init_resource::<ChatSettings>()
           .init_resource::<VoiceChatConfig>()
           .add_systems(Startup, setup_chat_system)
           .add_systems(Update, (
               process_chat_messages,
               handle_chat_commands,
               update_chat_ui,
               process_voice_chat,
           ));
    }
}
```

### Public Resources

Resources available for plugins to access and modify:

```rust
/// Settings for the chat system
#[derive(Resource, Clone)]
pub struct ChatSettings {
    pub text_chat_enabled: bool,
    pub voice_chat_enabled: bool,
    pub message_history_size: usize,
    pub notification_settings: NotificationSettings,
    pub accessibility_settings: ChatAccessibilitySettings,
    pub command_prefix: String,
}

/// Voice chat configuration
#[derive(Resource, Clone)]
pub struct VoiceChatConfig {
    pub enabled: bool,
    pub input_device_id: Option<String>,
    pub output_device_id: Option<String>,
    pub input_volume: f32,
    pub output_volume: f32,
    pub activation_mode: VoiceActivationMode,
    pub activation_threshold: f32,
    pub push_to_talk_key: KeyCode,
    pub noise_suppression_level: NoiseSuppressionLevel,
    pub echo_cancellation: bool,
    pub auto_gain_control: bool,
}

/// Chat message history resource
#[derive(Resource)]
pub struct ChatHistory {
    pub messages: Vec<ChatMessage>,
    pub channels: HashMap<ChatChannel, Vec<ChatMessage>>,
    pub last_read_indices: HashMap<ChatChannel, usize>,
}
```

### Public Components

Components that plugins can query and modify:

```rust
/// Chat window component
#[derive(Component)]
pub struct ChatWindow {
    pub mode: ChatMode,
    pub active_channel: ChatChannel,
    pub position: ChatPosition,
    pub is_focused: bool,
}

/// Chat message display component
#[derive(Component)]
pub struct ChatMessageDisplay {
    pub messages: Vec<ChatMessage>,
    pub scroll_position: f32,
    pub filter_settings: ChatFilterSettings,
}

/// Voice activity component
#[derive(Component)]
pub struct VoiceActivity {
    pub is_active: bool,
    pub volume_level: f32,
    pub player_id: PlayerId,
}
```

### Public Events

Events that plugins can send and receive:

```rust
/// Event for chat messages
#[derive(Event, Clone)]
pub struct ChatMessageEvent {
    pub message: ChatMessage,
    pub recipients: MessageRecipients,
}

/// Event for voice packets
#[derive(Event)]
pub struct VoicePacketEvent {
    pub player_id: PlayerId,
    pub audio_data: Vec<u8>,
    pub sequence_number: u32,
    pub timestamp: f64,
    pub channel: VoiceChannel,
}

/// Event for chat commands
#[derive(Event)]
pub struct ChatCommandEvent {
    pub command: String,
    pub args: Vec<String>,
    pub sender_id: PlayerId,
    pub channel: ChatChannel,
}
```

## Text Chat Integration

### Sending Messages

Plugins can send messages to the chat system:

```rust
/// Send a chat message from a plugin
pub fn send_chat_message(
    message_text: &str,
    sender_name: &str,
    message_type: MessageType,
    channel: ChatChannel,
    message_events: &mut EventWriter<ChatMessageEvent>,
) {
    let message = ChatMessage {
        sender_id: None,  // None indicates system/plugin sender
        sender_name: sender_name.to_string(),
        content: message_text.to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64(),
        channel,
        message_type,
        formatting: None,
    };
    
    message_events.send(ChatMessageEvent {
        message,
        recipients: MessageRecipients::All,
    });
}
```

### Reading Messages

Plugins can read messages from the chat system:

```rust
/// System to read chat messages in a plugin
pub fn read_chat_messages(
    mut message_events: EventReader<ChatMessageEvent>,
    mut plugin_state: ResMut<MyPluginState>,
) {
    for event in message_events.read() {
        // Process incoming messages
        plugin_state.process_message(&event.message);
        
        // Log or analyze messages
        if event.message.content.contains(&plugin_state.keyword) {
            // Do something with this message
        }
    }
}
```

### Registering Custom Commands

Plugins can register custom chat commands:

```rust
/// Register a custom chat command
pub fn register_chat_command(
    command: &str,
    description: &str,
    mut commands: ResMut<ChatCommandRegistry>,
) {
    commands.register(
        command.to_string(),
        ChatCommandInfo {
            description: description.to_string(),
            permission_level: PermissionLevel::User,
            handler: ChatCommandHandler::Plugin("my_plugin".to_string()),
        },
    );
}

/// System to handle custom chat commands
pub fn handle_custom_commands(
    mut command_events: EventReader<ChatCommandEvent>,
    mut message_events: EventWriter<ChatMessageEvent>,
    // Plugin-specific resources and access
) {
    for event in command_events.read() {
        if event.command == "my_custom_command" {
            // Handle the custom command
            // ...
            
            // Send response
            send_chat_message(
                "Command processed successfully!",
                "MyPlugin",
                MessageType::System,
                event.channel,
                &mut message_events,
            );
        }
    }
}
```

## Voice Chat Integration

### Voice Activity Detection

Plugins can hook into voice activity detection:

```rust
/// System to monitor voice activity
pub fn monitor_voice_activity(
    voice_activity: Query<&VoiceActivity>,
    mut plugin_state: ResMut<MyPluginVoiceState>,
) {
    for activity in &voice_activity {
        if activity.is_active {
            // Player is speaking
            plugin_state.player_speaking(activity.player_id, activity.volume_level);
        } else if plugin_state.was_speaking(activity.player_id) {
            // Player stopped speaking
            plugin_state.player_stopped_speaking(activity.player_id);
        }
    }
}
```

### Voice Processing

Plugins can process voice audio:

```rust
/// System to process voice packets
pub fn process_voice_packets(
    mut voice_events: EventReader<VoicePacketEvent>,
    mut processed_voice_events: EventWriter<VoicePacketEvent>,
    plugin_voice_processor: Res<MyVoiceProcessor>,
) {
    for event in voice_events.read() {
        // Get the audio data
        let audio_data = &event.audio_data;
        
        // Process the audio (e.g., apply effects, filter, etc.)
        let processed_data = plugin_voice_processor.process_audio(audio_data);
        
        // Create a new event with processed data
        processed_voice_events.send(VoicePacketEvent {
            player_id: event.player_id,
            audio_data: processed_data,
            sequence_number: event.sequence_number,
            timestamp: event.timestamp,
            channel: event.channel,
        });
    }
}
```

## Events and Messages

### Custom Message Types

Plugins can define custom message types:

```rust
/// Define custom message types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomMessageType {
    Achievement,
    Tip,
    Warning,
    Debug,
}

/// Convert custom message type to standard type
impl From<CustomMessageType> for MessageType {
    fn from(custom_type: CustomMessageType) -> Self {
        match custom_type {
            CustomMessageType::Achievement => MessageType::System,
            CustomMessageType::Tip => MessageType::System,
            CustomMessageType::Warning => MessageType::Error,
            CustomMessageType::Debug => MessageType::System,
        }
    }
}

/// Send a custom message
pub fn send_custom_message(
    text: &str,
    custom_type: CustomMessageType,
    mut message_events: EventWriter<ChatMessageEvent>,
) {
    let message_type: MessageType = custom_type.into();
    
    // Create message with custom formatting based on type
    let formatting = match custom_type {
        CustomMessageType::Achievement => Some(MessageFormatting {
            color: Some(Color::GOLD),
            icon: Some("achievement".to_string()),
            ..Default::default()
        }),
        CustomMessageType::Tip => Some(MessageFormatting {
            color: Some(Color::BLUE),
            icon: Some("tip".to_string()),
            ..Default::default()
        }),
        // ...other types
        _ => None,
    };
    
    let message = ChatMessage {
        sender_id: None,
        sender_name: match custom_type {
            CustomMessageType::Achievement => "Achievement Unlocked".to_string(),
            CustomMessageType::Tip => "Tip".to_string(),
            CustomMessageType::Warning => "Warning".to_string(),
            CustomMessageType::Debug => "Debug".to_string(),
        },
        content: text.to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64(),
        channel: ChatChannel::System,
        message_type,
        formatting,
    };
    
    message_events.send(ChatMessageEvent {
        message,
        recipients: MessageRecipients::All,
    });
}
```

### Event Interception

Plugins can intercept and modify chat events:

```rust
/// System to intercept chat messages
pub fn intercept_chat_messages(
    mut reader: EventReader<ChatMessageEvent>,
    mut writer: EventWriter<ChatMessageEvent>,
    plugin_config: Res<MyPluginConfig>,
) {
    for event in reader.read() {
        // Make a mutable copy of the event
        let mut modified_event = event.clone();
        
        // Apply modifications based on plugin rules
        if plugin_config.should_filter_profanity {
            modified_event.message.content = filter_profanity(&modified_event.message.content);
        }
        
        if plugin_config.should_translate && event.message.content.starts_with("!translate") {
            // Translate message to selected language
            modified_event.message.content = translate_message(
                &modified_event.message.content,
                plugin_config.target_language.clone(),
            );
        }
        
        // Forward the modified event
        writer.send(modified_event);
    }
}
```

## UI Customization

### Adding UI Elements

Plugins can add custom UI elements to the chat interface:

```rust
/// System to add custom UI elements to chat
pub fn setup_custom_chat_ui(
    mut commands: Commands,
    chat_window_query: Query<Entity, With<ChatWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(chat_entity) = chat_window_query.get_single() {
        // Add a custom button to the chat window
        commands.entity(chat_entity).with_children(|parent| {
            parent.spawn((
                CustomChatButton::TranslateToggle,
                ButtonBundle {
                    style: Style {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        // ...other style properties
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.3, 0.5, 0.8, 0.6)),
                    ..default()
                },
            ))
            .with_children(|button| {
                button.spawn(Text2d {
                    text: "🌐".into(),
                    font: asset_server.load("fonts/NotoEmoji-Regular.ttf"),
                    font_size: 16.0,
                    // ...other text properties
                });
            });
        });
    }
}

/// System to handle custom UI interactions
pub fn handle_custom_chat_ui(
    interaction_query: Query<
        (&Interaction, &CustomChatButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut plugin_config: ResMut<MyPluginConfig>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                CustomChatButton::TranslateToggle => {
                    // Toggle translation feature
                    plugin_config.should_translate = !plugin_config.should_translate;
                },
                // Handle other custom buttons
                // ...
            }
        }
    }
}
```

### Styling Messages

Plugins can define custom message styles:

```rust
/// Define custom message style
pub struct CustomMessageStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub border_color: Option<Color>,
    pub icon: Option<String>,
    pub font_style: FontStyle,
}

/// Register custom message styles
pub fn register_custom_styles(
    mut style_registry: ResMut<ChatStyleRegistry>,
) {
    // Register achievement style
    style_registry.register(
        "achievement",
        CustomMessageStyle {
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9),
            text_color: Color::rgb(1.0, 0.84, 0.0),  // Gold
            border_color: Some(Color::rgb(0.8, 0.7, 0.0)),
            icon: Some("trophy".to_string()),
            font_style: FontStyle::Bold,
        },
    );
    
    // Register other styles
    // ...
}
```

## Examples

### Basic Chat Command Plugin

A simple plugin that adds a dice rolling command:

```rust
/// Dice roller plugin
pub struct DiceRollerPlugin;

impl Plugin for DiceRollerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiceRollerConfig>()
           .add_systems(Startup, register_dice_commands)
           .add_systems(Update, handle_dice_commands);
    }
}

/// Configuration for dice roller
#[derive(Resource, Default)]
struct DiceRollerConfig {
    max_dice: u32,
    max_sides: u32,
}

/// Register dice rolling commands
fn register_dice_commands(
    mut command_registry: ResMut<ChatCommandRegistry>,
) {
    command_registry.register(
        "roll".to_string(),
        ChatCommandInfo {
            description: "Roll dice. Usage: /roll XdY".to_string(),
            permission_level: PermissionLevel::User,
            handler: ChatCommandHandler::Plugin("dice_roller".to_string()),
        },
    );
}

/// Handle dice rolling commands
fn handle_dice_commands(
    mut command_events: EventReader<ChatCommandEvent>,
    mut message_events: EventWriter<ChatMessageEvent>,
    config: Res<DiceRollerConfig>,
) {
    for event in command_events.read() {
        if event.command == "roll" {
            // Parse arguments (e.g., "2d6")
            if let Some(arg) = event.args.first() {
                if let Some((count_str, sides_str)) = arg.split_once('d') {
                    // Parse dice count and sides
                    if let (Ok(count), Ok(sides)) = (count_str.parse::<u32>(), sides_str.parse::<u32>()) {
                        // Validate against config limits
                        if count > 0 && count <= config.max_dice && sides > 0 && sides <= config.max_sides {
                            // Roll the dice
                            let mut rng = rand::thread_rng();
                            let rolls: Vec<u32> = (0..count)
                                .map(|_| rng.gen_range(1..=sides))
                                .collect();
                            
                            let total: u32 = rolls.iter().sum();
                            
                            // Format the result
                            let rolls_str = rolls
                                .iter()
                                .map(|r| r.to_string())
                                .collect::<Vec<_>>()
                                .join(", ");
                            
                            let result_message = format!(
                                "{} rolled {}d{}: [{}] = {}",
                                event.sender_id, count, sides, rolls_str, total
                            );
                            
                            // Send the message
                            send_chat_message(
                                &result_message,
                                "Dice Roller",
                                MessageType::System,
                                event.channel,
                                &mut message_events,
                            );
                        } else {
                            // Invalid dice parameters
                            send_chat_message(
                                &format!(
                                    "Invalid dice parameters. Maximum {} dice with {} sides.",
                                    config.max_dice, config.max_sides
                                ),
                                "Dice Roller",
                                MessageType::Error,
                                event.channel,
                                &mut message_events,
                            );
                        }
                    }
                }
            }
        }
    }
}
```

### Voice Effect Plugin

A plugin that adds voice effects:

```rust
/// Voice effect plugin
pub struct VoiceEffectPlugin;

impl Plugin for VoiceEffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoiceEffectSettings>()
           .add_systems(Startup, setup_voice_effect_ui)
           .add_systems(Update, (
               process_voice_effects,
               update_effect_settings,
           ));
    }
}

/// Voice effect settings
#[derive(Resource)]
struct VoiceEffectSettings {
    active_effect: Option<VoiceEffectType>,
    pitch_shift: f32,
    reverb_amount: f32,
    distortion: f32,
}

/// Voice effect types
enum VoiceEffectType {
    None,
    HighPitch,
    LowPitch,
    Robot,
    Echo,
    Custom,
}

/// System to process voice with effects
fn process_voice_effects(
    mut voice_events: EventReader<VoicePacketEvent>,
    mut processed_voice_events: EventWriter<VoicePacketEvent>,
    settings: Res<VoiceEffectSettings>,
    local_player: Res<LocalPlayer>,
) {
    // Only process local player's voice
    for event in voice_events.read() {
        if event.player_id != local_player.id {
            // Forward other players' packets unchanged
            processed_voice_events.send(event.clone());
            continue;
        }
        
        // Skip processing if no effect is active
        if settings.active_effect == Some(VoiceEffectType::None) || settings.active_effect.is_none() {
            processed_voice_events.send(event.clone());
            continue;
        }
        
        // Decompress audio data
        let decompressed = decompress_audio_data(&event.audio_data);
        
        // Apply selected effect
        let processed_audio = match settings.active_effect {
            Some(VoiceEffectType::HighPitch) => apply_pitch_shift(&decompressed, 1.5),
            Some(VoiceEffectType::LowPitch) => apply_pitch_shift(&decompressed, 0.7),
            Some(VoiceEffectType::Robot) => apply_robot_effect(&decompressed),
            Some(VoiceEffectType::Echo) => apply_echo_effect(&decompressed, 0.5, 0.3),
            Some(VoiceEffectType::Custom) => apply_custom_effect(
                &decompressed, 
                settings.pitch_shift, 
                settings.reverb_amount, 
                settings.distortion
            ),
            _ => decompressed,
        };
        
        // Compress processed audio
        let compressed = compress_audio_data(&processed_audio);
        
        // Send modified packet
        processed_voice_events.send(VoicePacketEvent {
            player_id: event.player_id,
            audio_data: compressed,
            sequence_number: event.sequence_number,
            timestamp: event.timestamp,
            channel: event.channel,
        });
    }
}
```

## Best Practices

### Performance Considerations

- Minimize processing in chat message handling systems
- Use efficient algorithms for text processing
- Cache results of expensive operations
- For voice processing, limit complexity when multiple players are speaking
- Clean up resources when plugins are disabled

### Compatibility

- Follow the chat message formatting conventions
- Don't override default commands without good reason
- Provide fallback behavior when your plugin's features are disabled
- Test interactions with other popular plugins

### User Experience

- Make plugin features discoverable through chat help commands
- Provide clear feedback for user actions
- Allow users to disable or customize plugin features
- Don't spam the chat with too many messages
- Respect user privacy settings with voice processing

### Error Handling

- Validate all user input before processing
- Provide helpful error messages for invalid commands
- Catch and log exceptions rather than crashing
- Gracefully handle missing resources or components
- Have fallback behavior when network operations fail 
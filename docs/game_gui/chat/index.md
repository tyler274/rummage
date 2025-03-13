# In-Game Chat System

This document details the in-game chat system in Rummage, providing communication capabilities for players during Commander format games.

## Table of Contents

1. [Overview](#overview)
2. [Chat System Components](#chat-system-components)
3. [Integration With Game UI](#integration-with-game-ui)
4. [Accessibility Features](#accessibility-features)
5. [Implementation Details](#implementation-details)
6. [Testing](#testing)
7. [Related Documentation](#related-documentation)

## Overview

The in-game chat system provides multiple communication channels for players during gameplay:

- **Text Chat**: Traditional text-based communication
- **Voice Chat**: Real-time audio communication
- **Game Event Messages**: Automated messages about game events
- **Emotes and Reactions**: Pre-defined expressions and reactions

The chat system is designed to be non-intrusive while remaining easily accessible during gameplay, enhancing the social experience of Commander format games.

## Chat System Components

The chat system consists of several integrated components:

### Text Chat

The text chat component allows players to type and send messages to others in the game. Features include:

- **Chat Channels**: Public, private, and team channels
- **Message Formatting**: Support for basic text formatting
- **Command System**: Chat commands for game actions
- **Message History**: Accessible chat history with search capabilities

[Detailed Text Chat Documentation](text_chat.md)

### Voice Chat

The voice chat component enables real-time audio communication between players:

- **Push-to-Talk**: Configurable key binding for activating microphone
- **Voice Activity Detection**: Optional automatic activation based on speech
- **Player Indicators**: Visual cues showing who is speaking
- **Individual Volume Controls**: Adjust volume for specific players

[Detailed Voice Chat Documentation](voice_chat.md)

### Game Event Messages

Automated messages about game actions and events:

- **Stackable Notifications**: Collapsible event messages
- **Filtering Options**: Configure which events generate messages
- **Verbosity Settings**: Adjust level of detail in event messages
- **Highlighting**: Color coding for important events

### Emotes and Reactions

Quick non-verbal communication options:

- **Contextual Emotes**: Reactions appropriate to game context
- **Emote Wheel**: Quick access to common emotes
- **Custom Emotes**: Limited customization options
- **Cooldown System**: Prevents emote spam

## Integration With Game UI

The chat system integrates seamlessly with the game UI:

### Chat Window Modes

The chat window can appear in multiple states:

- **Expanded View**: Full chat interface with history and channels
- **Minimized View**: Condensed view showing recent messages
- **Hidden**: Completely hidden with notification indicators for new messages
- **Pop-out**: Detachable window for multi-monitor setups

### Positioning

The chat interface can be positioned in different areas:

- **Default**: Bottom-left corner of the screen
- **Customizable**: User can reposition within constraints
- **Contextual**: Automatic repositioning based on game state
- **Size Adjustable**: Resizable chat window

### Visual Integration

Visual elements that tie the chat system to the game:

- **Player Color Coding**: Message colors match player identities
- **Thematic Styling**: Chat UI follows game's visual language
- **Transition Effects**: Smooth animations for state changes
- **Focus Management**: Proper keyboard focus handling

## Accessibility Features

The chat system includes several accessibility features:

- **Text-to-Speech**: Optional reading of incoming messages
- **Speech-to-Text**: Voice transcription for voice chat
- **High Contrast Mode**: Improved readability options
- **Customizable Text Size**: Adjustable font sizes
- **Keyboard Navigation**: Complete keyboard control
- **Alternative Communication**: Pre-defined phrases for quick communication
- **Message Timing**: Configurable message display duration

## Implementation Details

The chat system is implemented using Bevy's ECS architecture:

### Components

```rust
/// Component for the chat window
#[derive(Component)]
struct ChatWindow {
    mode: ChatMode,
    active_channel: ChatChannel,
    position: ChatPosition,
    is_focused: bool,
}

/// Component for input field
#[derive(Component)]
struct ChatInput {
    text: String,
    cursor_position: usize,
    selection_range: Option<(usize, usize)>,
}

/// Component for chat message display
#[derive(Component)]
struct ChatMessageDisplay {
    messages: Vec<ChatMessage>,
    scroll_position: f32,
    filter_settings: ChatFilterSettings,
}

/// Component for voice activity
#[derive(Component)]
struct VoiceActivity {
    is_active: bool,
    volume_level: f32,
    player_id: PlayerId,
}

/// Resource for chat settings
#[derive(Resource)]
struct ChatSettings {
    text_chat_enabled: bool,
    voice_chat_enabled: bool,
    message_history_size: usize,
    notification_settings: NotificationSettings,
    accessibility_settings: ChatAccessibilitySettings,
}
```

### Systems

```rust
/// System to handle incoming chat messages
fn handle_chat_messages(
    mut messages_query: Query<&mut ChatMessageDisplay>,
    chat_events: EventReader<ChatMessageEvent>,
    settings: Res<ChatSettings>,
) {
    // Implementation
}

/// System to handle voice chat
fn process_voice_chat(
    mut voice_activity_query: Query<(&mut VoiceActivity, &PlayerId)>,
    audio_input: Res<AudioInputBuffer>,
    settings: Res<ChatSettings>,
) {
    // Implementation
}

/// System to update chat UI
fn update_chat_ui(
    mut chat_window_query: Query<(&mut ChatWindow, &mut Node, &Children)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Implementation
}
```

### Chat Window Setup

```rust
/// Setup the chat window
fn setup_chat_window(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chat_settings: Res<ChatSettings>,
) {
    // Main chat container
    commands
        .spawn((
            ChatWindow {
                mode: ChatMode::Minimized,
                active_channel: ChatChannel::Global,
                position: ChatPosition::BottomLeft,
                is_focused: false,
            },
            Node {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
            BorderColor(Color::rgb(0.3, 0.3, 0.3)),
            Outline::new(Val::Px(1.0)),
            AppLayer::GameUI.layer(),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            // Chat header
            setup_chat_header(parent, &asset_server);
            
            // Message display area
            setup_message_display(parent, &asset_server);
            
            // Chat input area
            setup_chat_input(parent, &asset_server);
            
            // Voice chat indicators
            if chat_settings.voice_chat_enabled {
                setup_voice_indicators(parent, &asset_server);
            }
        });
}
```

## Testing

The chat system requires thorough testing to ensure reliability and performance:

### Unit Tests

- Test message processing
- Verify channel functionality
- Test input handling
- Validate filtering systems

### Integration Tests

- Test chat integration with game events
- Verify voice chat synchronization
- Test accessibility features
- Validate UI responsiveness

### Performance Tests

- Test with high message volume
- Measure voice chat latency
- Verify memory usage with large chat history
- Test network bandwidth usage

### Usability Tests

- Validate readability
- Test keyboard navigation
- Verify mobile touch interactions
- Test with screen readers

## Related Documentation

- [Text Chat Details](text_chat.md)
- [Voice Chat Details](voice_chat.md)
- [Chat API for Plugins](chat_api.md)
- [Network Protocol for Chat](network_protocol.md) 
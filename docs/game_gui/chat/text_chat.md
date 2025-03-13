# In-Game Text Chat System

This document provides detailed information about the text chat component of Rummage's in-game communication system.

## Table of Contents

1. [Overview](#overview)
2. [UI Components](#ui-components)
3. [Text Chat Features](#text-chat-features)
4. [Message Types](#message-types)
5. [Chat Commands](#chat-commands)
6. [Implementation Details](#implementation-details)
7. [Testing](#testing)

## Overview

The text chat system provides a flexible and intuitive interface for players to communicate during Commander games. It balances ease of use with powerful features, allowing for both casual conversation and game-specific communication.

Key design principles:

- **Non-Intrusive**: Minimizes screen space usage while maintaining readability
- **Context-Aware**: Adapts to game state and player actions
- **Flexible**: Supports various communication needs and styles
- **Integrated**: Closely tied to game mechanics and events

## UI Components

The text chat interface consists of several key components:

### Chat Window

The main container for all chat-related UI elements:

```
┌────────────────────────────────────────────┐
│ [Global] [Team] [Spectators] [Settings] [X]│
├────────────────────────────────────────────┤
│ [System] Game started                      │
│ Player1: Hello everyone                    │
│ Player2: Good luck & have fun!             │
│ [Event] Player3 casts Lightning Bolt       │
│                                            │
│                                            │
│                                            │
│                                            │
├────────────────────────────────────────────┤
│ Type message...                 [Send] [📢]│
└────────────────────────────────────────────┘
```

- **Header**: Channel tabs, settings button, close/minimize button
- **Message Display**: Scrollable area showing chat history
- **Input Field**: Text entry area with send button and voice chat toggle

### Notification Badge

When the chat is minimized, notifications appear showing new messages:

```
┌───────┐
│ Chat 3│
└───────┘
```

The badge shows the number of unread messages and changes color based on message importance.

### Message Bubbles

Individual messages are displayed in styled bubbles with context information:

```
┌─────────────────────────────────────────┐
│ Player1 (10:42):                        │
│ Does anyone have a response to this?    │
└─────────────────────────────────────────┘
```

Each message includes:
- Sender name with optional player color
- Timestamp
- Message content with optional formatting
- Message type indicator (system, event, etc.)

## Text Chat Features

### Chat Channels

The system supports multiple communication channels:

- **Global**: Messages visible to all players in the game
- **Team**: Messages visible only to teammates (for team games)
- **Private**: Direct messages to specific players
- **Spectator**: Communication with non-playing observers
- **System**: Game information and event messages
- **Event Log**: Detailed game action history

Users can switch between channels using tabs or chat commands.

### Message Formatting

The chat supports basic text formatting options:

- **Emphasis**: *italic* and **bold** text
- **Card References**: [[Card Name]] auto-links to card information
- **Links**: Clickable URLs with preview tooltips
- **Emoji**: Standard emoji support with game-specific additions
- **Color Coding**: Optional colored text based on message type or sender

### Message Filtering

Users can filter the chat display based on various criteria:

- **Channel Filters**: Show/hide messages from specific channels
- **Type Filters**: Show/hide system messages, events, etc.
- **Player Filters**: Focus on messages from specific players
- **Keyword Filters**: Highlight or hide messages containing specific terms

### Chat History

The chat system maintains a searchable history of messages:

- **Scrollback**: Browse previous messages within the current session
- **Search**: Find messages containing specific text or from specific players
- **Session Log**: Option to save chat history to a file
- **Persistence**: Optional storage of chat logs between sessions

## Message Types

The chat system supports several types of messages:

### Player Messages

Standard text messages sent by players:

- **Chat Messages**: Normal conversation text
- **Announcements**: Important player notifications
- **Responses**: Contextual replies to other messages or game events

### System Messages

Automated messages from the game system:

- **Game Events**: Card plays, battlefield changes, life total updates
- **Phase Updates**: Turn phase transitions, priority changes
- **Timer Notifications**: Round time, turn time warnings
- **Game Status**: Win conditions, player eliminations, etc.

### Command Messages

Special messages that trigger game actions or chat functions:

- **Chat Commands**: Messages starting with / that invoke special functions
- **Quick Commands**: Pre-defined messages accessible through hotkeys
- **Emote Commands**: Text triggers for emote animations

## Chat Commands

The chat system includes command functionality for quick actions:

### Core Commands

Essential commands available to all players:

- **/help**: Display available commands
- **/msg [player] [text]**: Send private message
- **/clear**: Clear chat history
- **/mute [player]**: Mute specified player
- **/unmute [player]**: Unmute specified player

### Game Commands

Commands that provide game information:

- **/life [player]**: Show player's current life total
- **/card [card name]**: Display card information
- **/hand**: Show number of cards in each player's hand
- **/graveyard [player]**: List cards in player's graveyard

### Social Commands

Commands for social interaction:

- **/emote [emote name]**: Display an emote
- **/roll [X]d[Y]**: Roll X Y-sided dice
- **/flip**: Flip a coin
- **/timer [seconds]**: Set a countdown timer

## Implementation Details

The text chat is implemented using Bevy's ECS architecture:

### Data Structures

```rust
/// Represents a single chat message
#[derive(Clone, Debug)]
struct ChatMessage {
    sender_id: Option<PlayerId>,
    sender_name: String,
    content: String,
    timestamp: f64,
    channel: ChatChannel,
    message_type: MessageType,
    formatting: Option<MessageFormatting>,
}

/// Defines available chat channels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ChatChannel {
    Global,
    Team,
    Private(PlayerId),
    Spectator,
    System,
    EventLog,
}

/// Defines message types for styling and filtering
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MessageType {
    Normal,
    System,
    Event,
    Error,
    Whisper,
    Command,
    Emote,
}

/// Event for chat message transmission
#[derive(Event)]
struct ChatMessageEvent {
    message: ChatMessage,
    recipients: MessageRecipients,
}

/// Defines message recipients
#[derive(Clone, Debug)]
enum MessageRecipients {
    All,
    Team(TeamId),
    Player(PlayerId),
    Spectators,
}
```

### Components

```rust
/// Component for the chat message display area
#[derive(Component)]
struct ChatMessageArea {
    visible_channels: HashSet<ChatChannel>,
    filter_settings: ChatFilterSettings,
    max_messages: usize,
    scroll_position: f32,
}

/// Component for the chat input field
#[derive(Component)]
struct ChatInputField {
    text: String,
    cursor_position: usize,
    history: Vec<String>,
    history_position: Option<usize>,
    target_channel: ChatChannel,
}

/// Component for individual message entities
#[derive(Component)]
struct ChatMessageEntity {
    message: ChatMessage,
    is_read: bool,
    animation_state: MessageAnimationState,
}
```

### Message Rendering System

```rust
/// System to render chat messages
fn render_chat_messages(
    mut commands: Commands,
    mut message_area_query: Query<(&mut ChatMessageArea, &Children)>,
    message_query: Query<(Entity, &ChatMessageEntity)>,
    message_events: EventReader<ChatMessageEvent>,
    chat_settings: Res<ChatSettings>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Process new messages
    for event in message_events.read() {
        for (mut message_area, children) in &mut message_area_query {
            // Check if message should be displayed in this area
            if !message_area.visible_channels.contains(&event.message.channel) {
                continue;
            }
            
            // Apply filters
            if !passes_filters(&event.message, &message_area.filter_settings) {
                continue;
            }
            
            // Create new message entity
            let message_entity = spawn_message_entity(
                &mut commands, 
                &event.message, 
                &asset_server, 
                &chat_settings
            );
            
            // Add to message area
            commands.entity(message_area_entity).add_child(message_entity);
            
            // Limit number of messages
            if children.len() > message_area.max_messages {
                // Remove oldest message
                if let Some(oldest) = children.first() {
                    commands.entity(*oldest).despawn_recursive();
                }
            }
            
            // Auto-scroll to new message
            message_area.scroll_position = 1.0;
        }
    }
    
    // Update message animations
    for (entity, message) in &message_query {
        // Update animation state based on time
        // ...
    }
}
```

### Chat Input System

```rust
/// System to handle chat input
fn handle_chat_input(
    mut commands: Commands,
    mut input_query: Query<&mut ChatInputField>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chat_events: EventWriter<ChatMessageEvent>,
    player_query: Query<(&Player, &PlayerId)>,
    time: Res<Time>,
) {
    for mut input_field in &mut input_query {
        // Check for Enter key to send message
        if keyboard_input.just_pressed(KeyCode::Return) {
            if !input_field.text.is_empty() {
                // Create message from input
                let message = create_message_from_input(
                    &input_field.text,
                    &player_query,
                    input_field.target_channel,
                    time.elapsed_seconds(),
                );
                
                // Handle commands
                if input_field.text.starts_with('/') {
                    process_command(&input_field.text, &mut chat_events);
                } else {
                    // Send regular message
                    chat_events.send(ChatMessageEvent {
                        message,
                        recipients: get_recipients_for_channel(input_field.target_channel),
                    });
                }
                
                // Add to history and clear input
                input_field.history.push(input_field.text.clone());
                input_field.text.clear();
                input_field.cursor_position = 0;
                input_field.history_position = None;
            }
        }
        
        // Handle Up/Down for history navigation
        if keyboard_input.just_pressed(KeyCode::Up) {
            navigate_history_up(&mut input_field);
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            navigate_history_down(&mut input_field);
        }
        
        // Handle Tab for channel switching
        if keyboard_input.just_pressed(KeyCode::Tab) {
            cycle_chat_channel(&mut input_field);
        }
    }
}
```

## Testing

The text chat component requires thorough testing:

### Unit Tests

```rust
#[test]
fn test_message_filtering() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_event::<ChatMessageEvent>()
       .add_systems(Update, chat_systems::filter_messages);
    
    // Setup test state
    let filter_settings = ChatFilterSettings {
        show_system_messages: false,
        show_event_messages: true,
        // ...
    };
    
    app.world.insert_resource(filter_settings);
    
    // Create test messages
    let system_message = ChatMessage {
        message_type: MessageType::System,
        // ...
    };
    
    let event_message = ChatMessage {
        message_type: MessageType::Event,
        // ...
    };
    
    // Send test messages
    app.world.send_event(ChatMessageEvent {
        message: system_message,
        recipients: MessageRecipients::All,
    });
    
    app.world.send_event(ChatMessageEvent {
        message: event_message,
        recipients: MessageRecipients::All,
    });
    
    // Run systems
    app.update();
    
    // Verify filtering
    let visible_messages = app.world.query::<&ChatMessageEntity>().iter(&app.world).collect::<Vec<_>>();
    
    assert_eq!(visible_messages.len(), 1, "Only event message should be visible");
    assert_eq!(visible_messages[0].message.message_type, MessageType::Event);
}
```

### Integration Tests

```rust
#[test]
fn test_chat_command_processing() {
    // Create test app with necessary plugins
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_systems(Update, (
           chat_systems::handle_chat_input,
           chat_systems::process_commands,
       ));
    
    // Setup test game with players
    setup_test_game(&mut app, 2);
    
    // Get chat input entity
    let input_entity = app.world.query_filtered::<Entity, With<ChatInputField>>().single(&app.world);
    
    // Simulate typing a command
    let mut input_field = app.world.get_mut::<ChatInputField>(input_entity).unwrap();
    input_field.text = "/roll 2d6".to_string();
    
    // Simulate Enter key press
    app.world.send_event(KeyboardInput {
        key: KeyCode::Return,
        state: ButtonState::Pressed,
    });
    
    // Run systems
    app.update();
    
    // Verify command was processed
    let messages = app.world.query::<&ChatMessageEntity>().iter(&app.world).collect::<Vec<_>>();
    
    // Should have a system message with dice roll results
    assert!(messages.iter().any(|m| 
        m.message.message_type == MessageType::System && 
        m.message.content.contains("rolled 2d6")
    ), "Dice roll command should create system message");
}
```

### Performance Tests

```rust
#[test]
fn test_chat_performance_with_many_messages() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(DiagnosticsPlugin);
    
    // Setup chat window
    let chat_entity = app.world.spawn((
        ChatWindow { /* ... */ },
        ChatMessageArea {
            max_messages: 100,
            // ...
        },
        // ...
    )).id();
    
    // Generate many test messages
    let messages = (0..500).map(|i| ChatMessage {
        content: format!("Test message {}", i),
        // ...
    }).collect::<Vec<_>>();
    
    // Measure performance while adding messages
    let mut frame_times = Vec::new();
    for message in messages {
        app.world.send_event(ChatMessageEvent {
            message,
            recipients: MessageRecipients::All,
        });
        
        let start = std::time::Instant::now();
        app.update();
        frame_times.push(start.elapsed());
    }
    
    // Calculate average frame time
    let avg_frame_time = frame_times.iter().sum::<std::time::Duration>() / frame_times.len() as u32;
    
    // Ensure performance remains acceptable
    assert!(avg_frame_time.as_millis() < 16, "Chat should maintain 60+ FPS with many messages");
}
``` 
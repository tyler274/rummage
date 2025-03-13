# Lobby Chat System UI

This document details the chat system UI used in multiplayer lobbies, which allows players to communicate before starting a Commander game. The chat system is crucial for discussing deck power levels, house rules, and general coordination.

## Table of Contents

1. [Overview](#overview)
2. [UI Components](#ui-components)
3. [Message Types](#message-types)
4. [Features](#features)
5. [Implementation](#implementation)

## Overview

The chat system occupies the central panel of the lobby detail screen, providing a real-time communication channel between all players in the lobby. It supports text messages, system notifications, and special formatting for enhanced communication.

```
┌───────────────────────────────────┐
│          Chat Panel               │
├───────────────────────────────────┤
│                                   │
│  [System] Player2 has joined      │
│                                   │
│  Player1: Hello everyone!         │
│                                   │
│  Player2: Hey, what power level   │
│  are we playing at?               │
│                                   │
│  Player1: Casual, around 6-7      │
│                                   │
│  [System] Player3 has joined      │
│                                   │
│  Player3: I have a deck that      │
│  should work for that             │
│                                   │
│                                   │
├───────────────────────────────────┤
│ ┌─────────────────────────┐  ┌──┐ │
│ │ Type a message...       │  │▶ │ │
│ └─────────────────────────┘  └──┘ │
└───────────────────────────────────┘
```

## UI Components

The chat UI consists of several key components:

### Message Display Area

- Scrollable container for message history
- Message bubbles with sender identification
- System messages with distinctive styling
- Timestamp indicators
- Auto-scrolling to newest messages

### Input Area

- Text input field
- Send button
- Character counter
- Emoji selector

### Additional Controls

- Chat options button (opens chat settings)
- Notification indicators for new messages
- Mentions highlighting

## Message Types

The chat system supports various message types:

```rust
/// Types of messages in the lobby chat
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageType {
    /// Regular player chat message
    Chat,
    /// System notification
    System,
    /// Direct message to specific player(s)
    DirectMessage,
    /// Game announcement (like deck selection)
    Announcement,
    /// Error message
    Error,
}

/// Chat message component
#[derive(Component, Clone, Debug)]
pub struct ChatMessage {
    /// Unique identifier
    pub id: String,
    /// Sender name (if applicable)
    pub sender: Option<String>,
    /// Message content
    pub content: String,
    /// Type of message
    pub message_type: MessageType,
    /// Timestamp when message was sent
    pub timestamp: f64,
    /// Players mentioned in the message
    pub mentions: Vec<String>,
}
```

## Features

### Real-time Updates

The chat system provides real-time message delivery with visual indicators:

```rust
/// System to add new messages to the chat
fn add_chat_message(
    mut commands: Commands,
    mut message_events: EventReader<NewChatMessageEvent>,
    mut chat_state: ResMut<ChatState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for event in message_events.read() {
        // Create the new message entity
        let message_entity = commands
            .spawn(ChatMessage {
                id: generate_uuid(),
                sender: event.sender.clone(),
                content: event.content.clone(),
                message_type: event.message_type.clone(),
                timestamp: time.elapsed_seconds(),
                mentions: find_mentions(&event.content),
            })
            .id();
            
        // Add to message history
        chat_state.messages.push(message_entity);
        
        // Trim history if too long
        if chat_state.messages.len() > MAX_CHAT_HISTORY {
            if let Some(old_message) = chat_state.messages.pop_front() {
                commands.entity(old_message).despawn();
            }
        }
        
        // Play notification sound if appropriate
        if needs_notification(&event.message_type, &event.mentions) {
            commands.spawn(AudioSource {
                source: asset_server.load("sounds/chat_notification.ogg"),
                ..default()
            });
        }
    }
}
```

### Message Formatting

Players can use basic formatting in their messages:

- **Bold text**: Using `**text**` syntax
- **Italics**: Using `*text*` syntax
- **Code blocks**: Using ```code``` syntax
- **Links**: URLs are automatically detected and made clickable

```rust
/// Process message formatting
fn process_message_formatting(
    mut message_query: Query<(&mut Text, &ChatMessage), Added<ChatMessage>>,
) {
    for (mut text, message) in &mut message_query {
        let formatted_content = format_message_content(&message.content);
        
        // Apply formatting to text component
        text.sections[0].value = formatted_content;
        
        // Apply styling based on message type
        match message.message_type {
            MessageType::System => {
                text.sections[0].style.color = SYSTEM_MESSAGE_COLOR;
            }
            MessageType::Error => {
                text.sections[0].style.color = ERROR_MESSAGE_COLOR;
            }
            MessageType::Announcement => {
                text.sections[0].style.color = ANNOUNCEMENT_COLOR;
            }
            _ => {
                // Regular chat styling
            }
        }
    }
}
```

### Mentions

Players can mention other players with the `@username` syntax:

```rust
/// Find and extract mentions from message content
fn find_mentions(content: &str) -> Vec<String> {
    let mut mentions = Vec::new();
    
    // Regular expression to find @mentions
    let re = Regex::new(r"@(\w+)").unwrap();
    for capture in re.captures_iter(content) {
        if let Some(username) = capture.get(1) {
            mentions.push(username.as_str().to_string());
        }
    }
    
    mentions
}

/// Highlight mentions in messages
fn highlight_mentions(
    mut message_query: Query<(&mut Text, &ChatMessage), Added<ChatMessage>>,
    local_player: Res<LocalPlayer>,
) {
    for (mut text, message) in &mut message_query {
        // Check if local player is mentioned
        if message.mentions.contains(&local_player.name) {
            // Highlight the entire message
            text.sections[0].style.color = MENTION_HIGHLIGHT_COLOR;
        }
    }
}
```

### Chat Filtering

The system includes filtering options for inappropriate content:

```rust
/// Filter message content for inappropriate language
fn filter_message_content(content: &str) -> String {
    let mut filtered = content.to_string();
    
    // Apply word filters
    for (pattern, replacement) in INAPPROPRIATE_WORDS.iter() {
        let re = Regex::new(pattern).unwrap();
        filtered = re.replace_all(&filtered, replacement).to_string();
    }
    
    filtered
}
```

### Notifications

Players receive notifications for new messages, especially mentions:

```rust
/// System to handle chat notifications
fn update_chat_notifications(
    message_query: Query<&ChatMessage, Added<ChatMessage>>,
    mut notification_state: ResMut<NotificationState>,
    local_player: Res<LocalPlayer>,
    lobby_state: Res<LobbyState>,
) {
    for message in &message_query {
        // Check if the player is mentioned
        if message.mentions.contains(&local_player.name) {
            notification_state.add_notification(NotificationType::ChatMention {
                sender: message.sender.clone().unwrap_or_default(),
                preview: truncate_message(&message.content, 30),
            });
        }
        
        // If chat is not focused, increment unread counter
        if !lobby_state.is_chat_focused {
            notification_state.unread_chat_messages += 1;
        }
    }
}
```

## Implementation

The chat system is implemented using Bevy's ECS architecture:

### Chat Panel Setup

```rust
/// Set up the chat panel UI
pub fn setup_chat_panel(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    // Chat panel container
    parent
        .spawn(Node {
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|chat_panel| {
            // Chat header
            chat_panel.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }).with_children(|header| {
                header.spawn(Text2d {
                    text: "CHAT".into(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                });
            });
            
            // Message display area
            chat_panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::Scroll,
                        ..default()
                    },
                    ChatMessageContainer,
                ))
                .with_children(|messages| {
                    // Messages will be spawned here dynamically
                });
                
            // Input area
            chat_panel
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(10.0)),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|input_area| {
                    // Text input field
                    input_area
                        .spawn((
                            Node {
                                width: Val::Percent(85.0),
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.7)),
                            BorderColor(Color::rgba(0.5, 0.5, 0.5, 0.7)),
                            Outline::new(Val::Px(1.0)),
                            ChatInputField,
                        ))
                        .with_children(|input| {
                            input.spawn((
                                Text2d {
                                    text: "Type a message...".into(),
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 14.0,
                                    color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                                },
                                ChatInputPlaceholder,
                            ));
                        });
                        
                    // Send button
                    input_area
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(15.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(Color::rgba(0.2, 0.4, 0.8, 0.7)),
                            ChatSendButton,
                        ))
                        .with_children(|button| {
                            button.spawn(Text2d {
                                text: "▶".into(),
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 18.0,
                                color: Color::WHITE,
                            });
                        });
                });
        });
}
```

### Message Rendering

```rust
/// System to render chat messages
fn render_chat_messages(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chat_state: Res<ChatState>,
    mut message_container_query: Query<Entity, With<ChatMessageContainer>>,
    message_query: Query<(Entity, &ChatMessage)>,
    time: Res<Time>,
) {
    if chat_state.is_dirty {
        // Clear existing messages
        if let Ok(container_entity) = message_container_query.get_single_mut() {
            commands.entity(container_entity).despawn_descendants();
            
            // Re-spawn all messages
            commands.entity(container_entity).with_children(|parent| {
                for &message_entity in &chat_state.messages {
                    if let Ok((_, message)) = message_query.get(message_entity) {
                        spawn_message_ui(parent, message, &asset_server, time.elapsed_seconds());
                    }
                }
            });
        }
        
        // Mark as clean
        chat_state.is_dirty = false;
    }
}

/// Spawn a single chat message UI element
fn spawn_message_ui(
    parent: &mut ChildBuilder,
    message: &ChatMessage,
    asset_server: &Res<AssetServer>,
    current_time: f64,
) {
    // Calculate relative time for display
    let time_ago = format_time_ago(current_time - message.timestamp);
    
    // Message container
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(get_message_background_color(&message.message_type)),
            BorderColor(get_message_border_color(&message.message_type)),
            Outline::new(Val::Px(1.0)),
            RenderChatMessage(message.id.clone()),
        ))
        .with_children(|message_container| {
            // Header with sender and timestamp
            message_container
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|header| {
                    // Sender name
                    if let Some(sender) = &message.sender {
                        header.spawn(Text2d {
                            text: format!("{}", sender),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            color: get_sender_color(sender),
                        });
                    } else {
                        header.spawn(Text2d {
                            text: "System".into(),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            color: SYSTEM_NAME_COLOR,
                        });
                    }
                    
                    // Timestamp
                    header.spawn(Text2d {
                        text: time_ago,
                        font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                        font_size: 12.0,
                        color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                    });
                });
                
            // Message content
            message_container.spawn(Text2d {
                text: message.content.clone(),
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 14.0,
                color: get_message_text_color(&message.message_type),
            });
        });
}
```

### Input Handling

```rust
/// System to handle chat input
fn handle_chat_input(
    mut char_input_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    mut chat_state: ResMut<ChatState>,
    mut chat_events: EventWriter<SendChatMessageEvent>,
    local_player: Res<LocalPlayer>,
) {
    if chat_state.is_input_focused {
        // Process character input
        for event in char_input_events.read() {
            if !event.char.is_control() {
                chat_state.current_input.push(event.char);
            }
        }
        
        // Handle backspace
        if keyboard_input.just_pressed(KeyCode::Backspace) && !chat_state.current_input.is_empty() {
            chat_state.current_input.pop();
        }
        
        // Handle Enter to send
        if keyboard_input.just_pressed(KeyCode::Return) && !chat_state.current_input.is_empty() {
            // Create chat message event
            chat_events.send(SendChatMessageEvent {
                content: chat_state.current_input.clone(),
                sender: local_player.name.clone(),
                message_type: MessageType::Chat,
            });
            
            // Clear input
            chat_state.current_input.clear();
        }
    }
}

/// System to handle send button clicks
fn handle_send_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ChatSendButton>)>,
    mut chat_state: ResMut<ChatState>,
    mut chat_events: EventWriter<SendChatMessageEvent>,
    local_player: Res<LocalPlayer>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed && !chat_state.current_input.is_empty() {
            // Create chat message event
            chat_events.send(SendChatMessageEvent {
                content: chat_state.current_input.clone(),
                sender: local_player.name.clone(),
                message_type: MessageType::Chat,
            });
            
            // Clear input
            chat_state.current_input.clear();
        }
    }
}
```

The chat system provides an essential communication channel for players to coordinate before starting their Commander game, helping ensure a fun and balanced gaming experience. 
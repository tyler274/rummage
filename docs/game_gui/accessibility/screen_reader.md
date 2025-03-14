# Screen Reader Support

This document describes how Rummage supports screen readers and implements accessible UI components.

## Overview

Accessibility is a core principle of Rummage's UI design. Screen reader support ensures that players with visual impairments can fully experience the game. Implementing proper screen reader support also benefits all players by providing clearer communication of game state and actions.

## Architecture

Rummage's screen reader support is built on these key components:

1. **Accessible Node Components**: Bevy components that provide semantic information
2. **Screen Reader Bridge**: System for communicating with platform screen reader APIs
3. **Focus Management**: System for tracking and managing UI focus
4. **Keyboard Navigation**: Support for navigating UI without a mouse

## Accessible Components

Each UI component implements the `AccessibleNode` component:

```rust
#[derive(Component)]
pub struct AccessibleNode {
    /// Human-readable label for the element
    pub label: String,
    /// Semantic role of the element
    pub role: AccessibilityRole,
    /// Current state of the element
    pub state: AccessibilityState,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AccessibilityRole {
    Button,
    Card,
    Checkbox,
    Dialog,
    Grid,
    GridCell,
    Image,
    Link,
    List,
    ListItem,
    Menu,
    MenuItem,
    Slider,
    Tab,
    TabPanel,
    Text,
    // Game-specific roles
    GameZone,
    PlayerInfo,
    PhaseIndicator,
}

#[derive(Default)]
pub struct AccessibilityState {
    pub disabled: bool,
    pub selected: bool,
    pub focused: bool,
    pub expanded: bool,
    pub pressed: bool,
}
```

## Implementation Examples

### Card Component

Here's how a card component implements screen reader accessibility:

```rust
fn spawn_card_entity(
    commands: &mut Commands,
    card_data: &CardData,
) -> Entity {
    commands.spawn((
        // Visual components
        SpriteBundle { ... },
        
        // Game logic components
        Card { ... },
        
        // Accessibility component
        AccessibleNode {
            label: format!("{}, {}", card_data.name, card_data.type_line),
            role: AccessibilityRole::Card,
            state: AccessibilityState {
                selected: false,
                ..default()
            },
            properties: {
                let mut props = HashMap::new();
                props.insert("power".to_string(), card_data.power.to_string());
                props.insert("toughness".to_string(), card_data.toughness.to_string());
                props.insert("rules_text".to_string(), card_data.rules_text.clone());
                props
            },
        },
    )).id()
}
```

### Game Zone

Game zones are important landmarks for screen reader navigation:

```rust
fn spawn_hand_zone(commands: &mut Commands) -> Entity {
    commands.spawn((
        // Visual components
        NodeBundle { ... },
        
        // Game zone component
        HandZone { ... },
        
        // Accessibility component
        AccessibleNode {
            label: "Hand".to_string(),
            role: AccessibilityRole::GameZone,
            state: default(),
            properties: {
                let mut props = HashMap::new();
                props.insert("card_count".to_string(), "0".to_string());
                props
            },
        },
    )).id()
}
```

## Focus Management

The focus management system tracks which element has keyboard focus:

```rust
fn update_focus(
    keyboard_input: Res<Input<KeyCode>>,
    mut focused_entity: ResMut<FocusedEntity>,
    mut query: Query<(Entity, &mut AccessibleNode)>,
) {
    // Handle Tab navigation
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let shift = keyboard_input.pressed(KeyCode::ShiftLeft) || 
                    keyboard_input.pressed(KeyCode::ShiftRight);
        
        if shift {
            focused_entity.focus_previous(&mut query);
        } else {
            focused_entity.focus_next(&mut query);
        }
    }
    
    // Update accessibility state based on focus
    for (entity, mut node) in query.iter_mut() {
        node.state.focused = Some(entity) == focused_entity.0;
    }
}
```

## Screen Reader Announcements

The game communicates important events to screen readers:

```rust
fn announce_phase_change(
    mut phase_events: EventReader<PhaseChangeEvent>,
    mut screen_reader: ResMut<ScreenReaderBridge>,
) {
    for event in phase_events.read() {
        let announcement = format!(
            "Phase changed to {} for player {}",
            event.new_phase.name(),
            event.active_player.name
        );
        
        screen_reader.announce(announcement);
    }
}
```

## Card State Announcements

Changes to card state are announced to the screen reader:

```rust
fn announce_card_state_changes(
    mut card_events: EventReader<CardStateChangeEvent>,
    mut screen_reader: ResMut<ScreenReaderBridge>,
    card_query: Query<&Card>,
) {
    for event in card_events.read() {
        if let Ok(card) = card_query.get(event.card_entity) {
            let announcement = match event.state_change {
                CardStateChange::Tapped => format!("{} tapped", card.name),
                CardStateChange::Untapped => format!("{} untapped", card.name),
                CardStateChange::Destroyed => format!("{} destroyed", card.name),
                CardStateChange::Exiled => format!("{} exiled", card.name),
                // Other state changes
            };
            
            screen_reader.announce(announcement);
        }
    }
}
```

## Keyboard Shortcuts

Rummage provides comprehensive keyboard shortcuts for gameplay:

| Key | Action |
|-----|--------|
| Space | Select/Activate focused element |
| Tab | Move focus to next element |
| Shift+Tab | Move focus to previous element |
| Arrow keys | Navigate within a zone or component |
| 1-9 | Select cards in hand |
| P | Pass priority |
| M | Open mana pool |
| C | View card details |
| Esc | Cancel current action |

## Testing Screen Reader Support

Screen reader support is tested through:

1. **Unit tests**: Test the accessibility components and focus management
2. **Integration tests**: Test the screen reader bridge with mock screen readers
3. **End-to-end tests**: Test with actual screen readers on target platforms
4. **User testing**: Work with visually impaired players to validate usability

## Platform Support

Rummage supports these screen reader platforms:

- **Windows**: NVDA and JAWS
- **macOS**: VoiceOver
- **Linux**: Orca
- **Web**: ARIA support for browser-based screen readers

## Future Improvements

Planned improvements for screen reader support:

- Enhanced contextual descriptions for complex board states
- Custom screen reader modes for different game phases
- Integrated tutorials specific to screen reader users
- Support for braille displays

## Best Practices

When implementing new UI components, follow these accessibility best practices:

1. Always include an `AccessibleNode` component with appropriate role and label
2. Ensure all interactive elements are keyboard navigable
3. Announce important state changes
4. Test with actual screen readers
5. Group related elements semantically

## Related Documentation

- [Color Contrast](color_contrast.md)
- [Control Options](control_options.md)
- [Keyboard Navigation](control_options.md#keyboard-navigation) 
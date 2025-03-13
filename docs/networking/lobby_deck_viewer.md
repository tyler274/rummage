# Lobby Deck Viewer UI

This document describes the deck viewer UI component in the multiplayer lobby. This feature allows players to view each other's commanders and deck information before starting a game, which helps ensure balanced gameplay and appropriate power levels.

## Table of Contents

1. [Overview](#overview)
2. [UI Components](#ui-components)
3. [Privacy Controls](#privacy-controls)
4. [Commander Preview](#commander-preview)
5. [Deck Statistics](#deck-statistics)
6. [Implementation](#implementation)

## Overview

The deck viewer occupies the right panel of the lobby detail screen, providing a preview of selected decks and their commanders. This feature promotes transparency and helps players ensure they're entering a game with compatible deck power levels.

```
┌───────────────────────────────────┐
│         Deck Viewer               │
├───────────────────────────────────┤
│                                   │
│ ┌─────────────────────────────┐   │
│ │                             │   │
│ │                             │   │
│ │     Commander Card          │   │
│ │                             │   │
│ │                             │   │
│ └─────────────────────────────┘   │
│                                   │
│ Player: Player2                   │
│ Deck: "Competitive Elves"         │
│                                   │
│ Color Identity: G                 │
│ Average CMC: 2.8                  │
│ Power Level: 7                    │
│                                   │
│ ┌─────────────────────────────┐   │
│ │  View Full Decklist (15/60) │   │
│ └─────────────────────────────┘   │
└───────────────────────────────────┘
```

## UI Components

The deck viewer consists of multiple components:

### Commander Card Display

- Large card image for the commander
- Partner commander toggle (if applicable)
- Card details (name, mana cost, type line, rules text)
- Color identity indicators

### Deck Information

- Deck name
- Owner's name
- Format legality
- Deck description (optional)

### Deck Statistics

- Color identity
- Card count
- Average mana value
- Card type distribution
- Mana curve graph
- Estimated power level

### Action Buttons

- View full decklist button (if shared by owner)
- Request deck details button
- Select deck button (for local player)

## Privacy Controls

Players have control over how much of their deck information is shared:

```rust
/// Deck privacy settings
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckPrivacyLevel {
    /// Only commander is visible
    CommanderOnly,
    /// Commander and basic stats are visible
    BasicStats,
    /// Full decklist is shared
    FullDecklist,
    /// Custom setting with specific visibility options
    Custom(DeckPrivacyOptions),
}

/// Detailed privacy options for decks
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckPrivacyOptions {
    /// Whether to show the commander
    pub show_commander: bool,
    /// Whether to show basic statistics
    pub show_stats: bool,
    /// Whether to show card type breakdown
    pub show_card_types: bool,
    /// Whether to show mana curve
    pub show_mana_curve: bool,
    /// Maximum number of cards to reveal (0 = none)
    pub cards_to_reveal: usize,
    /// Categories of cards to reveal
    pub reveal_categories: Vec<CardCategory>,
}
```

Players set their privacy preferences when readying up with a deck:

```rust
/// System to handle deck privacy settings
fn handle_deck_privacy_settings(
    mut interaction_query: Query<
        (&Interaction, &DeckPrivacyOption),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_deck: ResMut<SelectedDeck>,
) {
    for (interaction, privacy_option) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            selected_deck.privacy_level = privacy_option.level.clone();
        }
    }
}
```

## Commander Preview

The commander preview is the centerpiece of the deck viewer:

```rust
/// Set up the commander preview display
fn setup_commander_preview(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    commander_info: Option<&CommanderInfo>,
) {
    // Commander card container
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(300.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        })
        .with_children(|commander_container| {
            if let Some(commander) = commander_info {
                // Commander card image
                if let Some(ref image_uri) = commander.image_uri {
                    commander_container.spawn((
                        Image {
                            texture: asset_server.load(image_uri),
                            ..default()
                        },
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(300.0),
                            ..default()
                        },
                        CommanderCard,
                    ));
                } else {
                    // Fallback if no image is available
                    commander_container
                        .spawn((
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(300.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
                            BorderColor(Color::rgb(0.3, 0.3, 0.3)),
                            Outline::new(Val::Px(2.0)),
                            CommanderCard,
                        ))
                        .with_children(|card| {
                            // Card name and mana cost
                            card.spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            }).with_children(|header| {
                                // Card name
                                header.spawn(Text2d {
                                    text: commander.name.clone(),
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                });
                                
                                // Mana cost
                                header.spawn(Text2d {
                                    text: commander.mana_cost.clone(),
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                });
                            });
                            
                            // Type line
                            card.spawn(Text2d {
                                text: commander.type_line.clone(),
                                font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                                font_size: 14.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            });
                            
                            // Rules text
                            card.spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(70.0),
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            }).with_children(|text_box| {
                                text_box.spawn(Text2d {
                                    text: commander.text.clone(),
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                });
                            });
                            
                            // Power/Toughness if applicable
                            if let Some(ref pt) = commander.power_toughness {
                                card.spawn(Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Auto,
                                    justify_content: JustifyContent::FlexEnd,
                                    ..default()
                                }).with_children(|pt_container| {
                                    pt_container.spawn(Text2d {
                                        text: pt.clone(),
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                    });
                                });
                            }
                        });
                }
            } else {
                // No commander selected yet
                commander_container
                    .spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(300.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.5)),
                        BorderColor(Color::rgba(0.3, 0.3, 0.3, 0.5)),
                        Outline::new(Val::Px(1.0)),
                        EmptyCommanderCard,
                    ))
                    .with_children(|empty_card| {
                        empty_card.spawn(Text2d {
                            text: "No Commander Selected".into(),
                            font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                            font_size: 16.0,
                            color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                        });
                    });
            }
        });
}
```

## Deck Statistics

The deck statistics section provides additional information:

```rust
/// Set up the deck statistics display
fn setup_deck_statistics(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    deck_info: Option<&DeckInfo>,
) {
    // Stats container
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        })
        .with_children(|stats_container| {
            if let Some(deck) = deck_info {
                // Player and deck name
                stats_container.spawn(Text2d {
                    text: format!("Player: {}", deck.player_name),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                });
                
                stats_container.spawn(Text2d {
                    text: format!("Deck: \"{}\"", deck.name),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                });
                
                // Color identity
                stats_container
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    })
                    .with_children(|color_row| {
                        color_row.spawn(Text2d {
                            text: "Color Identity: ".into(),
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 14.0,
                            color: Color::WHITE,
                        });
                        
                        // Display color pips
                        for color in &deck.colors {
                            color_row.spawn((
                                Node {
                                    width: Val::Px(20.0),
                                    height: Val::Px(20.0),
                                    margin: UiRect::left(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(get_color_identity_color(color)),
                                BorderColor(Color::WHITE),
                                Outline::new(Val::Px(1.0)),
                                ColorIdentityPip(color.clone()),
                            ));
                        }
                    });
                
                // Other statistics
                stats_container.spawn(Text2d {
                    text: format!("Average CMC: {:.1}", deck.avg_mana_value),
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                });
                
                stats_container.spawn(Text2d {
                    text: format!("Power Level: {}/10", deck.power_level),
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    color: get_power_level_color(deck.power_level),
                });
                
                // View full decklist button
                if deck.share_decklist {
                    stats_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::rgba(0.2, 0.4, 0.8, 0.7)),
                            ViewDecklistButton(deck.player_id.clone()),
                        ))
                        .with_children(|button| {
                            button.spawn(Text2d {
                                text: format!("View Full Decklist ({}/{})", deck.card_count, deck.card_count),
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: 14.0,
                                color: Color::WHITE,
                            });
                        });
                } else {
                    // Decklist not shared
                    stats_container
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.7)),
                        ))
                        .with_children(|container| {
                            container.spawn(Text2d {
                                text: "Decklist Not Shared".into(),
                                font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                                font_size: 14.0,
                                color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                            });
                        });
                }
            } else {
                // No deck selected
                stats_container.spawn(Text2d {
                    text: "No Deck Selected".into(),
                    font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                    font_size: 16.0,
                    color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                });
            }
        });
}
```

## Implementation

The deck viewer is implemented using Bevy's ECS architecture:

### Overall Deck Viewer Setup

```rust
/// Set up the deck viewer panel UI
pub fn setup_deck_viewer_panel(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    // Deck viewer container
    parent
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            DeckViewerUI,
        ))
        .with_children(|deck_viewer| {
            // Header
            deck_viewer
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn(Text2d {
                        text: "DECK VIEWER".into(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    });
                });
                
            // Deck selection tabs
            setup_deck_selection_tabs(deck_viewer, asset_server);
            
            // Commander preview area
            setup_commander_preview(deck_viewer, asset_server, None);
            
            // Deck statistics area
            setup_deck_statistics(deck_viewer, asset_server, None);
        });
}

/// Set up tabs to select which player's deck to view
fn setup_deck_selection_tabs(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            DeckSelectionTabs,
        ));
    // Tabs will be populated dynamically based on players in lobby
}
```

### Deck Selection

The viewer supports selecting whose deck to view:

```rust
/// System to update deck selection tabs
fn update_deck_selection_tabs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lobby_info: Res<CurrentLobbyInfo>,
    tabs_query: Query<Entity, With<DeckSelectionTabs>>,
    children_query: Query<Entity>,
    selected_player_deck: Res<SelectedPlayerDeck>,
) {
    if lobby_info.is_changed() || selected_player_deck.is_changed() {
        // Get the tabs container
        if let Ok(tabs_entity) = tabs_query.get_single() {
            // Clear existing tabs
            let children = children_query.iter_descendants(tabs_entity);
            for child in children {
                commands.entity(child).despawn_recursive();
            }
            
            // Add tabs for each player with a selected deck
            commands.entity(tabs_entity).with_children(|tabs| {
                for player in lobby_info.players.values() {
                    if player.status == PlayerLobbyState::Ready {
                        let is_selected = selected_player_deck.player_id == Some(player.id.clone());
                        
                        tabs.spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                min_width: Val::Px(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                margin: UiRect::right(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(if is_selected {
                                Color::rgba(0.3, 0.5, 0.8, 0.7)
                            } else {
                                Color::rgba(0.2, 0.2, 0.2, 0.7)
                            }),
                            DeckSelectionTab(player.id.clone()),
                        )).with_children(|tab| {
                            tab.spawn(Text2d {
                                text: player.name.clone(),
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: 14.0,
                                color: Color::WHITE,
                            });
                        });
                    }
                }
            });
        }
    }
}

/// System to handle deck tab selection
fn handle_deck_tab_selection(
    mut interaction_query: Query<
        (&Interaction, &DeckSelectionTab),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_player_deck: ResMut<SelectedPlayerDeck>,
) {
    for (interaction, tab) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            selected_player_deck.player_id = Some(tab.0.clone());
        }
    }
}
```

### Full Decklist Viewer

When a player clicks to view a full decklist (if shared):

```rust
/// System to handle decklist view button
fn handle_view_decklist_button(
    mut interaction_query: Query<
        (&Interaction, &ViewDecklistButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut deck_view_events: EventWriter<ViewDecklistEvent>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            deck_view_events.send(ViewDecklistEvent(button.0.clone()));
        }
    }
}

/// System to show full decklist popup
fn show_decklist_popup(
    mut commands: Commands,
    mut deck_view_events: EventReader<ViewDecklistEvent>,
    asset_server: Res<AssetServer>,
    lobby_connection: Res<LobbyConnection>,
) {
    for event in deck_view_events.read() {
        // Request decklist from server
        lobby_connection.request_decklist(event.0.clone());
        
        // Show loading popup
        commands
            .spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.95)),
                BorderColor(Color::rgba(0.3, 0.3, 0.3, 1.0)),
                Outline::new(Val::Px(2.0)),
                DecklistPopup,
            ))
            .with_children(|popup| {
                // Header
                popup
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    })
                    .with_children(|header| {
                        // Title
                        header.spawn(Text2d {
                            text: "Loading Decklist...".into(),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        });
                        
                        // Close button
                        header
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::rgba(0.7, 0.3, 0.3, 0.7)),
                                CloseDecklistPopupButton,
                            ))
                            .with_children(|button| {
                                button.spawn(Text2d {
                                    text: "X".into(),
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                });
                            });
                    });
                
                // Loading indicator
                popup.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    DecklistContentContainer,
                )).with_children(|container| {
                    container.spawn(Text2d {
                        text: "Loading...".into(),
                        font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                        font_size: 18.0,
                        color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                    });
                });
            });
    }
}
```

The deck viewer provides a crucial function in multiplayer Commander games by allowing players to gauge deck compatibility before starting a game, helping to ensure a balanced and enjoyable experience for all participants. 
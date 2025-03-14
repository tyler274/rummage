# Zone Visualization

This document describes how different game zones are visually represented in the Rummage UI.

## Zone Types

Magic: The Gathering defines several game zones, each with unique visualization requirements:

- **Battlefield**: Where permanents are played
- **Hand**: Cards held by a player
- **Library**: A player's deck
- **Graveyard**: Discard pile
- **Exile**: Cards removed from the game
- **Stack**: Spells and abilities waiting to resolve
- **Command Zone**: Special zone for commanders and emblems

## Visual Representation

Each zone has a distinct visual style to help players quickly identify it:

### Battlefield

- **Layout**: Grid-based layout with flexible positioning
- **Background**: Textured playmat appearance
- **Borders**: Subtle zone boundaries for each player's area
- **Organization**: Cards automatically group by type (creatures, lands, etc.)

```rust
fn create_battlefield_zone(commands: &mut Commands, materials: &UiMaterials) {
    commands.spawn((
        SpatialBundle::default(),
        Node {
            background_color: materials.battlefield_background.clone(),
            ..default()
        },
        BattlefieldZone,
        Name::new("Battlefield Zone"),
    ));
}
```

### Hand

- **Layout**: Fan or straight-line arrangement
- **Background**: Semi-transparent panel
- **Privacy**: Only visible to the controlling player (except in spectator mode)
- **Interaction**: Cards rise when hovered

### Library

- **Visualization**: Stack of cards showing only the back
- **Counter**: Numerical display of remaining cards
- **Animation**: Cards visibly draw from top

### Graveyard

- **Layout**: Stacked with slight offset to show multiple cards
- **Visibility**: Top card always visible
- **Interaction**: Can be expanded to view all cards

### Exile

- **Visual Style**: Distinct "removed from game" appearance
- **Organization**: Grouped by source when relevant
- **Special Effects**: Subtle visual effects to distinguish from other zones

### Stack

- **Layout**: Cascading arrangement showing pending spells/abilities
- **Order**: Clear visual indication of resolution order
- **Targeting**: Visual connections to targets
- **Animation**: Items move as they resolve

### Command Zone

- **Prominence**: Visually distinct and always accessible
- **Commander Display**: Shows commander card(s)
- **Tax Counter**: Visual indicator of commander tax

## Zone Transitions

When cards move between zones, the transition is animated to provide visual feedback:

```rust
fn animate_zone_transition(
    commands: &mut Commands,
    card_entity: Entity,
    from_zone: Zone,
    to_zone: Zone,
    animation_assets: &AnimationAssets,
) {
    // Calculate start and end positions
    let start_pos = get_zone_position(from_zone);
    let end_pos = get_zone_position(to_zone);
    
    // Create animation sequence
    commands.entity(card_entity).insert(AnimationSequence {
        animations: vec![
            Animation::new_position(start_pos, end_pos, Duration::from_millis(300)),
            Animation::new_scale(Vec3::splat(0.9), Vec3::ONE, Duration::from_millis(150)),
        ],
        on_complete: Some(ZoneTransitionComplete { zone: to_zone }),
    });
}
```

## Zone Interaction

Zones support various interactions:

- **Click**: Select the zone or expand it for more detail
- **Drag-to**: Move cards to the zone
- **Drag-from**: Take cards from the zone
- **Right-click**: Open zone-specific context menu
- **Hover**: Show additional information about the zone

## Zone Components

Zones are implemented using several components:

```rust
#[derive(Component)]
pub struct ZoneVisualization {
    pub zone_type: Zone,
    pub owner: Option<Entity>,
    pub expanded: bool,
    pub card_count: usize,
    pub layout_style: ZoneLayoutStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneLayoutStyle {
    Grid,
    Stack,
    Fan,
    Cascade,
    List,
}
```

## Zone Systems

Several systems manage zone visualization:

- **Zone Layout System**: Positions cards within zones
- **Zone Transition System**: Handles card movements between zones
- **Zone Interaction System**: Processes player interactions with zones
- **Zone Update System**: Updates zone visuals based on game state

```rust
fn update_zone_visuals(
    mut zone_query: Query<(&mut ZoneVisualization, &Children)>,
    card_query: Query<Entity, With<Card>>,
    game_state: Res<GameState>,
) {
    for (mut zone, children) in zone_query.iter_mut() {
        // Update card count
        zone.card_count = children.iter()
            .filter(|child| card_query.contains(**child))
            .count();
            
        // Update visuals based on card count and zone type
        // ...
    }
}
```

## Multiplayer Considerations

In multiplayer games, zones are arranged to clearly indicate ownership:

- **Player Zones**: Positioned relative to each player's seat
- **Shared Zones**: Positioned in neutral areas
- **Opponent Zones**: Scaled and positioned for visibility

## Accessibility Features

Zone visualization includes accessibility considerations:

- **Color Coding**: Zones have distinct colors that work with colorblind modes
- **Text Labels**: Optional text labels for each zone
- **Screen Reader Support**: Zones announce their type and contents
- **Keyboard Navigation**: Zones can be selected and manipulated via keyboard

## Integration

Zone visualization integrates with:

- [Layout System](layout_system.md): Zones are positioned by the layout system
- [Card Visualization](../cards/index.md): Cards appear differently based on their zone
- [Responsive Design](responsive_design.md): Zones adapt to different screen sizes

For more details on specific zone implementations, see the relevant game rules documentation. 
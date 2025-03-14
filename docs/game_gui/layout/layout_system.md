# Layout System

The Layout System is responsible for organizing and positioning UI elements in the Rummage game interface. It provides a flexible foundation for arranging game elements across different screen sizes and orientations.

## Core Architecture

The layout system is built on Bevy's native UI components, using a component-based approach:

```rust
#[derive(Component)]
pub struct LayoutContainer {
    pub container_type: ContainerType,
    pub flex_direction: FlexDirection,
    pub size: Vec2,
    pub padding: UiRect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    Root,
    Player,
    Battlefield,
    Hand,
    Stack,
    CommandZone,
    InfoPanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
```

## Hierarchy Structure

The UI is organized in a hierarchical structure:

1. **Root Container**: The main layout container that spans the entire screen
2. **Zone Containers**: Containers for each game zone (battlefield, hand, graveyard, etc.)
3. **Element Containers**: Containers for specific UI elements (cards, controls, info panels)
4. **Individual Elements**: The actual UI components that players interact with

## Layout Calculation

Layout positions are calculated using a combination of:

- **Absolute Positioning**: Fixed positions for certain UI elements
- **Flex Layout**: Flexible positioning based on container size and available space
- **Grid Layout**: Grid-based positioning for elements like cards on the battlefield
- **Anchoring**: Elements can be anchored to edges or centers of their containers

```rust
fn calculate_layout(
    mut query: Query<(&mut Transform, &LayoutElement, &Parent)>,
    containers: Query<&LayoutContainer>,
) {
    // Implementation details for calculating positions
}
```

## Responsive Design

The layout system adapts to different screen sizes and aspect ratios:

- **Breakpoints**: Different layouts are used at specific screen size breakpoints
- **Scale Factors**: UI elements scale based on screen resolution
- **Prioritization**: Critical elements are prioritized in limited space
- **Overflow Handling**: Scrolling or pagination for overflow content

```rust
fn adapt_to_screen_size(
    mut layout_query: Query<&mut LayoutContainer>,
    windows: Res<Windows>,
) {
    // Implementation details for screen adaptation
}
```

## Zone-specific Layouts

Each game zone has specialized layout behavior:

### Battlefield Layout

- Grid-based layout for permanents
- Automatic card grouping by type
- Dynamic spacing based on card count
- Support for attacking/blocking visualization

### Hand Layout

- Fan or straight-line layouts
- Automatic card reorganization
- Vertical position adjustment during card selection
- Overflow handling for large hands

### Stack Layout

- Cascading layout for spells and abilities
- Visual nesting for complex interactions
- Animation support for stack resolution

## Integration with Card Visualization

The layout system works closely with the [Card Visualization](../cards/index.md) system:

- Positions cards according to their game state
- Handles z-ordering for overlapping cards
- Provides layout input for card animations
- Adjusts spacing based on card visibility needs

## Plugin Integration

The layout system is implemented as a Bevy plugin:

```rust
pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_layout)
            .add_systems(
                Update,
                (
                    calculate_layout,
                    adapt_to_screen_size,
                    handle_layout_changes,
                ),
            );
    }
}
```

## Performance Considerations

The layout system is optimized for performance:

- Batched layout calculations to minimize CPU usage
- Layout caching to prevent unnecessary recalculations
- Hierarchical dirty flagging to only update changed layouts
- Custom layout algorithms for common patterns

## Example: Battlefield Layout

```rust
fn create_battlefield_layout(commands: &mut Commands, ui_materials: &UiMaterials) {
    commands
        .spawn((
            SpatialBundle::default(),
            LayoutContainer {
                container_type: ContainerType::Battlefield,
                flex_direction: FlexDirection::Column,
                size: Vec2::new(1600.0, 900.0),
                padding: UiRect::all(Val::Px(10.0)),
            },
            Name::new("Battlefield Container"),
        ))
        .with_children(|parent| {
            // Create player areas within battlefield
            for player_index in 0..4 {
                create_player_battlefield_area(parent, ui_materials, player_index);
            }
            
            // Create center area for shared elements
            create_center_battlefield_area(parent, ui_materials);
        });
}
```

## Customization

The layout system supports customization through:

- **Layout Themes**: Pre-defined layout configurations
- **User Preferences**: User-adjustable layout options
- **Dynamic Adaptation**: Runtime layout adjustments based on game state

For more details on how layouts integrate with other UI systems, see [Zone Visualization](zone_visualization.md) and [Responsive Design](responsive_design.md). 
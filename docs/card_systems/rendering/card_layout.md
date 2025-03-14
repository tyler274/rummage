# Card Layout

This document details the layout system used for rendering Magic: The Gathering cards in Rummage. The layout system ensures cards are displayed with the correct proportions, elements, and visual style.

## Card Dimensions

Standard Magic cards have specific dimensions that are maintained in the rendering system:

- **Physical card ratio**: 2.5" × 3.5" (63mm × 88mm)
- **Digital aspect ratio**: 5:7 (0.714)
- **Standard rendering size**: Configurable based on UI scale

## Layout Components

Cards are composed of several layout components:

### Frame Components

- **Border**: The outermost edge of the card
- **Frame**: The colored frame indicating card type
- **Text box**: Area containing rules text
- **Art box**: Area containing the card's artwork
- **Title bar**: Area containing the card's name and mana cost
- **Type line**: Area containing the card's type information
- **Expansion symbol**: Symbol indicating the card's set and rarity

### Text Components

- **Name text**: The card's name
- **Type text**: The card's type line
- **Rules text**: The card's rules text
- **Flavor text**: The card's flavor text
- **Power/toughness**: Creature's power and toughness
- **Loyalty**: Planeswalker's loyalty

### Special Components

- **Mana symbols**: Symbols in the mana cost and rules text
- **Watermark**: Background symbol in the text box
- **Foil pattern**: Overlay for foil cards
- **Collector information**: Card number, artist, copyright

## Layout System

The layout system uses a combination of:

- **Relative positioning**: Elements positioned relative to card dimensions
- **Grid system**: Invisible grid for consistent alignment
- **Responsive scaling**: Adjusting to different display sizes
- **Dynamic text flow**: Text that reflows based on content

## Implementation

The layout is implemented using Bevy's UI system:

```rust
// Example of creating a card layout
fn create_card_layout(
    commands: &mut Commands,
    card_entity: Entity,
    card_data: &CardData,
    assets: &CardAssets,
) {
    // Create the main card entity
    commands.entity(card_entity)
        .insert(Node::default())
        .insert(Style {
            size: Size::new(Val::Px(300.0), Val::Px(420.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            // Add frame background
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                image: assets.get_frame_for_card(card_data).into(),
                ..Default::default()
            });
            
            // Add art box
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(40.0)),
                    margin: UiRect::new(
                        Val::Percent(5.0),
                        Val::Percent(5.0),
                        Val::Percent(15.0),
                        Val::Auto,
                    ),
                    ..Default::default()
                },
                image: assets.get_art_for_card(card_data).into(),
                ..Default::default()
            });
            
            // Add name text
            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(20.0),
                        left: Val::Px(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    card_data.name.clone(),
                    TextStyle {
                        font: assets.card_name_font.clone(),
                        font_size: 18.0,
                        color: Color::BLACK,
                    },
                ),
                ..Default::default()
            });
            
            // Add other card elements...
        });
}
```

## Special Card Layouts

The system supports various special card layouts:

### Split Cards

- **Layout**: Two half-cards joined along an edge
- **Orientation**: Can be displayed horizontally or vertically
- **Text scaling**: Smaller text to fit in half-size areas

### Double-Faced Cards

- **Front face**: Primary card face
- **Back face**: Secondary card face
- **Transition**: Smooth flip animation between faces

### Adventure Cards

- **Main card**: The creature portion
- **Adventure**: The spell portion in a sub-frame

### Saga Cards

- **Chapter markers**: Visual indicators for saga chapters
- **Art layout**: Extended art area
- **Chapter text**: Text for each chapter ability

## Text Rendering

Text rendering is handled with special care:

- **Font selection**: Fonts that match the Magic card style
- **Text scaling**: Automatic scaling based on text length
- **Symbol replacement**: Replacing mana symbols in text
- **Text wrapping**: Proper wrapping of rules text
- **Italics and emphasis**: Styling for flavor text and reminders

## Related Documentation

- [Art Assets](art_assets.md): The art assets used in card layouts
- [Card Rendering](index.md): Overview of the card rendering system
- [UI Integration](../ui_integration.md): How card layouts integrate with the UI 
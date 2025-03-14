# Art Assets

This document details the art assets used in the Rummage card rendering system. These assets are essential for creating visually accurate and appealing card representations.

## Asset Categories

The rendering system uses several categories of art assets:

### Card Artwork

- **Card illustrations**: The main artwork for each card
- **Alternative art**: Alternate versions of card artwork
- **Full-art treatments**: Extended artwork for special cards
- **Borderless treatments**: Artwork that extends to the card edges

### Card Frames

- **Standard frames**: Regular card frames for each card type
- **Special frames**: Unique frames for special card sets
- **Showcase frames**: Stylized frames for showcase cards
- **Promo frames**: Special frames for promotional cards

### Symbols

- **Mana symbols**: Symbols representing different mana types
- **Set symbols**: Symbols for each Magic: The Gathering set
- **Rarity indicators**: Symbols indicating card rarity
- **Ability symbols**: Symbols for keyword abilities (e.g., tap symbol)

### UI Elements

- **Counter indicators**: Visual elements for counters on cards
- **Status indicators**: Indicators for card states (tapped, etc.)
- **Selection highlights**: Visual feedback for selected cards
- **Targeting arrows**: Visual elements for targeting

## Asset Management

### Asset Loading

Assets are loaded using Bevy's asset system:

```rust
// Example of loading card assets
fn load_card_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load mana symbols texture atlas
    let mana_symbols_texture = asset_server.load("textures/mana_symbols.png");
    let mana_atlas = TextureAtlas::from_grid(
        mana_symbols_texture,
        Vec2::new(32.0, 32.0),
        6, 3,
        None,
        None
    );
    let mana_atlas_handle = texture_atlases.add(mana_atlas);
    
    // Store the handle for later use
    commands.insert_resource(ManaSymbolsAtlas(mana_atlas_handle));
    
    // Load card frame textures
    let standard_frame = asset_server.load("textures/frames/standard.png");
    commands.insert_resource(StandardFrame(standard_frame));
    
    // ... load other assets
}
```

### Asset Optimization

The system optimizes asset usage through:

- **Texture atlases**: Combining multiple small textures
- **Mipmap generation**: For different viewing distances
- **Asset caching**: Reusing assets across multiple cards
- **Lazy loading**: Loading assets only when needed

## Asset Creation Pipeline

### Source Files

- **Vector graphics**: SVG files for symbols and UI elements
- **High-resolution artwork**: Source artwork files
- **Template designs**: Base designs for card frames

### Processing

- **Rasterization**: Converting vector graphics to textures
- **Compression**: Optimizing file sizes
- **Format conversion**: Converting to engine-compatible formats
- **Atlas generation**: Creating texture atlases

## Special Asset Types

### Animated Assets

- **Card animations**: Special effects for certain cards
- **Transition effects**: Animations for state changes
- **Foil effects**: Simulating foil card appearance
- **Parallax effects**: Depth effects for special cards

### Procedural Assets

- **Dynamic frames**: Frames generated based on card properties
- **Custom counters**: Counters generated for specific cards
- **Adaptive backgrounds**: Backgrounds that adapt to card colors

## Asset Organization

Assets are organized in a structured directory hierarchy:

```
assets/
├── textures/
│   ├── card_art/
│   │   ├── set_code/
│   │   │   └── card_name.png
│   ├── frames/
│   │   ├── standard/
│   │   ├── special/
│   │   └── promo/
│   ├── symbols/
│   │   ├── mana/
│   │   ├── set/
│   │   └── ability/
│   └── ui/
│       ├── counters/
│       ├── indicators/
│       └── effects/
└── shaders/
    ├── card_effects/
    └── special_treatments/
```

## Related Documentation

- [Card Layout](card_layout.md): How assets are used in card layouts
- [Card Rendering](index.md): Overview of the card rendering system
- [UI Integration](../ui_integration.md): How assets integrate with the UI 
# Color Identity in Commander

This document explains how color identity is implemented in the Rummage game engine for the Commander format.

## Color Identity Rules

In Commander, a card's color identity determines which decks it can be included in. The color identity rules are:

1. A card's color identity includes all colored mana symbols that appear:
   - In its mana cost
   - In its rules text
   - On either face of a double-faced card
   - On all parts of a split or adventure card

2. Color identity is represented by the colors: White, Blue, Black, Red, and Green

3. A card's color identity can include colors even if the card itself is not those colors

4. Cards in a commander deck must only use colors within the color identity of the deck's commander

5. Basic lands with intrinsic mana abilities are only legal in decks where all their produced colors are in the commander's color identity

## Component Implementation

```rust
/// Component representing a card's color identity
#[derive(Component, Debug, Clone, Reflect)]
pub struct ColorIdentity {
    /// White in color identity
    pub white: bool,
    /// Blue in color identity
    pub blue: bool,
    /// Black in color identity
    pub black: bool,
    /// Red in color identity
    pub red: bool,
    /// Green in color identity
    pub green: bool,
}

impl ColorIdentity {
    /// Creates a new color identity from a set of colors
    pub fn new(white: bool, blue: bool, black: bool, red: bool, green: bool) -> Self {
        Self {
            white,
            blue,
            black,
            red,
            green,
        }
    }
    
    /// Creates a colorless identity
    pub fn colorless() -> Self {
        Self::new(false, false, false, false, false)
    }
    
    /// Checks if this color identity contains another
    pub fn contains(&self, other: &ColorIdentity) -> bool {
        (!other.white || self.white) &&
        (!other.blue || self.blue) &&
        (!other.black || self.black) &&
        (!other.red || self.red) &&
        (!other.green || self.green)
    }
    
    /// Gets the colors as an array of booleans [W, U, B, R, G]
    pub fn as_array(&self) -> [bool; 5] {
        [self.white, self.blue, self.black, self.red, self.green]
    }
    
    /// Gets the number of colors in this identity
    pub fn color_count(&self) -> usize {
        self.as_array().iter().filter(|&&color| color).count()
    }
    
    /// Checks if this is a colorless identity
    pub fn is_colorless(&self) -> bool {
        self.color_count() == 0
    }
}
```

## Calculating Color Identity

The system for calculating a card's color identity needs to analyze all text and symbols on the card:

```rust
/// Calculates a card's color identity from its various components
pub fn calculate_color_identity(
    mana_cost: &Mana,
    rules_text: &str,
    card_type: &CardTypes,
) -> ColorIdentity {
    let mut identity = ColorIdentity::colorless();
    
    // Check mana cost
    identity.white = identity.white || mana_cost.white > 0;
    identity.blue = identity.blue || mana_cost.blue > 0;
    identity.black = identity.black || mana_cost.black > 0;
    identity.red = identity.red || mana_cost.red > 0;
    identity.green = identity.green || mana_cost.green > 0;
    
    // Check rules text for mana symbols using regex
    let re = Regex::new(r"\{([WUBRG])/([WUBRG])\}|\{([WUBRG])\}").unwrap();
    for cap in re.captures_iter(rules_text) {
        if let Some(hybrid_1) = cap.get(1) {
            match hybrid_1.as_str() {
                "W" => identity.white = true,
                "U" => identity.blue = true,
                "B" => identity.black = true,
                "R" => identity.red = true,
                "G" => identity.green = true,
                _ => {}
            }
        }
        if let Some(hybrid_2) = cap.get(2) {
            match hybrid_2.as_str() {
                "W" => identity.white = true,
                "U" => identity.blue = true,
                "B" => identity.black = true,
                "R" => identity.red = true,
                "G" => identity.green = true,
                _ => {}
            }
        }
        if let Some(single) = cap.get(3) {
            match single.as_str() {
                "W" => identity.white = true,
                "U" => identity.blue = true,
                "B" => identity.black = true,
                "R" => identity.red = true,
                "G" => identity.green = true,
                _ => {}
            }
        }
    }
    
    // Check for color indicators
    if let Some(colors) = card_type.get_color_indicator() {
        for color in colors {
            match color {
                Color::White => identity.white = true,
                Color::Blue => identity.blue = true,
                Color::Black => identity.black = true,
                Color::Red => identity.red = true,
                Color::Green => identity.green = true,
                _ => {}
            }
        }
    }
    
    identity
}
```

## Deck Validation

Deck validation ensures that all cards in a deck match the commander's color identity:

```rust
/// System to validate deck color identity during deck construction
pub fn validate_deck_color_identity(
    commanders: Query<(&Commander, &ColorIdentity)>,
    cards: Query<(Entity, &InDeck, &ColorIdentity)>,
    mut validation_events: EventWriter<DeckValidationEvent>,
) {
    // For each deck
    let decks = cards.iter()
        .map(|(_, in_deck, _)| in_deck.deck)
        .collect::<HashSet<_>>();
    
    for deck in decks {
        // Find the commander(s) for this deck
        let deck_commanders = commanders.iter()
            .filter(|(cmdr, _)| cmdr.deck == deck)
            .collect::<Vec<_>>();
        
        if deck_commanders.is_empty() {
            validation_events.send(DeckValidationEvent::Invalid {
                deck,
                reason: "No commander found for deck".to_string(),
            });
            continue;
        }
        
        // Calculate the combined color identity for all commanders (for partner support)
        let mut combined_identity = ColorIdentity::colorless();
        for (_, identity) in deck_commanders.iter() {
            combined_identity.white |= identity.white;
            combined_identity.blue |= identity.blue;
            combined_identity.black |= identity.black;
            combined_identity.red |= identity.red;
            combined_identity.green |= identity.green;
        }
        
        // Check that all cards match the commander's color identity
        for (entity, in_deck, card_identity) in cards.iter() {
            if in_deck.deck != deck {
                continue;
            }
            
            if !combined_identity.contains(card_identity) {
                validation_events.send(DeckValidationEvent::InvalidCard {
                    deck,
                    card: entity,
                    reason: format!("Card color identity outside of commander's color identity"),
                });
            }
        }
    }
}
```

## Special Cases

### Partner Commanders

When using partner commanders, the deck's color identity is the union of both commanders' color identities:

```rust
/// System to handle partner commanders' combined color identity
pub fn handle_partner_color_identity(
    partners: Query<(Entity, &Partner, &ColorIdentity)>,
    decks: Query<&Deck>,
) {
    // Group partners by deck
    let mut deck_partners = HashMap::new();
    for (entity, partner, identity) in partners.iter() {
        deck_partners.entry(partner.deck)
            .or_insert_with(Vec::new)
            .push((entity, identity));
    }
    
    // For each deck with partners
    for (deck_entity, partners) in deck_partners.iter() {
        if let Ok(deck) = decks.get(*deck_entity) {
            // Calculate combined identity
            let mut combined = ColorIdentity::colorless();
            for (_, identity) in partners.iter() {
                combined.white |= identity.white;
                combined.blue |= identity.blue;
                combined.black |= identity.black;
                combined.red |= identity.red;
                combined.green |= identity.green;
            }
            
            // Store combined identity for deck validation
            // ...
        }
    }
}
```

### Five-Color Commanders

Five-color commanders like "The Ur-Dragon" allow any card in the deck:

```rust
impl ColorIdentity {
    /// Checks if this is a five-color identity
    pub fn is_five_color(&self) -> bool {
        self.white && self.blue && self.black && self.red && self.green
    }
}
```

## UI Representation

Color identity is visually represented in the UI:

1. The commander's color identity is displayed in the command zone
2. During deck construction, cards that don't match the commander's color identity are highlighted
3. Card browser filters can be set to only show cards matching the commander's color identity
4. Color identity is shown as colored pips in card displays

## Testing

### Example Tests

```rust
#[test]
fn test_color_identity_calculation() {
    // Test basic mana cost identity
    let cost = Mana::new(0, 1, 1, 0, 0, 0); // WU
    let identity = calculate_color_identity(&cost, "", &CardTypes::default());
    assert!(identity.white);
    assert!(identity.blue);
    assert!(!identity.black);
    assert!(!identity.red);
    assert!(!identity.green);
    
    // Test rules text identity
    let cost = Mana::new(0, 0, 0, 0, 0, 0); // Colorless
    let text = "Add {R} or {G} to your mana pool.";
    let identity = calculate_color_identity(&cost, text, &CardTypes::default());
    assert!(!identity.white);
    assert!(!identity.blue);
    assert!(!identity.black);
    assert!(identity.red);
    assert!(identity.green);
    
    // Test hybrid mana
    let cost = Mana::new(0, 0, 0, 0, 0, 0); // Colorless
    let text = "This spell costs {W/B} less to cast.";
    let identity = calculate_color_identity(&cost, text, &CardTypes::default());
    assert!(identity.white);
    assert!(!identity.blue);
    assert!(identity.black);
    assert!(!identity.red);
    assert!(!identity.green);
}

#[test]
fn test_deck_validation() {
    // Set up a test environment
    let mut app = App::new();
    app.add_systems(Update, validate_deck_color_identity)
        .add_event::<DeckValidationEvent>();
    
    // Create a Simic (Green-Blue) commander
    let commander_entity = app.world.spawn((
        Commander { deck: Entity::from_raw(1), ..Default::default() },
        ColorIdentity::new(false, true, false, false, true), // Blue-Green
    )).id();
    
    // Create a deck with the commander and some cards
    let deck_entity = Entity::from_raw(1);
    
    // Valid cards
    let valid_card1 = app.world.spawn((
        InDeck { deck: deck_entity },
        ColorIdentity::new(false, true, false, false, false), // Blue only
    )).id();
    
    let valid_card2 = app.world.spawn((
        InDeck { deck: deck_entity },
        ColorIdentity::new(false, false, false, false, true), // Green only
    )).id();
    
    // Invalid card (contains red)
    let invalid_card = app.world.spawn((
        InDeck { deck: deck_entity },
        ColorIdentity::new(false, true, false, true, true), // Blue-Red-Green
    )).id();
    
    // Run validation
    app.update();
    
    // Check validation results
    let events = app.world.resource::<Events<DeckValidationEvent>>();
    let mut reader = events.get_reader();
    
    let mut invalid_cards = Vec::new();
    for event in reader.read(&events) {
        if let DeckValidationEvent::InvalidCard { card, .. } = event {
            invalid_cards.push(*card);
        }
    }
    
    assert_eq!(invalid_cards.len(), 1);
    assert_eq!(invalid_cards[0], invalid_card);
}
```

## Summary

Color identity in Commander is implemented as a comprehensive system that:

1. Correctly calculates color identity from all relevant card components
2. Enforces deck construction rules based on the commander's color identity
3. Supports special cases like partner commanders and colorless commanders
4. Provides clear visual feedback in the UI
5. Is thoroughly tested with both unit and integration tests 
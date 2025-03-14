# Partner Commanders

This document details the implementation of partner commanders and related mechanics in the Commander format.

## Overview

Partner is a mechanic introduced in Commander 2016 that allows a deck to have two commanders instead of one. There are several variations of the partner mechanic:

1. **Traditional Partners**: Cards with the text "Partner" can pair with any other card that has Partner
2. **Partner With**: Cards with "Partner with [specific card]" can only pair with that specific card
3. **Background**: A commander that can have a Background enchantment as a second commander
4. **Friends Forever**: Cards with "Friends Forever" can pair with any other card that has Friends Forever

## Rules Implementation

### Core Partner Rules

The partner mechanic modifies several fundamental Commander rules:

1. Two commanders instead of one
2. Color identity is the combined colors of both commanders
3. Starting life total and commander damage tracking apply to each commander separately
4. Both commanders start in the command zone
5. Commander tax applies to each commander separately

### Implementation Details

```rust
/// Component marking an entity as a partner commander
#[derive(Component, Clone, Debug)]
pub enum PartnerType {
    /// Can partner with any other commander with Partner
    Universal,
    /// Can only partner with a specific commander
    Specific(Entity),
    /// Can have a Background as a partner
    CanHaveBackground,
    /// Is a Background enchantment
    IsBackground,
    /// Can partner with any Friends Forever commander
    FriendsForever,
}

/// Resource tracking partner commanders
#[derive(Resource)]
pub struct PartnerSystem {
    /// Maps players to their partner commanders
    pub player_partners: HashMap<Entity, Vec<Entity>>,
    /// Tracks which commanders are partnered together
    pub partnered_with: HashMap<Entity, Entity>,
}

/// System to validate partner legality during deck construction
pub fn validate_partner_legality(
    player: Entity,
    deck: &Deck,
    partners: &Query<(Entity, &CardName, &PartnerType)>,
) -> Result<(), DeckValidationError> {
    // Get commanders marked as partners
    let commander_entities = deck.get_commanders();
    
    // Filter to keep only entities with Partner component
    let partner_entities: Vec<Entity> = partners
        .iter_many(commander_entities)
        .map(|(entity, _, _)| entity)
        .collect();
    
    // Validate partner relationships
    match partner_entities.len() {
        0 => Ok(()), // No partners, standard single commander
        1 => Err(DeckValidationError::SinglePartnerNotAllowed), // Single partner needs a pair
        2 => validate_two_partners(&partner_entities, partners), // Check if these two can partner
        _ => Err(DeckValidationError::TooManyPartners), // More than 2 partners not allowed
    }
}

/// Validates if two commanders can be partners
fn validate_two_partners(
    entities: &[Entity],
    partners: &Query<(Entity, &CardName, &PartnerType)>,
) -> Result<(), DeckValidationError> {
    let (entity_a, name_a, type_a) = partners.get(entities[0]).unwrap();
    let (entity_b, name_b, type_b) = partners.get(entities[1]).unwrap();
    
    match (type_a, type_b) {
        // Universal partners can pair with any other universal partner
        (PartnerType::Universal, PartnerType::Universal) => Ok(()),
        
        // Specific partners can only pair with their named partner
        (PartnerType::Specific(target), _) if *target == entity_b => Ok(()),
        (_, PartnerType::Specific(target)) if *target == entity_a => Ok(()),
        
        // Background pairings
        (PartnerType::CanHaveBackground, PartnerType::IsBackground) => Ok(()),
        (PartnerType::IsBackground, PartnerType::CanHaveBackground) => Ok(()),
        
        // Friends Forever pairings
        (PartnerType::FriendsForever, PartnerType::FriendsForever) => Ok(()),
        
        // All other combinations are invalid
        _ => Err(DeckValidationError::InvalidPartnerCombination {
            commander_a: name_a.0.clone(),
            commander_b: name_b.0.clone(),
        }),
    }
}
```

## Color Identity with Partners

A deck with partner commanders uses the combined color identity of both commanders:

```rust
/// Calculate color identity for a deck with partners
pub fn calculate_partner_color_identity(
    commanders: &[Entity],
    identity_query: &Query<&ColorIdentity>,
) -> ColorIdentity {
    let mut combined_identity = ColorIdentity::default();
    
    for commander in commanders {
        if let Ok(identity) = identity_query.get(*commander) {
            combined_identity = combined_identity.union(identity);
        }
    }
    
    combined_identity
}
```

## Partner Variations

### Partner With

The "Partner with" mechanic has additional functionality beyond allowing two commanders:

1. When either commander enters the battlefield, its controller may search their library for the other
2. This tutor effect requires special implementation

```rust
/// Component for the "Partner with" tutoring ability
#[derive(Component)]
pub struct PartnerWithTutorAbility {
    pub partner_name: String,
}

/// System that handles the "Partner with" tutoring ability
pub fn handle_partner_with_tutor(
    mut commands: Commands,
    mut entered_battlefield: EventReader<EnteredBattlefieldEvent>,
    tutor_abilities: Query<(&PartnerWithTutorAbility, &Owner)>,
    mut tutor_events: EventWriter<TutorEvent>,
) {
    for event in entered_battlefield.read() {
        if let Ok((ability, owner)) in tutor_abilities.get(event.entity) {
            // Create a tutor effect allowing player to search for the partner
            tutor_events.send(TutorEvent {
                player: owner.0,
                card_name: ability.partner_name.clone(),
                origin: event.entity,
                destination: Zone::Hand,
                optional: true,
            });
        }
    }
}
```

### Background

The Background mechanic, introduced in Commander Legends: Battle for Baldur's Gate, allows certain commanders to have a Background enchantment as their second commander:

```rust
/// Component marking a card as a Background
#[derive(Component)]
pub struct Background;

/// Component for commanders that can have a Background
#[derive(Component)]
pub struct CanHaveBackground;

/// System to validate Background legality
pub fn validate_background_legality(
    commanders: &[Entity],
    can_have_query: &Query<Entity, With<CanHaveBackground>>,
    background_query: &Query<Entity, With<Background>>,
) -> Result<(), DeckValidationError> {
    if commanders.len() != 2 {
        return Ok(());
    }
    
    let has_commander_with_background = can_have_query
        .iter_many(commanders)
        .count() == 1;
        
    let has_background = background_query
        .iter_many(commanders)
        .count() == 1;
    
    if has_commander_with_background && has_background {
        Ok(())
    } else if has_commander_with_background || has_background {
        Err(DeckValidationError::IncompleteBackgroundPairing)
    } else {
        Ok(()) // Not using Background mechanic
    }
}
```

## User Interface Considerations

The UI needs special handling for partner commanders:

1. Both commanders need to be displayed in the command zone
2. Players need a way to choose which commander to cast
3. Commander tax display needs to track each commander separately

## Testing Partner Mechanics

```rust
#[test]
fn test_universal_partners() {
    let mut app = App::new();
    app.add_systems(Startup, setup_test_partners);
    app.add_systems(Update, validate_partner_legality);
    
    // Test universal partners (e.g., Thrasios and Tymna)
    let thrasios = app.world.spawn((
        CardName("Thrasios, Triton Hero".to_string()),
        PartnerType::Universal,
    )).id();
    
    let tymna = app.world.spawn((
        CardName("Tymna the Weaver".to_string()),
        PartnerType::Universal,
    )).id();
    
    let deck = create_test_deck(vec![thrasios, tymna]);
    let result = validate_deck(&app.world, deck);
    assert!(result.is_ok());
}

#[test]
fn test_partners_with() {
    // Test for "Partner with" mechanic (e.g., Brallin and Shabraz)
    // Implementation details...
}

#[test]
fn test_background() {
    // Test for Background mechanic
    // Implementation details...
}
```

## Related Documentation

- [Commander Damage](../combat/commander_damage.md): How commander damage is tracked with partner commanders
- [Command Zone](../zones/command_zone.md): How partners behave in the command zone
- [Commander Tax](../player_mechanics/commander_tax.md): How tax is applied to each partner
- [Color Identity](../player_mechanics/color_identity.md): How color identity is calculated with partners 
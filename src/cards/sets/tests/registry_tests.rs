use crate::cards::rarity::Rarity;
use crate::cards::set::CardSet;
use crate::cards::sets::CardRegistry;
use crate::cards::{Card, CardDetails, CardTypes};
use crate::mana::Mana;
use bevy::prelude::*;

/// Test function demonstrating how to use the registry functions
#[test]
fn test_card_registry() {
    // Create a new registry
    let mut registry = CardRegistry::default();

    // Create a set
    let alpha_set = CardSet {
        code: "LEA".to_string(),
        name: "Limited Edition Alpha".to_string(),
        release_date: "1993-08-05".to_string(),
    };

    // Create an app to spawn entities
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Spawn some test cards with default Mana
    let lightning_bolt = app
        .world_mut()
        .spawn(
            Card::builder("Lightning Bolt")
                .cost(Mana::default())
                .types(CardTypes::INSTANT)
                .details(CardDetails::default())
                .rules_text("Lightning Bolt deals 3 damage to any target.")
                .build_or_panic(),
        )
        .id();

    let serra_angel = app
        .world_mut()
        .spawn(
            Card::builder("Serra Angel")
                .cost(Mana::default())
                .types(CardTypes::CREATURE)
                .details(CardDetails::Creature(crate::cards::details::CreatureCard {
                    power: 4,
                    toughness: 4,
                    creature_type: crate::cards::types::CreatureType::ANGEL,
                }))
                .rules_text("Flying, vigilance")
                .build_or_panic(),
        )
        .id();

    let black_lotus = app
        .world_mut()
        .spawn(
            Card::builder("Black Lotus")
                .cost(Mana::default())
                .types(CardTypes::ARTIFACT)
                .details(CardDetails::default())
                .rules_text("{T}, Sacrifice Black Lotus: Add three mana of any one color.")
                .build_or_panic(),
        )
        .id();

    // Get the card components
    let lightning_bolt_card = app
        .world()
        .entity(lightning_bolt)
        .get::<Card>()
        .unwrap()
        .clone();
    let serra_angel_card = app
        .world()
        .entity(serra_angel)
        .get::<Card>()
        .unwrap()
        .clone();
    let black_lotus_card = app
        .world()
        .entity(black_lotus)
        .get::<Card>()
        .unwrap()
        .clone();

    // Register the cards
    registry.register_card(
        lightning_bolt,
        &lightning_bolt_card,
        &alpha_set,
        Rarity::Common,
    );
    registry.register_card(serra_angel, &serra_angel_card, &alpha_set, Rarity::Uncommon);
    registry.register_card(black_lotus, &black_lotus_card, &alpha_set, Rarity::Rare);

    // Test getting cards by various criteria
    let alpha_cards = registry.get_set_cards("LEA").unwrap();
    assert_eq!(alpha_cards.len(), 3);

    // Test registry system initialization
    app.insert_resource(registry);
    let system_check = move |registry: Res<CardRegistry>| {
        assert_eq!(registry.get_set_cards("LEA").unwrap().len(), 3);
    };

    app.add_systems(Update, system_check);
    app.update();
}

#[test]
fn test_registry_systems() {
    // Create an app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add registry initialization system
    app.add_systems(Startup, crate::cards::sets::systems::init_card_registry);
    app.add_systems(Update, crate::cards::sets::systems::register_card);

    // Update to run startup systems
    app.update();

    // Check that the registry was created
    assert!(app.world().contains_resource::<CardRegistry>());

    // Create a new card entity directly in the world
    let mountain = app
        .world_mut()
        .spawn((
            Card::builder("Mountain")
                .cost(Mana::default())
                .types(CardTypes::LAND)
                .details(CardDetails::default())
                .rules_text("{T}: Add {R}.")
                .build_or_panic(),
            CardSet {
                code: "LEA".to_string(),
                name: "Limited Edition Alpha".to_string(),
                release_date: "1993-08-05".to_string(),
            },
            Rarity::Common,
        ))
        .id();

    // Update to run the register system
    app.update();

    // Check that the card was registered
    let registry = app.world().resource::<CardRegistry>();
    assert!(registry.get_set_cards("LEA").is_some());
    let lea_cards = registry.get_set_cards("LEA").unwrap();
    assert_eq!(lea_cards.len(), 1);
    assert_eq!(lea_cards[0], mountain);
}

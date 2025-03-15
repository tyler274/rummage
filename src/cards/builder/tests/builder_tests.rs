use crate::cards::{Card, builder::CardBuilder, details::CardDetails, types::CardTypes};
use crate::mana::{Color, Mana};
use bevy::prelude::*;

/// Test the basic functionality of the CardBuilder
#[test]
fn test_builder_basic_functionality() {
    // Create a basic creature card using the builder
    let card = Card::builder("Test Creature")
        .cost(Mana::new_with_colors(2, 1, 0, 0, 0, 0))
        .types(CardTypes::new_creature(vec![
            "Elf".to_string(),
            "Warrior".to_string(),
        ]))
        .details(CardDetails::new_creature(3, 3))
        .rules_text("Vigilance (Attacking doesn't cause this creature to tap.)")
        .build_or_panic();

    // Check that all properties were correctly set
    assert_eq!(Card::get_name(&card), "Test Creature");

    let cost = Card::get_cost(&card);
    assert_eq!(cost.converted_mana_cost(), 3);
    assert_eq!(cost.colored_mana_cost(Color::COLORLESS), 2);
    assert_eq!(cost.colored_mana_cost(Color::WHITE), 1);

    let types = Card::get_types(&card);
    assert!(types.is_creature());
    assert_eq!(types.get_creature_types().len(), 2);
    assert!(types.get_creature_types().contains(&"Elf".to_string()));
    assert!(types.get_creature_types().contains(&"Warrior".to_string()));

    let details = Card::get_details(&card);
    if let CardDetails::Creature(creature_details) = details {
        assert_eq!(creature_details.power, 3);
        assert_eq!(creature_details.toughness, 3);
    } else {
        panic!("Expected creature details");
    }

    assert!(Card::get_rules_text(&card).contains("Vigilance"));
    assert!(Card::has_keyword(
        &card,
        crate::cards::keywords::KeywordAbility::Vigilance
    ));
}

/// Test error handling when required fields are missing
#[test]
fn test_builder_missing_fields() {
    // Missing cost
    let result = Card::builder("Test Card")
        .types(CardTypes::new_creature(vec!["Human".to_string()]))
        .details(CardDetails::new_creature(1, 1))
        .build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must have a mana cost"));

    // Missing types
    let result = Card::builder("Test Card")
        .cost(Mana::new_with_colors(1, 0, 0, 0, 0, 0))
        .details(CardDetails::new_creature(1, 1))
        .build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must have types"));

    // Missing details
    let result = Card::builder("Test Card")
        .cost(Mana::new_with_colors(1, 0, 0, 0, 0, 0))
        .types(CardTypes::new_creature(vec!["Human".to_string()]))
        .build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must have details"));
}

/// Test building different card types
#[test]
fn test_builder_different_card_types() {
    // Instant card
    let instant = Card::builder("Test Instant")
        .cost(Mana::new_with_colors(1, 0, 0, 1, 0, 0))
        .types(CardTypes::new_instant())
        .details(CardDetails::Other)
        .rules_text("Counter target spell.")
        .build_or_panic();

    assert!(Card::has_type(&instant, CardTypes::TYPE_INSTANT));
    assert!(!Card::has_type(&instant, CardTypes::TYPE_CREATURE));

    // Sorcery card
    let sorcery = Card::builder("Test Sorcery")
        .cost(Mana::new_with_colors(2, 0, 1, 0, 0, 0))
        .types(CardTypes::new_sorcery())
        .details(CardDetails::Other)
        .rules_text("Draw two cards.")
        .build_or_panic();

    assert!(Card::has_type(&sorcery, CardTypes::TYPE_SORCERY));

    // Enchantment card
    let enchantment = Card::builder("Test Enchantment")
        .cost(Mana::new_with_colors(2, 1, 1, 0, 0, 0))
        .types(CardTypes::new_enchantment())
        .details(CardDetails::Other)
        .rules_text("At the beginning of your upkeep, draw a card.")
        .build_or_panic();

    assert!(Card::has_type(&enchantment, CardTypes::TYPE_ENCHANTMENT));
}

/// Test that empty rules text doesn't cause problems
#[test]
fn test_builder_empty_rules_text() {
    let vanilla = Card::builder("Vanilla Creature")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 0, 0))
        .types(CardTypes::new_creature(vec!["Beast".to_string()]))
        .details(CardDetails::new_creature(4, 4))
        .build_or_panic();

    assert_eq!(Card::get_rules_text(&vanilla), "");
    assert!(Card::get_types(&vanilla).is_creature());
}

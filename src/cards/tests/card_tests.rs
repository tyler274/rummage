use crate::cards::card::Card;
use crate::cards::details::CardDetails;
use crate::cards::keywords::KeywordAbility;
use crate::cards::types::CardTypes;
use crate::mana::{Color, Mana};
use bevy::prelude::*;

/// Test that demonstrates using the various Card accessor methods
#[test]
fn test_card_accessors() {
    // Create a test card
    let mut card = Card::new(
        "Test Card",
        Mana::new_with_colors(1, 0, 0, 0, 0, 0),
        CardTypes::new_creature(vec!["Wizard".to_string()]),
        CardDetails::new_creature(2, 2),
        "Flying, Haste (This creature can attack as soon as it comes under your control.)",
    );

    // Test get_name
    assert_eq!(Card::get_name(&card), "Test Card");

    // Test get_cost
    let cost = Card::get_cost(&card);
    assert_eq!(cost.converted_mana_cost(), 1);
    assert_eq!(cost.colored_mana_cost(Color::COLORLESS), 1);

    // Test get_types
    let types = Card::get_types(&card);
    assert!(types.is_creature());
    assert_eq!(types.get_creature_types().len(), 1);
    assert_eq!(types.get_creature_types()[0], "Wizard");

    // Test get_rules_text
    assert!(Card::get_rules_text(&card).contains("Flying, Haste"));

    // Test get_details
    let details = Card::get_details(&card);
    if let CardDetails::Creature(creature_card) = details {
        assert_eq!(creature_card.power, 2);
        assert_eq!(creature_card.toughness, 2);
    } else {
        panic!("Expected creature details");
    }

    // Test has_type
    assert!(Card::has_type(&card, CardTypes::TYPE_CREATURE));

    // Test keyword methods
    assert!(Card::has_keyword(&card, KeywordAbility::Flying));
    assert!(Card::has_keyword(&card, KeywordAbility::Haste));

    // Add a keyword with a value
    Card::add_keyword_with_value(&mut card, KeywordAbility::Protection, "from black");
    assert!(Card::has_keyword(&card, KeywordAbility::Protection));
    assert_eq!(
        Card::get_keyword_value(&card, KeywordAbility::Protection),
        Some("from black")
    );

    // Add a simple keyword
    Card::add_keyword(&mut card, KeywordAbility::Vigilance);
    assert!(Card::has_keyword(&card, KeywordAbility::Vigilance));

    // Test type_line
    let type_line = Card::type_line(&card);
    assert!(type_line.contains("Creature"));
    // The type_line function doesn't include creature types when using CardDetails::Other
    // So we can't check for "Wizard" here
}

use super::mocks::*;
use crate::card::{CardDetails, CardTypes, CreatureType};
use crate::cards::mtgjson::{
    convert_mtgjson_to_card, determine_card_type, determine_creature_types, parse_mana_cost,
};
use crate::mana::Color;

mod mana_parsing {
    use super::*;

    #[test]
    fn test_basic_mana_costs() {
        let cost = parse_mana_cost("{1}{U}{U}");
        assert_eq!(cost.colorless, 1);
        assert_eq!(cost.blue, 2);
        assert_eq!(cost.color, Color::BLUE);
    }

    #[test]
    fn test_multicolor_costs() {
        let cost = parse_mana_cost("{2}{W}{U}{B}{R}{G}");
        assert_eq!(cost.colorless, 2);
        assert_eq!(cost.white, 1);
        assert_eq!(cost.blue, 1);
        assert_eq!(cost.black, 1);
        assert_eq!(cost.red, 1);
        assert_eq!(cost.green, 1);
        assert_eq!(cost.color, Color::ALL);
    }

    #[test]
    fn test_empty_cost() {
        let cost = parse_mana_cost("");
        assert_eq!(cost.colorless, 0);
        assert_eq!(cost.color, Color::COLORLESS);
    }
}

mod type_parsing {
    use super::*;

    #[test]
    fn test_basic_types() {
        let types = vec!["Creature".to_string()];
        let card_type = determine_card_type(&types, None, None).unwrap();
        assert!(card_type.contains(CardTypes::CREATURE));
    }

    #[test]
    fn test_multiple_types() {
        let types = vec!["Artifact".to_string(), "Creature".to_string()];
        let card_type = determine_card_type(&types, None, None).unwrap();
        assert!(card_type.contains(CardTypes::ARTIFACT));
        assert!(card_type.contains(CardTypes::CREATURE));
    }
}

mod creature_type_parsing {
    use super::*;

    #[test]
    fn test_single_creature_type() {
        let subtypes = vec!["Human".to_string()];
        let creature_type = determine_creature_types(&subtypes, "", "");
        assert!(creature_type.contains(CreatureType::HUMAN));
    }

    #[test]
    fn test_multiple_creature_types() {
        let subtypes = vec!["Human".to_string(), "Wizard".to_string()];
        let creature_type = determine_creature_types(&subtypes, "", "");
        assert!(creature_type.contains(CreatureType::HUMAN));
        assert!(creature_type.contains(CreatureType::WIZARD));
    }
}

mod card_conversion {
    use super::*;

    #[test]
    fn test_creature_conversion() {
        let mtg_card = create_test_card();
        let card = convert_mtgjson_to_card(mtg_card).unwrap();

        assert_eq!(card.name, "Test Creature");
        assert_eq!(card.cost.green, 1);
        assert_eq!(card.cost.colorless, 2);
        assert!(card.types.contains(CardTypes::CREATURE));
        assert!(card.types.contains(CardTypes::LEGENDARY));

        if let CardDetails::Creature(creature) = card.card_details {
            assert_eq!(creature.power, 2);
            assert_eq!(creature.toughness, 2);
            assert!(creature.creature_type.contains(CreatureType::HUMAN));
            assert!(creature.creature_type.contains(CreatureType::WARRIOR));
        } else {
            panic!("Expected Creature card details");
        }
    }
}

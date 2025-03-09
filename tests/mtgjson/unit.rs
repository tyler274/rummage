use rummage::card::{CardDetails, CardTypes, CreatureType};

use rummage::cards::mtgjson::{MTGClientType, MTGService, test_utils};
use rummage::cards::mtgjson::{
    convert_mtgjson_to_card, determine_card_type, determine_creature_types, parse_mana_cost,
    test_utils::create_test_mtgjson_card,
};
use rummage::mana::Color;

use std::sync::Arc;
use tokio;

#[test]
fn test_parse_mana_cost_simple() {
    let cost = parse_mana_cost("{1}{U}{U}");
    assert_eq!(cost.colorless, 1);
    assert_eq!(cost.blue, 2);
    assert_eq!(cost.color, Color::BLUE);
}

#[test]
fn test_parse_mana_cost_wubrg() {
    let cost = parse_mana_cost("{2}{W}{U}{B}{R}{G}");
    assert_eq!(cost.colorless, 2);
    assert_eq!(cost.white, 1);
    assert_eq!(cost.blue, 1);
    assert_eq!(cost.black, 1);
    assert_eq!(cost.red, 1);
    assert_eq!(cost.green, 1);
    assert_eq!(
        cost.color,
        Color::WHITE | Color::BLUE | Color::BLACK | Color::RED | Color::GREEN
    );
}

#[test]
fn test_parse_mana_cost_empty() {
    let cost = parse_mana_cost("");
    assert_eq!(cost.colorless, 0);
    assert_eq!(cost.white, 0);
    assert_eq!(cost.blue, 0);
    assert_eq!(cost.black, 0);
    assert_eq!(cost.red, 0);
    assert_eq!(cost.green, 0);
    assert_eq!(cost.color, Color::COLORLESS);
}

#[test]
fn test_determine_card_type_creature() {
    let types = vec!["Creature".to_string()];
    let card_type = determine_card_type(&types, None, None).unwrap();
    assert!(card_type.contains(CardTypes::CREATURE));
}

#[test]
fn test_determine_card_type_legendary_creature() {
    let types = vec!["Creature".to_string()];
    let supertypes = vec!["Legendary".to_string()];
    let card_type = determine_card_type(&types, Some(&supertypes), None).unwrap();
    assert!(card_type.contains(CardTypes::CREATURE));
    assert!(card_type.contains(CardTypes::LEGENDARY));
}

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

#[test]
fn test_convert_mtgjson_to_card() {
    let mtg_card = create_test_mtgjson_card();
    let card = convert_mtgjson_to_card(mtg_card).unwrap();
    assert_eq!(card.name, "Test Creature");
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

#[tokio::test]
async fn test_fetch_basic_land_set() {
    let mock_client = Arc::new(test_utils::MockClient::new());
    let mock_set = test_utils::create_mock_set();
    let mock_meta = test_utils::create_mock_meta();

    mock_client.mock_response("30A", mock_set).await;
    mock_client.mock_meta(mock_meta).await;

    let service = MTGService::new(MTGClientType::Mock(mock_client));
    let result = service.fetch_set("30A").await;
    assert!(result.is_ok(), "Failed to fetch 30A: {:?}", result.err());

    let cards = result.unwrap();
    assert!(!cards.is_empty(), "Set should contain cards");

    let mut basic_land_types = 0;
    for card in &cards {
        if card.types.contains(CardTypes::BASIC) && card.types.contains(CardTypes::LAND) {
            basic_land_types += 1;
            assert_eq!(card.cost.total(), 0, "Basic lands should have no mana cost");
        }
    }

    assert!(basic_land_types >= 5, "Should have all basic land types");
}

#[tokio::test]
async fn test_fetch_modern_set() {
    let mock_client = Arc::new(test_utils::MockClient::new());
    let mock_set = test_utils::create_mock_set();
    let mock_meta = test_utils::create_mock_meta();

    mock_client.mock_response("MH2", mock_set).await;
    mock_client.mock_meta(mock_meta).await;

    let service = MTGService::new(MTGClientType::Mock(mock_client));
    let result = service.fetch_set("MH2").await;
    assert!(result.is_ok(), "Failed to fetch MH2: {:?}", result.err());

    let cards = result.unwrap();
    assert!(!cards.is_empty(), "Set should contain cards");
    assert!(cards.len() > 0, "Set should have cards");
}

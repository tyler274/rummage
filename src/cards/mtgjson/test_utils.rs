use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{MTGClient, MTGJSONCard, MTGJSONCardIdentifiers, MTGJSONMeta, MTGJSONSet};
use crate::cards::{
    Card, CardCost, CardDetails, CardDetailsComponent, CardKeywords, CardName, CardRulesText,
    CardTypeInfo, CardTypes, CreatureCard, CreatureType,
};
use crate::mana::Mana;
use bevy::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MockClient {
    pub sets: Arc<RwLock<HashMap<String, MTGJSONSet>>>,
    pub meta: Arc<RwLock<Option<MTGJSONMeta>>>,
}

#[allow(dead_code)]
impl Default for MockClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            sets: Arc::new(RwLock::new(HashMap::new())),
            meta: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn mock_response(&self, set_code: &str, set: MTGJSONSet) {
        let mut sets = self.sets.write().unwrap();
        sets.insert(set_code.to_string(), set);
    }

    pub async fn mock_meta(&self, meta: MTGJSONMeta) {
        let mut meta_guard = self.meta.write().unwrap();
        *meta_guard = Some(meta);
    }

    pub async fn add_set(&self, set_code: &str, cards: Vec<MTGJSONCard>) {
        let set = MTGJSONSet {
            artist_ids: Some(vec![]),
            availability: vec!["paper".to_string()],
            cards,
            code: set_code.to_string(),
            name: format!("Test Set {}", set_code),
            total_set_size: 0,
            release_date: "2023-01-01".to_string(),
            type_: "expansion".to_string(),
            uuid: Some("test-uuid".to_string()),
            languages: vec!["en".to_string()],
            booster: None,
            sealed_product: None,
            tokens: None,
            translations: None,
            base_set_size: Some(0),
            block: None,
            is_foreign_only: Some(false),
            is_partial_preview: Some(false),
            is_online_only: Some(false),
            keyrunecode: None,
            mcm_id: None,
            mcm_name: None,
            mtgo_code: None,
            tcgplayer_group_id: None,
            meta: None,
        };
        self.mock_response(set_code, set).await;
    }
}

#[async_trait]
impl MTGClient for MockClient {
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>> {
        let sets = self.sets.read().unwrap();
        sets.get(set_code)
            .cloned()
            .ok_or_else(|| format!("No mock response for set {}", set_code).into())
    }
}

#[allow(dead_code)]
pub fn create_test_card() -> (
    Card,
    CardName,
    CardCost,
    CardTypeInfo,
    CardDetailsComponent,
    CardRulesText,
    CardKeywords,
) {
    let card = Card::new(
        "Test Card",
        Mana::default(),
        CardTypes::CREATURE,
        CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::NONE,
        }),
        "Test rules text",
    );

    // Return the card and its individual components
    card.get_components()
}

/// Creates a test card entity with all components
#[allow(dead_code)]
pub fn spawn_test_card(commands: &mut Commands) -> Entity {
    let card = Card::new(
        "Test Card",
        Mana::default(),
        CardTypes::CREATURE,
        CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::NONE,
        }),
        "Test rules text",
    );

    // Spawn the card directly
    commands.spawn(card).insert(Name::new("Test Card")).id()
}

#[allow(dead_code)]
pub fn create_test_mtgjson_card() -> MTGJSONCard {
    MTGJSONCard {
        artist: None,
        artist_ids: None,
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec!["G".to_string()],
        colors: Some(vec!["G".to_string()]),
        converted_mana_cost: Some(3.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "modern".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: Some("{2}{G}".to_string()),
        mana_value: Some(3.0),
        name: "Test Creature".to_string(),
        number: "1".to_string(),
        power: Some("2".to_string()),
        printings: vec!["TEST".to_string()],
        purchase_urls: None,
        rarity: "rare".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TEST".to_string(),
        source_products: None,
        subtypes: vec!["Human".to_string(), "Warrior".to_string()],
        supertypes: vec!["Legendary".to_string()],
        text: Some("Test rules text".to_string()),
        toughness: Some("2".to_string()),
        type_: "Legendary Creature — Human Warrior".to_string(),
        types: vec!["Creature".to_string()],
        uuid: "test-uuid".to_string(),
        variations: None,
    }
}

pub fn mock_basic_land(name: String, subtypes: Vec<String>) -> MTGJSONCard {
    let uuid = format!("test-uuid-{}", name.to_lowercase());
    MTGJSONCard {
        artist: Some("Test Artist".to_string()),
        artist_ids: Some(vec!["test-artist-id".to_string()]),
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec![],
        colors: Some(vec![]),
        converted_mana_cost: Some(0.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "2015".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: None,
        mana_value: Some(0.0),
        name,
        number: "1".to_string(),
        power: None,
        printings: vec![],
        purchase_urls: None,
        rarity: "basic".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TST".to_string(),
        source_products: None,
        subtypes,
        supertypes: vec!["Basic".to_string()],
        text: None,
        toughness: None,
        type_: "Basic Land".to_string(),
        types: vec!["Land".to_string()],
        uuid,
        variations: None,
    }
}

pub fn _mock_creature(name: &str, power: i32, toughness: i32) -> MTGJSONCard {
    MTGJSONCard {
        artist: Some("Test Artist".to_string()),
        artist_ids: Some(vec!["test-artist-id".to_string()]),
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec!["G".to_string()],
        colors: Some(vec!["G".to_string()]),
        converted_mana_cost: Some(2.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "2015".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: Some("{1}{G}".to_string()),
        mana_value: Some(2.0),
        name: name.to_string(),
        number: "1".to_string(),
        power: Some(power.to_string()),
        printings: vec![],
        purchase_urls: None,
        rarity: "common".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TST".to_string(),
        source_products: None,
        subtypes: vec!["Beast".to_string()],
        supertypes: vec![],
        text: None,
        toughness: Some(toughness.to_string()),
        type_: "Creature — Beast".to_string(),
        types: vec!["Creature".to_string()],
        uuid: format!("test-uuid-{}", name.to_lowercase()),
        variations: None,
    }
}

pub fn mock_instant(name: &str) -> MTGJSONCard {
    MTGJSONCard {
        artist: Some("Test Artist".to_string()),
        artist_ids: Some(vec!["test-artist-id".to_string()]),
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec!["U".to_string()],
        colors: Some(vec!["U".to_string()]),
        converted_mana_cost: Some(2.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "2015".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: Some("{1}{U}".to_string()),
        mana_value: Some(2.0),
        name: name.to_string(),
        number: "1".to_string(),
        power: None,
        printings: vec![],
        purchase_urls: None,
        rarity: "common".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TST".to_string(),
        source_products: None,
        subtypes: vec![],
        supertypes: vec![],
        text: Some("Counter target spell.".to_string()),
        toughness: None,
        type_: "Instant".to_string(),
        types: vec!["Instant".to_string()],
        uuid: format!("test-uuid-{}", name.to_lowercase()),
        variations: None,
    }
}

pub fn mock_sorcery(name: &str) -> MTGJSONCard {
    MTGJSONCard {
        artist: Some("Test Artist".to_string()),
        artist_ids: Some(vec!["test-artist-id".to_string()]),
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec!["R".to_string()],
        colors: Some(vec!["R".to_string()]),
        converted_mana_cost: Some(2.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "2015".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: Some("{1}{R}".to_string()),
        mana_value: Some(2.0),
        name: name.to_string(),
        number: "1".to_string(),
        power: None,
        printings: vec![],
        purchase_urls: None,
        rarity: "common".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TST".to_string(),
        source_products: None,
        subtypes: vec![],
        supertypes: vec![],
        text: Some("Deal 3 damage to any target.".to_string()),
        toughness: None,
        type_: "Sorcery".to_string(),
        types: vec!["Sorcery".to_string()],
        uuid: format!("test-uuid-{}", name.to_lowercase()),
        variations: None,
    }
}

#[allow(dead_code)]
pub fn create_mock_set() -> MTGJSONSet {
    let mut cards = vec![create_test_mtgjson_card()];

    // Add an instant card
    cards.push(mock_instant("Test Instant"));

    // Add a sorcery card
    cards.push(mock_sorcery("Test Sorcery"));

    // Add basic lands
    let basic_land_types = [
        ("Plains".to_string(), vec!["Plains".to_string()]),
        ("Island".to_string(), vec!["Island".to_string()]),
        ("Swamp".to_string(), vec!["Swamp".to_string()]),
        ("Mountain".to_string(), vec!["Mountain".to_string()]),
        ("Forest".to_string(), vec!["Forest".to_string()]),
    ];

    for (name, subtypes) in basic_land_types {
        cards.push(mock_basic_land(name, subtypes));
    }

    MTGJSONSet {
        artist_ids: Some(vec![]),
        availability: vec!["paper".to_string()],
        cards,
        code: "TEST".to_string(),
        name: "Test Set".to_string(),
        total_set_size: 8,
        release_date: "2024-03-21".to_string(),
        type_: "expansion".to_string(),
        uuid: Some("test-uuid".to_string()),
        languages: vec!["en".to_string()],
        booster: None,
        sealed_product: None,
        tokens: None,
        translations: None,
        base_set_size: Some(8),
        block: None,
        is_foreign_only: Some(false),
        is_partial_preview: Some(false),
        is_online_only: Some(false),
        keyrunecode: None,
        mcm_id: None,
        mcm_name: None,
        mtgo_code: None,
        tcgplayer_group_id: None,
        meta: None,
    }
}

#[allow(dead_code)]
pub fn create_mock_meta() -> MTGJSONMeta {
    MTGJSONMeta {
        version: "5.2.1".to_string(),
        date: "2024-03-21".to_string(),
        checksums: HashMap::new(),
    }
}

pub fn _mock_basic_land_set() -> MTGJSONSet {
    MTGJSONSet {
        artist_ids: Some(vec![]),
        availability: vec!["paper".to_string()],
        cards: vec![
            mock_basic_land("Plains".to_string(), vec!["Plains".to_string()]),
            mock_basic_land("Island".to_string(), vec!["Island".to_string()]),
            mock_basic_land("Swamp".to_string(), vec!["Swamp".to_string()]),
            mock_basic_land("Mountain".to_string(), vec!["Mountain".to_string()]),
            mock_basic_land("Forest".to_string(), vec!["Forest".to_string()]),
        ],
        code: "30A".to_string(),
        name: "30th Anniversary Edition".to_string(),
        total_set_size: 5,
        release_date: "2023-01-01".to_string(),
        type_: "masters".to_string(),
        uuid: Some("test-uuid".to_string()),
        languages: vec!["en".to_string()],
        booster: None,
        sealed_product: None,
        tokens: None,
        translations: None,
        base_set_size: Some(5),
        block: None,
        is_foreign_only: Some(false),
        is_partial_preview: Some(false),
        is_online_only: Some(false),
        keyrunecode: None,
        mcm_id: None,
        mcm_name: None,
        mtgo_code: None,
        tcgplayer_group_id: None,
        meta: None,
    }
}

pub fn _mock_modern_horizons_set() -> MTGJSONSet {
    MTGJSONSet {
        artist_ids: Some(vec![]),
        availability: vec!["paper".to_string()],
        cards: vec![
            _mock_creature("Test Creature", 2, 2),
            mock_instant("Test Instant"),
            mock_sorcery("Test Sorcery"),
        ],
        code: "MH2".to_string(),
        name: "Modern Horizons 2".to_string(),
        total_set_size: 3,
        release_date: "2023-01-01".to_string(),
        type_: "masters".to_string(),
        uuid: Some("test-uuid".to_string()),
        languages: vec!["en".to_string()],
        booster: None,
        sealed_product: None,
        tokens: None,
        translations: None,
        base_set_size: Some(3),
        block: None,
        is_foreign_only: Some(false),
        is_partial_preview: Some(false),
        is_online_only: Some(false),
        keyrunecode: None,
        mcm_id: None,
        mcm_name: None,
        mtgo_code: None,
        tcgplayer_group_id: None,
        meta: None,
    }
}

pub fn _mock_set(cards: Vec<MTGJSONCard>) -> MTGJSONSet {
    MTGJSONSet {
        artist_ids: Some(vec![]),
        availability: vec!["paper".to_string()],
        cards,
        code: "TST".to_string(),
        name: "Test Set".to_string(),
        total_set_size: 0,
        release_date: "2023-01-01".to_string(),
        type_: "expansion".to_string(),
        uuid: Some("test-uuid".to_string()),
        languages: vec!["en".to_string()],
        booster: None,
        sealed_product: None,
        tokens: None,
        translations: None,
        base_set_size: Some(0),
        block: None,
        is_foreign_only: Some(false),
        is_partial_preview: Some(false),
        is_online_only: Some(false),
        keyrunecode: None,
        mcm_id: None,
        mcm_name: None,
        mtgo_code: None,
        tcgplayer_group_id: None,
        meta: None,
    }
}

pub fn _mock_card() -> MTGJSONCard {
    MTGJSONCard {
        artist: Some("Test Artist".to_string()),
        artist_ids: Some(vec!["test-artist-id".to_string()]),
        availability: vec!["paper".to_string()],
        border_color: "black".to_string(),
        color_identity: vec![],
        colors: Some(vec![]),
        converted_mana_cost: Some(0.0),
        edhrec_rank: None,
        finishes: vec!["nonfoil".to_string()],
        foreign_data: None,
        frame_version: "2015".to_string(),
        has_foil: false,
        has_non_foil: true,
        identifiers: MTGJSONCardIdentifiers {
            card_kingdom_id: None,
            card_kingdom_foil_id: None,
            mtgjson_v4_id: None,
            scryfall_card_back_id: None,
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_oracle_id: None,
            tcgplayer_product_id: None,
        },
        is_reprint: Some(false),
        is_starter: None,
        keywords: None,
        language: "English".to_string(),
        layout: "normal".to_string(),
        legalities: HashMap::new(),
        mana_cost: Some("{0}".to_string()),
        mana_value: Some(0.0),
        name: "Test Card".to_string(),
        number: "1".to_string(),
        power: None,
        printings: vec![],
        purchase_urls: None,
        rarity: "common".to_string(),
        rulings: None,
        security_stamp: None,
        set_code: "TST".to_string(),
        source_products: None,
        subtypes: vec![],
        supertypes: vec![],
        text: None,
        toughness: None,
        type_: "Card".to_string(),
        types: vec![],
        uuid: "test-uuid".to_string(),
        variations: None,
    }
}

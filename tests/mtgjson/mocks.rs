use crate::cards::mtgjson::{
    MTGClient, MTGJSONCard, MTGJSONMeta, MTGJSONSet, MTGJSONSetData, MTGJSONSetMeta,
};
use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MockClient {
    responses: Arc<Mutex<HashMap<String, MTGJSONSet>>>,
    meta: Arc<Mutex<Option<MTGJSONMeta>>>,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            meta: Arc::new(Mutex::new(None)),
        }
    }

    pub fn mock_response(&self, set_code: &str, set: MTGJSONSet) {
        self.responses
            .lock()
            .unwrap()
            .insert(set_code.to_string(), set);
    }

    pub fn mock_meta(&self, meta: MTGJSONMeta) {
        *self.meta.lock().unwrap() = Some(meta);
    }
}

#[async_trait]
impl MTGClient for MockClient {
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>> {
        // First check if we have metadata
        if self.meta.lock().unwrap().is_none() {
            return Err("No metadata available".into());
        }

        self.responses
            .lock()
            .unwrap()
            .get(set_code)
            .cloned()
            .ok_or_else(|| "Set not found".into())
    }
}

pub fn create_test_card() -> MTGJSONCard {
    MTGJSONCard {
        name: "Test Creature".to_string(),
        mana_cost: Some("{2}{G}".to_string()),
        text: Some("Test rules text".to_string()),
        types: vec!["Creature".to_string()],
        supertypes: Some(vec!["Legendary".to_string()]),
        subtypes: Some(vec!["Human".to_string(), "Warrior".to_string()]),
        colors: Some(vec!["G".to_string()]),
        power: Some("2".to_string()),
        toughness: Some("2".to_string()),
    }
}

pub fn create_mock_set() -> MTGJSONSet {
    MTGJSONSet {
        data: MTGJSONSetData {
            cards: vec![create_test_card()],
        },
        meta: MTGJSONSetMeta {
            version: "5.2.1".to_string(),
            date: "2024-03-21".to_string(),
        },
    }
}

pub fn create_mock_meta() -> MTGJSONMeta {
    let mut checksums = HashMap::new();
    let test_set = create_mock_set();
    let test_json = serde_json::to_string(&test_set.data).unwrap();

    // Create a hash for each set using the same test data
    for set_code in ["TEST", "SET1", "SET2"] {
        let mut hasher = Sha256::new();
        hasher.update(test_json.as_bytes());
        checksums.insert(
            format!("{}.json", set_code),
            format!("{:x}", hasher.finalize()),
        );
    }

    MTGJSONMeta {
        version: "5.2.1".to_string(),
        date: "2024-03-21".to_string(),
        checksums,
    }
}

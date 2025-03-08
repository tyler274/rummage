use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};
use async_trait::async_trait;
use bzip2::read::BzDecoder;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tar;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MTGJSONCard {
    pub name: String,
    #[serde(rename = "manaCost")]
    pub mana_cost: Option<String>,
    pub text: Option<String>,
    pub types: Vec<String>,
    pub supertypes: Option<Vec<String>>,
    pub subtypes: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    pub power: Option<String>,
    pub toughness: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MTGJSONSet {
    pub data: MTGJSONSetData,
    pub meta: MTGJSONSetMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MTGJSONSetMeta {
    pub version: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MTGJSONSetData {
    pub cards: Vec<MTGJSONCard>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MTGJSONMeta {
    pub version: String,
    pub date: String,
    pub checksums: HashMap<String, String>,
}

// Define client types
#[derive(Clone)]
pub enum MTGClientType {
    Reqwest(reqwest::Client),
    #[cfg(test)]
    Mock(Arc<tests::MockClient>),
}

#[async_trait]
pub trait MTGClient: Send + Sync {
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>>;
}

#[async_trait]
impl MTGClient for MTGClientType {
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>> {
        match self {
            MTGClientType::Reqwest(client) => {
                // First, download the set directly
                let url = format!("https://mtgjson.com/api/v5/{}.json", set_code);
                let response = client.get(&url).send().await?;
                let set: MTGJSONSet = response.json().await?;
                Ok(set)
            }
            #[cfg(test)]
            MTGClientType::Mock(client) => client.fetch_set(set_code).await,
        }
    }
}

#[derive(Clone)]
pub struct MTGService {
    client: MTGClientType,
    cache: Arc<Mutex<HashMap<String, Vec<Card>>>>,
    cache_dir: PathBuf,
    raw_cache_dir: PathBuf,
    sets_dir: PathBuf,
    last_request: Arc<Mutex<Instant>>,
    rate_limit: Duration,
    meta: Arc<Mutex<Option<MTGJSONMeta>>>,
}

impl MTGService {
    pub fn new(client: MTGClientType) -> Self {
        let cache_dir = Self::get_cache_dir();
        let raw_cache_dir = cache_dir.join("raw");
        let sets_dir = PathBuf::from("sets");
        let memory_cache = Arc::new(Mutex::new(HashMap::new()));

        // Create raw cache and sets directories
        fs::create_dir_all(&raw_cache_dir)
            .unwrap_or_else(|e| eprintln!("Failed to create raw cache dir: {}", e));
        fs::create_dir_all(&sets_dir)
            .unwrap_or_else(|e| eprintln!("Failed to create sets dir: {}", e));

        // Load cache from disk if it exists and is valid
        if let Ok(cache) = Self::load_cache_from_disk(&cache_dir) {
            *memory_cache.lock().unwrap() = cache;
        }

        Self {
            client,
            cache: memory_cache,
            cache_dir,
            raw_cache_dir,
            sets_dir,
            last_request: Arc::new(Mutex::new(Instant::now())),
            rate_limit: Duration::from_millis(100), // 100ms between requests
            meta: Arc::new(Mutex::new(None)),
        }
    }

    fn get_cache_dir() -> PathBuf {
        let mut cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".cache"));
        cache_dir.push("rummage");
        cache_dir.push("mtgjson");
        fs::create_dir_all(&cache_dir)
            .unwrap_or_else(|e| eprintln!("Failed to create cache dir: {}", e));
        cache_dir
    }

    fn get_executable_hash() -> String {
        let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from(""));
        if let Ok(contents) = fs::read(&exe_path) {
            let mut hasher = Sha256::new();
            hasher.update(&contents);
            format!("{:x}", hasher.finalize())
        } else {
            String::from("default")
        }
    }

    fn load_cache_from_disk(
        cache_dir: &Path,
    ) -> Result<HashMap<String, Vec<Card>>, Box<dyn std::error::Error>> {
        let hash_file = cache_dir.join("executable.hash");
        let cache_file = cache_dir.join("cache.json");

        // Check if cache exists and hash matches
        if !hash_file.exists() || !cache_file.exists() {
            return Err("Cache files don't exist".into());
        }

        let stored_hash = fs::read_to_string(&hash_file)?;
        let current_hash = Self::get_executable_hash();

        if stored_hash != current_hash {
            // Cache is invalid, clean it up
            let _ = fs::remove_file(&hash_file);
            let _ = fs::remove_file(&cache_file);
            return Err("Cache is outdated".into());
        }

        // Load cache
        let cache_contents = fs::read_to_string(&cache_file)?;
        let cache: HashMap<String, Vec<Card>> = serde_json::from_str(&cache_contents)?;
        Ok(cache)
    }

    fn save_cache_to_disk(&self) {
        let hash_file = self.cache_dir.join("executable.hash");
        let cache_file = self.cache_dir.join("cache.json");

        // Save executable hash
        if let Err(e) = fs::write(&hash_file, Self::get_executable_hash()) {
            eprintln!("Failed to write hash file: {}", e);
            return;
        }

        // Save cache
        if let Ok(cache_contents) = serde_json::to_string(&*self.cache.lock().unwrap()) {
            if let Err(e) = fs::write(&cache_file, cache_contents) {
                eprintln!("Failed to write cache file: {}", e);
            }
        }
    }

    pub fn new_with_reqwest() -> Self {
        Self::new(MTGClientType::Reqwest(reqwest::Client::new()))
    }

    async fn enforce_rate_limit(&self) {
        let mut last_request = self.last_request.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(*last_request);

        if elapsed < self.rate_limit {
            let sleep_duration = self.rate_limit - elapsed;
            drop(last_request); // Release the lock before sleeping
            sleep(sleep_duration).await;
            *self.last_request.lock().unwrap() = Instant::now();
        } else {
            *last_request = now;
        }
    }

    fn get_raw_cache_path(&self, set_code: &str) -> PathBuf {
        self.raw_cache_dir.join(format!("{}.json", set_code))
    }

    fn get_set_archive_path(&self, set_code: &str) -> PathBuf {
        self.sets_dir.join(format!("{}.json.bz2", set_code))
    }

    async fn fetch_meta(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we already have meta data
        if self.meta.lock().unwrap().is_some() {
            return Ok(());
        }

        // Enforce rate limit before making request
        self.enforce_rate_limit().await;

        // Fetch meta.json from MTGJSON
        let url = "https://mtgjson.com/api/v5/Meta.json";
        let response = match &self.client {
            MTGClientType::Reqwest(client) => {
                let response = client.get(url).send().await?;
                let meta: MTGJSONMeta = response.json().await?;
                meta
            }
            #[cfg(test)]
            MTGClientType::Mock(_) => {
                return Ok(());
            }
        };

        *self.meta.lock().unwrap() = Some(response);
        Ok(())
    }

    fn verify_file_checksum(&self, file_path: &Path, set_code: &str) -> bool {
        if let Some(meta) = &*self.meta.lock().unwrap() {
            if let Some(expected_checksum) = meta.checksums.get(&format!("{}.json", set_code)) {
                if let Ok(contents) = fs::read(file_path) {
                    let mut hasher = Sha256::new();
                    hasher.update(&contents);
                    let actual_checksum = format!("{:x}", hasher.finalize());
                    return actual_checksum == *expected_checksum;
                }
            }
        }
        false
    }

    fn cache_set_data(
        &self,
        set_code: &str,
        set_data: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Save the raw JSON
        let raw_cache_path = self.get_raw_cache_path(set_code);
        fs::write(&raw_cache_path, set_data)?;

        // Verify checksum
        if !self.verify_file_checksum(&raw_cache_path, set_code) {
            fs::remove_file(&raw_cache_path)?;
            return Err("Checksum verification failed".into());
        }

        // Save the compressed bz2 file
        let set_archive_path = self.get_set_archive_path(set_code);
        let mut compressed = Vec::new();
        let mut compressor =
            bzip2::write::BzEncoder::new(&mut compressed, bzip2::Compression::best());
        compressor.write_all(set_data.as_bytes())?;
        compressor.finish()?;
        fs::write(&set_archive_path, compressed)?;

        Ok(())
    }

    pub async fn fetch_set(&self, set_code: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        // Fetch meta data if we don't have it
        self.fetch_meta().await?;

        // Check memory cache first
        if let Some(cards) = self.cache.lock().unwrap().get(set_code) {
            return Ok(cards.clone());
        }

        // Check raw cache
        let raw_cache_path = self.get_raw_cache_path(set_code);
        let set_archive_path = self.get_set_archive_path(set_code);

        let set_data =
            if raw_cache_path.exists() && self.verify_file_checksum(&raw_cache_path, set_code) {
                fs::read_to_string(&raw_cache_path)?
            } else {
                // Check if we have the set archive and try to decompress it
                let set_data = if set_archive_path.exists() {
                    let file = fs::File::open(&set_archive_path)?;
                    let mut bz = BzDecoder::new(file);
                    let mut set_data = String::new();
                    bz.read_to_string(&mut set_data)?;

                    // Verify the decompressed data
                    fs::write(&raw_cache_path, &set_data)?;
                    if !self.verify_file_checksum(&raw_cache_path, set_code) {
                        fs::remove_file(&raw_cache_path)?;
                        fs::remove_file(&set_archive_path)?;
                        return Err("Checksum verification failed for cached archive".into());
                    }
                    set_data
                } else {
                    // Enforce rate limit before making request
                    self.enforce_rate_limit().await;

                    // Download the AllSetFiles.tar.bz2
                    let url = "https://mtgjson.com/api/v5/AllSetFiles.tar.bz2";
                    let response = match &self.client {
                        MTGClientType::Reqwest(client) => {
                            let response = client.get(url).send().await?;
                            let bytes = response.bytes().await?;

                            // Create a bzip2 decoder
                            let bz = BzDecoder::new(&bytes[..]);
                            let mut archive = tar::Archive::new(bz);

                            // Find and extract the specific set file
                            let set_path = format!("{}.json", set_code);
                            let mut set_data = String::new();

                            for entry in archive.entries()? {
                                let mut entry = entry?;
                                let path = entry.path()?.to_path_buf();
                                if path.to_string_lossy().ends_with(&set_path) {
                                    entry.read_to_string(&mut set_data)?;
                                    break;
                                }
                            }

                            if set_data.is_empty() {
                                return Err(format!("Set {} not found in archive", set_code).into());
                            }

                            // Cache the set data and verify checksum
                            self.cache_set_data(set_code, &set_data)?;

                            set_data
                        }
                        #[cfg(test)]
                        MTGClientType::Mock(client) => {
                            let set = client.fetch_set(set_code).await?;
                            let set_data = serde_json::to_string(&set)?;
                            self.cache_set_data(set_code, &set_data)?;
                            set_data
                        }
                    };

                    response
                };

                set_data
            };

        // Parse JSON
        let set: MTGJSONSet = serde_json::from_str(&set_data)?;
        let cards = set
            .data
            .cards
            .into_iter()
            .filter_map(|mtg_card| convert_mtgjson_to_card(mtg_card))
            .collect::<Vec<_>>();

        // Store in memory cache
        self.cache
            .lock()
            .unwrap()
            .insert(set_code.to_string(), cards.clone());

        // Save to disk
        self.save_cache_to_disk();

        Ok(cards)
    }

    pub async fn fetch_multiple_sets(
        &self,
        set_codes: &[&str],
    ) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let mut all_cards = Vec::new();
        for set_code in set_codes {
            match self.fetch_set(set_code).await {
                Ok(mut cards) => all_cards.append(&mut cards),
                Err(e) => eprintln!("Error fetching set {}: {}", set_code, e),
            }
        }
        Ok(all_cards)
    }
}

fn determine_card_type(
    types: &[String],
    supertypes: Option<&Vec<String>>,
    subtypes: Option<&Vec<String>>,
) -> Option<CardTypes> {
    let mut card_types = CardTypes::NONE;

    // Process supertypes
    if let Some(supertypes) = supertypes {
        for supertype in supertypes {
            match supertype.as_str() {
                "Basic" => card_types |= CardTypes::BASIC,
                "Legendary" => card_types |= CardTypes::LEGENDARY,
                "Snow" => card_types |= CardTypes::SNOW,
                "World" => card_types |= CardTypes::WORLD,
                _ => {}
            }
        }
    }

    // Process main types
    for card_type in types {
        match card_type.as_str() {
            "Artifact" => card_types |= CardTypes::ARTIFACT,
            "Conspiracy" => card_types |= CardTypes::CONSPIRACY,
            "Creature" => card_types |= CardTypes::CREATURE,
            "Enchantment" => card_types |= CardTypes::ENCHANTMENT,
            "Instant" => card_types |= CardTypes::INSTANT,
            "Land" => card_types |= CardTypes::LAND,
            "Phenomenon" => card_types |= CardTypes::PHENOMENON,
            "Plane" => card_types |= CardTypes::PLANE,
            "Planeswalker" => card_types |= CardTypes::PLANESWALKER,
            "Scheme" => card_types |= CardTypes::SCHEME,
            "Sorcery" => card_types |= CardTypes::SORCERY,
            "Tribal" => card_types |= CardTypes::TRIBAL,
            "Vanguard" => card_types |= CardTypes::VANGUARD,
            _ => {}
        }
    }

    // Process subtypes
    if let Some(subtypes) = subtypes {
        for subtype in subtypes {
            match subtype.as_str() {
                "Saga" => card_types |= CardTypes::SAGA,
                "Equipment" => card_types |= CardTypes::EQUIPMENT,
                "Aura" => card_types |= CardTypes::AURA,
                "Vehicle" => card_types |= CardTypes::VEHICLE,
                "Food" => card_types |= CardTypes::FOOD,
                "Clue" => card_types |= CardTypes::CLUE,
                "Treasure" => card_types |= CardTypes::TREASURE,
                "Fortification" => card_types |= CardTypes::FORTIFICATION,
                "Contraption" => card_types |= CardTypes::CONTRAPTION,
                "Plains" => card_types |= CardTypes::PLAINS,
                "Island" => card_types |= CardTypes::ISLAND,
                "Swamp" => card_types |= CardTypes::SWAMP,
                "Mountain" => card_types |= CardTypes::MOUNTAIN,
                "Forest" => card_types |= CardTypes::FOREST,
                _ => {}
            }
        }
    }

    if card_types == CardTypes::NONE {
        None
    } else {
        Some(card_types)
    }
}

fn infer_creature_types_from_name_and_text(name: &str, rules_text: &str) -> CreatureType {
    let mut creature_types = CreatureType::NONE;
    let text = format!("{} {}", name, rules_text).to_lowercase();

    // Phyrexian inference
    if text.contains("phyrexian") || text.contains("compleated") || text.contains("praetors") {
        creature_types |= CreatureType::PHYREXIAN;
    }

    // Common race/class patterns
    let patterns = [
        ("artificer", CreatureType::ARTIFICER),
        ("assassin", CreatureType::ASSASSIN),
        ("berserker", CreatureType::BERSERKER),
        ("knight", CreatureType::KNIGHT),
        ("warrior", CreatureType::WARRIOR),
        ("wizard", CreatureType::WIZARD),
        ("cleric", CreatureType::CLERIC),
        ("druid", CreatureType::DRUID),
        ("shaman", CreatureType::SHAMAN),
        ("rogue", CreatureType::ROGUE),
        ("soldier", CreatureType::SOLDIER),
        ("monk", CreatureType::MONK),
    ];

    for (pattern, creature_type) in patterns.iter() {
        if text.contains(pattern) {
            creature_types |= *creature_type;
        }
    }

    creature_types
}

fn apply_retroactive_creature_types(name: &str, existing_types: CreatureType) -> CreatureType {
    let mut types = existing_types;

    // Known retroactive additions
    let retroactive_types = [
        ("Urza", CreatureType::HUMAN | CreatureType::ARTIFICER),
        ("Mishra", CreatureType::HUMAN | CreatureType::ARTIFICER),
        ("Yawgmoth", CreatureType::PHYREXIAN),
        ("Gix", CreatureType::PHYREXIAN),
        ("Xantcha", CreatureType::PHYREXIAN),
        ("Ashnod", CreatureType::HUMAN | CreatureType::ARTIFICER),
        ("Tawnos", CreatureType::HUMAN | CreatureType::ARTIFICER),
        ("Slobad", CreatureType::GOBLIN | CreatureType::ARTIFICER),
        ("Glissa", CreatureType::PHYREXIAN | CreatureType::ELF),
    ];

    for (character_name, character_types) in retroactive_types.iter() {
        if name.contains(character_name) {
            types |= *character_types;
        }
    }

    types
}

fn determine_creature_types(subtypes: &[String], name: &str, rules_text: &str) -> CreatureType {
    let mut creature_types = CreatureType::NONE;

    // First, get types from explicit subtypes
    for subtype in subtypes {
        match subtype.as_str() {
            "Human" => creature_types |= CreatureType::HUMAN,
            "Wizard" => creature_types |= CreatureType::WIZARD,
            "Dragon" => creature_types |= CreatureType::DRAGON,
            "Angel" => creature_types |= CreatureType::ANGEL,
            "Demon" => creature_types |= CreatureType::DEMON,
            "Warrior" => creature_types |= CreatureType::WARRIOR,
            "Soldier" => creature_types |= CreatureType::SOLDIER,
            "Cleric" => creature_types |= CreatureType::CLERIC,
            "Rogue" => creature_types |= CreatureType::ROGUE,
            "Shaman" => creature_types |= CreatureType::SHAMAN,
            "Beast" => creature_types |= CreatureType::BEAST,
            "Elemental" => creature_types |= CreatureType::ELEMENTAL,
            "Vampire" => creature_types |= CreatureType::VAMPIRE,
            "Zombie" => creature_types |= CreatureType::ZOMBIE,
            "Goblin" => creature_types |= CreatureType::GOBLIN,
            "Elf" => creature_types |= CreatureType::ELF,
            "Merfolk" => creature_types |= CreatureType::MERFOLK,
            "Bird" => creature_types |= CreatureType::BIRD,
            "Spirit" => creature_types |= CreatureType::SPIRIT,
            "Knight" => creature_types |= CreatureType::KNIGHT,
            "Druid" => creature_types |= CreatureType::DRUID,
            "Assassin" => creature_types |= CreatureType::ASSASSIN,
            "Artificer" => creature_types |= CreatureType::ARTIFICER,
            "Monk" => creature_types |= CreatureType::MONK,
            "Horror" => creature_types |= CreatureType::HORROR,
            "Giant" => creature_types |= CreatureType::GIANT,
            "Dinosaur" => creature_types |= CreatureType::DINOSAUR,
            "Hydra" => creature_types |= CreatureType::HYDRA,
            "Phoenix" => creature_types |= CreatureType::PHOENIX,
            "Wurm" => creature_types |= CreatureType::WURM,
            "Phyrexian" => creature_types |= CreatureType::PHYREXIAN,
            "Berserker" => creature_types |= CreatureType::BERSERKER,
            _ => {}
        }
    }

    // Apply retroactive types
    creature_types = apply_retroactive_creature_types(name, creature_types);

    // If no types were found, try to infer from name and text
    if creature_types == CreatureType::NONE {
        creature_types = infer_creature_types_from_name_and_text(name, rules_text);
    }

    creature_types
}

fn parse_mana_cost(cost_str: &str) -> Mana {
    let mut mana = Mana::default();
    let mut color = Color::COLORLESS;

    // MTGJSON format uses {X} for costs, e.g. "{2}{U}{U}"
    for symbol in cost_str.split('}') {
        if let Some(symbol) = symbol.strip_prefix('{') {
            match symbol {
                "W" => {
                    mana.white += 1;
                    color |= Color::WHITE;
                }
                "U" => {
                    mana.blue += 1;
                    color |= Color::BLUE;
                }
                "B" => {
                    mana.black += 1;
                    color |= Color::BLACK;
                }
                "R" => {
                    mana.red += 1;
                    color |= Color::RED;
                }
                "G" => {
                    mana.green += 1;
                    color |= Color::GREEN;
                }
                // Handle colorless mana costs
                s if s.chars().all(|c| c.is_ascii_digit()) => {
                    if let Ok(amount) = s.parse::<u64>() {
                        mana.colorless += amount;
                    }
                }
                // Ignore other symbols like {X}, {P}, etc. for now
                _ => {}
            }
        }
    }

    mana.color = color;
    mana
}

fn convert_mtgjson_to_card(mtg_card: MTGJSONCard) -> Option<Card> {
    let card_types = determine_card_type(
        &mtg_card.types,
        mtg_card.supertypes.as_ref(),
        mtg_card.subtypes.as_ref(),
    )?;
    let _colors = mtg_card.colors.unwrap_or_default();

    let card_details = if card_types.contains(CardTypes::CREATURE) {
        let creature_types = mtg_card
            .subtypes
            .as_ref()
            .map(|subtypes| {
                determine_creature_types(
                    subtypes,
                    &mtg_card.name,
                    &mtg_card.text.as_deref().unwrap_or(""),
                )
            })
            .unwrap_or_else(|| {
                infer_creature_types_from_name_and_text(
                    &mtg_card.name,
                    &mtg_card.text.as_deref().unwrap_or(""),
                )
            });

        CardDetails::Creature(CreatureCard {
            power: mtg_card.power.and_then(|p| p.parse().ok()).unwrap_or(0),
            toughness: mtg_card.toughness.and_then(|t| t.parse().ok()).unwrap_or(0),
            creature_type: creature_types,
        })
    } else if card_types.contains(CardTypes::PLANESWALKER) {
        CardDetails::Planeswalker { loyalty: 0 }
    } else {
        CardDetails::Other
    };

    Some(Card {
        name: mtg_card.name,
        cost: parse_mana_cost(&mtg_card.mana_cost.unwrap_or_default()),
        types: card_types,
        card_details,
        rules_text: mtg_card.text.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock client implementation
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
        async fn fetch_set(
            &self,
            set_code: &str,
        ) -> Result<MTGJSONSet, Box<dyn std::error::Error>> {
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

    // Test helpers and fixtures
    mod test_utils {
        use super::*;

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
    }

    // Unit tests
    mod unit {
        use super::*;
        use test_utils::*;

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
    }

    // Integration tests
    mod integration {
        use super::*;
        use test_utils::*;

        mod service {
            use super::*;

            #[tokio::test]
            async fn test_fetch_set_with_mock() {
                let mock_client = MockClient::new();
                let test_set = create_mock_set();
                mock_client.mock_response("TEST", test_set);
                mock_client.mock_meta(create_mock_meta());

                let service = MTGService::new(MTGClientType::Mock(Arc::new(mock_client)));
                let cards = service.fetch_set("TEST").await.unwrap();

                assert_eq!(cards.len(), 1);
                assert_eq!(cards[0].name, "Test Creature");
            }

            #[tokio::test]
            async fn test_fetch_multiple_sets_with_mock() {
                let mock_client = MockClient::new();
                let test_set = create_mock_set();
                mock_client.mock_response("SET1", test_set.clone());
                mock_client.mock_response("SET2", test_set);
                mock_client.mock_meta(create_mock_meta());

                let service = MTGService::new(MTGClientType::Mock(Arc::new(mock_client)));
                let cards = service
                    .fetch_multiple_sets(&["SET1", "SET2"])
                    .await
                    .unwrap();

                assert_eq!(cards.len(), 2);
            }

            #[tokio::test]
            async fn test_checksum_validation() {
                let mock_client = MockClient::new();
                let mut meta = create_mock_meta();

                // Set an invalid checksum
                meta.checksums
                    .insert("TEST.json".to_string(), "invalid_checksum".to_string());

                mock_client.mock_meta(meta);
                mock_client.mock_response("TEST", create_mock_set());

                let service = MTGService::new(MTGClientType::Mock(Arc::new(mock_client)));
                let result = service.fetch_set("TEST").await;

                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("Checksum verification failed")
                );
            }
        }

        // Network-dependent tests that should be skipped in CI
        mod network {
            use super::*;

            async fn validate_card(card: &Card) {
                assert!(!card.name.is_empty(), "Card name should not be empty");
                assert!(
                    card.types != CardTypes::NONE,
                    "Card should have at least one type"
                );

                if card.types.contains(CardTypes::CREATURE) {
                    if let CardDetails::Creature(creature) = &card.card_details {
                        assert!(creature.power >= 0, "Creature power should be >= 0");
                        assert!(creature.toughness >= 0, "Creature toughness should be >= 0");
                    } else {
                        panic!("Card with CREATURE type should have Creature details");
                    }
                }

                if card.types.contains(CardTypes::PLANESWALKER) {
                    assert!(
                        matches!(card.card_details, CardDetails::Planeswalker { .. }),
                        "Card with PLANESWALKER type should have Planeswalker details"
                    );
                }
            }

            #[tokio::test]
            async fn test_fetch_modern_set() {
                if std::env::var("CI").is_ok() || std::env::var("SKIP_NETWORK_TESTS").is_ok() {
                    return;
                }

                let service = MTGService::new_with_reqwest();
                let result = service.fetch_set("MH2").await;
                assert!(result.is_ok(), "Failed to fetch MH2: {:?}", result.err());

                let cards = result.unwrap();
                assert!(!cards.is_empty(), "Set should contain cards");
                assert!(cards.len() > 100, "Modern set should have many cards");

                for card in &cards {
                    validate_card(card).await;
                }
            }

            #[tokio::test]
            async fn test_fetch_basic_land_set() {
                if std::env::var("CI").is_ok() || std::env::var("SKIP_NETWORK_TESTS").is_ok() {
                    return;
                }

                let service = MTGService::new_with_reqwest();
                let result = service.fetch_set("30A").await;
                assert!(result.is_ok(), "Failed to fetch 30A: {:?}", result.err());

                let cards = result.unwrap();
                assert!(!cards.is_empty(), "Set should contain cards");

                let mut basic_land_types = 0;
                for card in &cards {
                    validate_card(card).await;
                    if card.types.contains(CardTypes::BASIC) && card.types.contains(CardTypes::LAND)
                    {
                        basic_land_types += 1;
                        assert_eq!(card.cost.total(), 0, "Basic lands should have no mana cost");
                    }
                }

                assert!(basic_land_types >= 5, "Should have all basic land types");
            }
        }
    }
}

//! MTGJSON Integration Module
//!
//! This module provides integration with the MTGJSON API (https://mtgjson.com/), allowing the application
//! to fetch and process Magic: The Gathering card data. It includes functionality for:
//!
//! - Fetching card sets and individual cards
//! - Rate-limited API access
//! - Local caching of downloaded data
//! - Conversion between MTGJSON and internal card representations
//! - Mock client support for testing
//!
//! The module implements proper rate limiting to respect MTGJSON's API guidelines and includes
//! robust error handling and data validation.

use crate::cards::{
    Card, CardCost, CardDetails, CardDetailsComponent, CardKeywords, CardName, CardRulesText,
    CardTypeInfo, CardTypes, CreatureCard, CreatureType,
};
use crate::mana::{Mana, ManaColor};
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{Duration, sleep};

pub mod test_utils;

use test_utils::MockClient;

lazy_static! {
    /// Global rate limiter for MTGJSON API requests
    /// Ensures we don't exceed the API's rate limits
    static ref RATE_LIMITER: Arc<TokioMutex<Instant>> = Arc::new(TokioMutex::new(Instant::now()));
}

/// Duration between API requests (100ms = 10 requests per second max)
const RATE_LIMIT_DURATION: Duration = Duration::from_millis(100);

#[allow(dead_code)]
type Error = Box<dyn std::error::Error>;

/// Response structure for MTGJSON Meta API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMetaResponse {
    /// Meta information about the response
    pub meta: MTGJSONMetaData,
    /// Actual meta data content
    pub data: MTGJSONMetaData,
}

/// Metadata structure containing version and date information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMetaData {
    /// Date of the data update
    pub date: String,
    /// Version of the MTGJSON data
    pub version: String,
}

/// Combined metadata structure including checksums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMeta {
    /// Date of the data update
    pub date: String,
    /// Version of the MTGJSON data
    pub version: String,
    /// Map of file checksums for validation
    pub checksums: HashMap<String, String>,
}

impl From<MTGJSONMetaResponse> for MTGJSONMeta {
    fn from(response: MTGJSONMetaResponse) -> Self {
        Self {
            date: response.data.date,
            version: response.data.version,
            checksums: HashMap::new(), // We'll need to fetch checksums separately
        }
    }
}

/// Metadata specific to a card set
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MTGJSONSetMeta {
    /// Version of the set data
    #[serde(default)]
    pub version: String,
    /// Date of the set data update
    #[serde(default)]
    pub date: String,
}

/// Response structure for set data from MTGJSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetResponse {
    /// The actual set data
    pub data: MTGJSONSet,
    /// Metadata about the set
    pub meta: MTGJSONSetMeta,
}

/// Comprehensive structure representing a Magic: The Gathering card set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSet {
    /// List of artist IDs associated with the set
    #[serde(default)]
    pub artist_ids: Option<Vec<String>>,
    /// Platforms where the set is available (e.g., "paper", "mtgo")
    #[serde(default)]
    pub availability: Vec<String>,
    /// List of cards in the set
    pub cards: Vec<MTGJSONCard>,
    /// Set code (e.g., "M21" for Core Set 2021)
    pub code: String,
    /// Full name of the set
    pub name: String,
    /// Total number of cards in the set including variants
    #[serde(default)]
    pub total_set_size: i32,
    /// Official release date of the set
    #[serde(default)]
    pub release_date: String,
    /// Type of the set (e.g., "core", "expansion")
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for the set
    #[serde(default)]
    pub uuid: Option<String>,
    /// Available language versions
    #[serde(default)]
    pub languages: Vec<String>,
    /// Booster pack configuration data
    #[serde(default)]
    pub booster: Option<serde_json::Value>,
    /// Information about sealed products
    #[serde(default)]
    pub sealed_product: Option<Vec<serde_json::Value>>,
    /// Token cards in the set
    #[serde(default)]
    pub tokens: Option<Vec<MTGJSONCard>>,
    /// Set name translations
    #[serde(default)]
    pub translations: Option<HashMap<String, Option<String>>>,
    /// Number of cards in the base set (excluding variants)
    #[serde(default)]
    pub base_set_size: Option<i32>,
    /// Block the set belongs to
    #[serde(default)]
    pub block: Option<String>,
    /// Whether the set is only available in non-English
    #[serde(default)]
    pub is_foreign_only: Option<bool>,
    /// Whether this is a preview of an unreleased set
    #[serde(default)]
    pub is_partial_preview: Option<bool>,
    /// Whether the set is only available online
    #[serde(default)]
    pub is_online_only: Option<bool>,
    /// Keyrune code for set symbol font
    #[serde(default)]
    pub keyrunecode: Option<String>,
    /// CardMarket set ID
    #[serde(default)]
    pub mcm_id: Option<i32>,
    /// CardMarket set name
    #[serde(default)]
    pub mcm_name: Option<String>,
    /// Magic Online set code
    #[serde(default)]
    pub mtgo_code: Option<String>,
    /// TCGPlayer group ID
    #[serde(default)]
    pub tcgplayer_group_id: Option<i32>,
    /// Set metadata
    #[serde(default)]
    pub meta: Option<MTGJSONSetMeta>,
}

/// Comprehensive structure representing a single Magic: The Gathering card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONCard {
    /// Artist name
    pub artist: Option<String>,
    /// Artist unique identifiers
    #[serde(rename = "artistIds")]
    pub artist_ids: Option<Vec<String>>,
    /// Platforms where the card is available
    pub availability: Vec<String>,
    /// Card border color
    #[serde(rename = "borderColor")]
    pub border_color: String,
    /// Card's color identity in Commander
    #[serde(rename = "colorIdentity")]
    pub color_identity: Vec<String>,
    /// Card's colors
    pub colors: Option<Vec<String>>,
    /// Converted mana cost (total mana value)
    #[serde(rename = "convertedManaCost")]
    pub converted_mana_cost: Option<f32>,
    /// EDHREC card ranking
    #[serde(rename = "edhrecRank")]
    pub edhrec_rank: Option<i32>,
    /// Available card finishes (e.g., foil)
    pub finishes: Vec<String>,
    /// Card data in other languages
    #[serde(rename = "foreignData")]
    pub foreign_data: Option<Vec<serde_json::Value>>,
    /// Card frame style version
    #[serde(rename = "frameVersion")]
    pub frame_version: String,
    /// Whether foil version exists
    #[serde(rename = "hasFoil")]
    pub has_foil: bool,
    /// Whether non-foil version exists
    #[serde(rename = "hasNonFoil")]
    pub has_non_foil: bool,
    /// Various platform-specific identifiers
    pub identifiers: MTGJSONCardIdentifiers,
    /// Whether this is a reprint
    #[serde(rename = "isReprint")]
    pub is_reprint: Option<bool>,
    /// Whether this is a starter card
    #[serde(rename = "isStarter")]
    pub is_starter: Option<bool>,
    /// Card's keyword abilities
    pub keywords: Option<Vec<String>>,
    /// Card's language
    #[serde(default = "default_language")]
    pub language: String,
    /// Card's layout (e.g., normal, split)
    pub layout: String,
    /// Format legality information
    #[serde(default)]
    pub legalities: HashMap<String, String>,
    /// Mana cost string
    #[serde(rename = "manaCost")]
    pub mana_cost: Option<String>,
    /// Total mana value
    #[serde(rename = "manaValue")]
    pub mana_value: Option<f32>,
    /// Card name
    pub name: String,
    /// Collector number
    pub number: String,
    /// Power (for creatures)
    pub power: Option<String>,
    /// Set codes where this card was printed
    #[serde(default)]
    pub printings: Vec<String>,
    /// URLs to purchase the card
    #[serde(rename = "purchaseUrls")]
    pub purchase_urls: Option<HashMap<String, String>>,
    /// Card rarity
    #[serde(default = "default_rarity")]
    pub rarity: String,
    /// Official rulings
    pub rulings: Option<Vec<MTGJSONRuling>>,
    /// Security stamp type
    #[serde(rename = "securityStamp")]
    pub security_stamp: Option<String>,
    /// Set code this card belongs to
    #[serde(rename = "setCode")]
    pub set_code: String,
    /// Products containing this card
    #[serde(rename = "sourceProducts")]
    pub source_products: Option<HashMap<String, Vec<String>>>,
    /// Card subtypes
    pub subtypes: Vec<String>,
    /// Card supertypes
    pub supertypes: Vec<String>,
    /// Card text/rules
    pub text: Option<String>,
    /// Toughness (for creatures)
    pub toughness: Option<String>,
    /// Full type line
    #[serde(rename = "type")]
    pub type_: String,
    /// Card types
    pub types: Vec<String>,
    /// Unique identifier
    pub uuid: String,
    /// Alternative versions
    pub variations: Option<Vec<String>>,
}

/// Collection of various card identifiers across different platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONCardIdentifiers {
    /// CardKingdom product ID
    #[serde(rename = "cardKingdomId")]
    pub card_kingdom_id: Option<String>,
    /// CardKingdom foil product ID
    #[serde(rename = "cardKingdomFoilId")]
    pub card_kingdom_foil_id: Option<String>,
    /// MTGJSON v4 ID
    #[serde(rename = "mtgjsonV4Id")]
    pub mtgjson_v4_id: Option<String>,
    /// Scryfall card back ID
    #[serde(rename = "scryfallCardBackId")]
    pub scryfall_card_back_id: Option<String>,
    /// Scryfall card ID
    #[serde(rename = "scryfallId")]
    pub scryfall_id: Option<String>,
    /// Scryfall illustration ID
    #[serde(rename = "scryfallIllustrationId")]
    pub scryfall_illustration_id: Option<String>,
    /// Scryfall Oracle ID
    #[serde(rename = "scryfallOracleId")]
    pub scryfall_oracle_id: Option<String>,
    /// TCGPlayer product ID
    #[serde(rename = "tcgplayerProductId")]
    pub tcgplayer_product_id: Option<String>,
}

/// Structure representing an official card ruling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONRuling {
    /// Date the ruling was issued
    pub date: String,
    /// Ruling text
    pub text: String,
}

/// Trait defining the interface for MTG data clients
#[async_trait]
pub trait MTGClient: Send + Sync {
    /// Fetches a complete set by its code
    #[allow(dead_code)]
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>>;
}

/// Enum representing different types of MTG clients
#[derive(Debug)]
#[allow(dead_code)]
pub enum MTGClientType {
    /// Live HTTP client for actual API requests
    HTTP(reqwest::Client),
    /// Mock client for testing
    Mock(Arc<MockClient>),
}

impl MTGClientType {
    /// Fetches a set from MTGJSON by its set code
    ///
    /// This method handles both live HTTP requests and mock responses for testing.
    /// For HTTP requests, it implements rate limiting to respect API guidelines.
    ///
    /// # Arguments
    ///
    /// * `set_code` - The code of the set to fetch (e.g., "M21" for Core Set 2021)
    ///
    /// # Returns
    ///
    /// Returns a Result containing either the fetched set data or an error
    ///
    /// # Rate Limiting
    ///
    /// HTTP requests are limited to 10 requests per second using a global rate limiter
    pub async fn fetch_set(
        &self,
        set_code: &str,
    ) -> Result<MTGJSONSet, Box<dyn std::error::Error>> {
        match self {
            MTGClientType::HTTP(client) => {
                // Apply rate limiting for HTTP requests
                let mut last_request = RATE_LIMITER.lock().await;
                let now = Instant::now();
                let elapsed = now.duration_since(*last_request);
                if elapsed < RATE_LIMIT_DURATION {
                    sleep(RATE_LIMIT_DURATION - elapsed).await;
                }
                *last_request = Instant::now();

                let url = format!("https://mtgjson.com/api/v5/{}.json.bz2", set_code);
                let response = client.get(&url).send().await?;
                if !response.status().is_success() {
                    return Err(
                        format!("Failed to fetch set {}: {}", set_code, response.status()).into(),
                    );
                }
                let bytes = response.bytes().await?;
                let decompressed = bzip2::read::BzDecoder::new(&bytes[..]);
                let set_response: MTGJSONSetResponse = serde_json::from_reader(decompressed)?;
                Ok(set_response.data)
            }
            MTGClientType::Mock(client) => client.fetch_set(set_code).await,
        }
    }
}

/// Service for interacting with MTGJSON data
///
/// This service handles fetching and caching of MTG card data,
/// including versioning and data validation.
#[allow(dead_code)]
pub struct MTGService {
    /// The client used to fetch data (either HTTP or Mock)
    client: MTGClientType,
    /// In-memory cache of card sets
    cache: Arc<TokioMutex<HashMap<String, Vec<Card>>>>,
    /// Cached metadata about the MTGJSON version
    meta: Arc<TokioMutex<Option<MTGJSONMeta>>>,
}

impl MTGService {
    /// Creates a new MTGService instance with the specified client
    #[allow(dead_code)]
    pub fn new(client: MTGClientType) -> Self {
        Self {
            client,
            cache: Arc::new(TokioMutex::new(HashMap::new())),
            meta: Arc::new(TokioMutex::new(None)),
        }
    }

    /// Creates a new MTGService instance with a default HTTP client
    #[allow(dead_code)]
    pub fn new_with_reqwest() -> Self {
        Self::new(MTGClientType::HTTP(reqwest::Client::new()))
    }

    /// Gets the path for compressed set archives
    fn get_set_archive_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2", set_code))
    }

    /// Gets the path for set checksums
    fn get_set_checksum_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2.sha256", set_code))
    }

    /// Gets the path for set version information

    fn get_set_version_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2.version", set_code))
    }

    /// Fetches metadata about the current MTGJSON version
    ///
    /// This includes the current version number and update date.
    /// Results are cached to avoid unnecessary API calls.

    pub async fn fetch_meta(&self) -> Result<MTGJSONMeta, Error> {
        let mut meta = self.meta.lock().await;
        if meta.is_some() {
            return Ok(meta.as_ref().unwrap().clone());
        }

        let url = "https://mtgjson.com/api/v5/Meta.json";
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let meta_response: MTGJSONMetaResponse = response.json().await?;

        let meta_data = MTGJSONMeta {
            date: meta_response.data.date,
            version: meta_response.data.version,
            checksums: HashMap::new(),
        };
        *meta = Some(meta_data.clone());
        Ok(meta_data)
    }

    /// Verifies the integrity of a cached set file
    ///
    /// Checks both the version and checksum of the file against
    /// the current MTGJSON version.

    pub async fn verify_file_checksum(&self, set_code: &str, path: &Path) -> Result<bool, Error> {
        // First check if we have a version file and if it matches current version
        let version_path = self.get_set_version_path(set_code);
        let current_meta = self.fetch_meta().await?;

        if version_path.exists() {
            let stored_version = fs::read_to_string(&version_path)?;
            if stored_version.trim() != current_meta.version {
                return Ok(false); // Version mismatch, need to refresh cache
            }
        } else {
            return Ok(false); // No version file, need to refresh cache
        }

        // Check if we have a local checksum file
        let checksum_path = self.get_set_checksum_path(set_code);
        if checksum_path.exists() {
            let stored_checksum = fs::read_to_string(&checksum_path)?;
            let contents = fs::read(path)?;
            let mut hasher = Sha256::new();
            hasher.update(&contents);
            let hash = format!("{:x}", hasher.finalize());
            return Ok(hash == stored_checksum.trim());
        }

        Ok(false)
    }

    /// Saves a set's data to the disk cache
    ///
    /// Stores the compressed data along with its checksum and version
    /// information for future validation.
    #[allow(dead_code)]
    async fn save_cache_to_disk(
        &self,
        set_code: &str,
        compressed_data: &[u8],
    ) -> Result<(), Error> {
        // Create the sets directory if it doesn't exist
        fs::create_dir_all("sets")?;

        // Save the compressed bz2 file
        let set_archive_path = self.get_set_archive_path(set_code);
        fs::write(&set_archive_path, compressed_data)?;

        // Calculate and save the checksum of the compressed data
        let mut hasher = Sha256::new();
        hasher.update(compressed_data);
        let hash = format!("{:x}", hasher.finalize());
        let checksum_path = self.get_set_checksum_path(set_code);
        fs::write(&checksum_path, &hash)?;

        // Save the current version
        let meta = self.fetch_meta().await?;
        let version_path = self.get_set_version_path(set_code);
        fs::write(&version_path, &meta.version)?;

        Ok(())
    }

    /// Fetches a set by its code, using caching when possible
    ///
    /// This method implements a multi-level caching strategy:
    /// 1. First checks the in-memory cache
    /// 2. Then checks the disk cache
    /// 3. Finally falls back to fetching from the API
    ///
    /// Cache validation includes both version checking and checksum verification.
    #[allow(dead_code)]
    pub async fn fetch_set(&self, set_code: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        // Check memory cache first
        let memory_cache = self.cache.lock().await;
        if let Some(cards) = memory_cache.get(set_code) {
            return Ok(cards.clone());
        }
        drop(memory_cache);

        // Check if we have a cached version and its checksum is valid
        let set_archive_path = self.get_set_archive_path(set_code);
        if set_archive_path.exists() {
            if self
                .verify_file_checksum(set_code, &set_archive_path)
                .await?
            {
                let compressed_data = fs::read(&set_archive_path)?;
                let decompressed = bzip2::read::BzDecoder::new(&compressed_data[..]);
                let set: MTGJSONSetResponse = serde_json::from_reader(decompressed)?;
                let cards: Vec<Card> = set
                    .data
                    .cards
                    .into_iter()
                    .filter_map(convert_mtgjson_to_card)
                    .map(|(card, _, _, _, _, _, _)| card)
                    .collect();

                // Update memory cache
                let mut memory_cache = self.cache.lock().await;
                memory_cache.insert(set_code.to_string(), cards.clone());

                return Ok(cards);
            }
        }

        // Get the set data from the client
        let set = self.client.fetch_set(set_code).await?;

        // Create a complete response with meta data
        let meta = self.fetch_meta().await?;
        let response = MTGJSONSetResponse {
            data: set,
            meta: MTGJSONSetMeta {
                version: meta.version,
                date: meta.date,
            },
        };

        // Convert to JSON and compress
        let json_data = serde_json::to_string(&response)?;
        let mut compressed = Vec::new();
        {
            let mut compressor =
                bzip2::write::BzEncoder::new(&mut compressed, bzip2::Compression::best());
            compressor.write_all(json_data.as_bytes())?;
            compressor.finish()?;
        }

        // Save compressed data to disk cache
        self.save_cache_to_disk(set_code, &compressed).await?;

        // Convert to our internal format
        let cards: Vec<Card> = response
            .data
            .cards
            .into_iter()
            .filter_map(convert_mtgjson_to_card)
            .map(|(card, _, _, _, _, _, _)| card)
            .collect();

        // Update memory cache
        let mut memory_cache = self.cache.lock().await;
        memory_cache.insert(set_code.to_string(), cards.clone());

        Ok(cards)
    }

    /// Fetches multiple sets in sequence
    ///
    /// Returns a combined vector of all cards from the specified sets.
    #[allow(dead_code)]
    pub async fn fetch_multiple_sets(
        &self,
        set_codes: &[&str],
    ) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let mut all_cards = Vec::new();
        for set_code in set_codes {
            let cards = self.fetch_set(set_code).await?;
            all_cards.extend(cards);
        }
        Ok(all_cards)
    }

    /// Fetches the list of all available sets
    ///
    /// Filters the sets based on various criteria:
    /// - Only includes main sets, expansions, and special sets
    /// - Excludes unreleased sets
    /// - Excludes empty sets
    ///
    /// Returns a vector of set codes sorted by release date (newest first)
    #[allow(dead_code)]
    pub async fn fetch_set_list(&self) -> Result<Vec<String>, Error> {
        let url = "https://mtgjson.com/api/v5/SetList.json";
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let set_list: MTGJSONSetList = response.json().await?;

        // Get current date in YYYY-MM-DD format
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Filter out non-standard sets, unreleased sets, and empty sets
        let mut sets: Vec<_> = set_list
            .data
            .into_iter()
            .filter(|set| {
                // Include only main sets, expansions, and special sets
                matches!(
                    set.set_type.as_str(),
                    "core"
                        | "expansion"
                        | "masters"
                        | "draft_innovation"
                        | "commander"
                        | "funny"
                        | "starter"
                        | "promo"
                        | "box"
                        | "duel_deck"
                        | "premium_deck"
                        | "from_the_vault"
                        | "spellbook"
                        | "masterpiece"
                ) && 
                // Filter out future sets and empty sets
                set.release_date <= current_date &&
                !set.is_partial_preview &&
                set.total_set_size > 0
            })
            .collect();

        // Sort by release date (newest first)
        sets.sort_by(|a, b| b.release_date.cmp(&a.release_date));

        Ok(sets.into_iter().map(|s| s.code).collect())
    }
}

/// Convert an MTGJSONCard to our internal Card format
pub fn convert_mtgjson_to_card(
    mtg_card: MTGJSONCard,
) -> Option<(
    Card,
    CardName,
    CardCost,
    CardTypeInfo,
    CardDetailsComponent,
    CardRulesText,
    CardKeywords,
)> {
    // Parse the mana cost
    let mana_cost = parse_mana_cost(&mtg_card.mana_cost.unwrap());

    // Get the card types
    let types = determine_card_type(
        &mtg_card.types,
        Some(&mtg_card.supertypes),
        Some(&mtg_card.subtypes),
    )?;

    // Process card details based on type
    let card_details = if types.contains(CardTypes::CREATURE) {
        CardDetails::Creature(CreatureCard {
            power: mtg_card
                .power
                .as_deref()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            toughness: mtg_card
                .toughness
                .as_deref()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            creature_type: determine_creature_types(
                &mtg_card.subtypes,
                &mtg_card.name,
                mtg_card.text.as_deref().unwrap_or(""),
            ),
        })
    } else {
        CardDetails::Other
    };

    let rules_text = mtg_card.text.unwrap_or_default();
    let name = mtg_card.name;

    // Create the card and return it with its components
    let card = Card::new(&name, mana_cost, types, card_details, &rules_text);

    // Return the card and its individual components
    Some(card.get_components())
}

/// Determines the card types from type strings
///
/// Processes three levels of type information:
/// - Basic types (Creature, Instant, etc.)
/// - Supertypes (Legendary, Basic, etc.)
/// - Subtypes (Equipment, Saga, etc.)
///
/// Returns None if no valid types are found
pub fn determine_card_type(
    types: &[String],
    supertypes: Option<&Vec<String>>,
    subtypes: Option<&Vec<String>>,
) -> Option<CardTypes> {
    let mut card_types = CardTypes::NONE;

    // Process basic types
    for type_str in types {
        match type_str.as_str() {
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
            _ => continue,
        }
    }

    // Process supertypes
    if let Some(supertypes) = supertypes {
        for supertype in supertypes {
            match supertype.as_str() {
                "Basic" => card_types |= CardTypes::BASIC,
                "Legendary" => card_types |= CardTypes::LEGENDARY,
                "Ongoing" => card_types |= CardTypes::ONGOING,
                "Snow" => card_types |= CardTypes::SNOW,
                "World" => card_types |= CardTypes::WORLD,
                _ => continue,
            }
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
                _ => continue,
            }
        }
    }

    if card_types == CardTypes::NONE {
        None
    } else {
        Some(card_types)
    }
}

/// Determines creature types from subtypes and card text
///
/// This function processes multiple sources of type information:
/// - Explicit subtypes from the card
/// - Types inferred from the card name
/// - Types inferred from rules text
///
/// Returns a CreatureType bitflag with all applicable types
pub fn determine_creature_types(subtypes: &[String], name: &str, text: &str) -> CreatureType {
    let mut creature_types = CreatureType::NONE;

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
            "Sphinx" => creature_types |= CreatureType::SPHINX,
            "Imp" => creature_types |= CreatureType::IMP,
            "Gargoyle" => creature_types |= CreatureType::GARGOYLE,
            "Lhurgoyf" => creature_types |= CreatureType::LHURGOYF,
            "Ooze" => creature_types |= CreatureType::OOZE,
            "Squirrel" => creature_types |= CreatureType::SQUIRREL,
            "Kavu" => creature_types |= CreatureType::KAVU,
            "Cat" => creature_types |= CreatureType::CAT,
            "Drake" => creature_types |= CreatureType::DRAKE,
            "Gnome" => creature_types |= CreatureType::GNOME,
            "Archon" => creature_types |= CreatureType::ARCHON,
            "Lizard" => creature_types |= CreatureType::LIZARD,
            "Insect" => creature_types |= CreatureType::INSECT,
            "Construct" => creature_types |= CreatureType::CONSTRUCT,
            "Golem" => creature_types |= CreatureType::GOLEM,
            "Monkey" => creature_types |= CreatureType::MONKEY,
            "Nymph" => creature_types |= CreatureType::NYMPH,
            "Efreet" => creature_types |= CreatureType::EFREET,
            "Incarnation" => creature_types |= CreatureType::INCARNATION,
            "Dryad" => creature_types |= CreatureType::DRYAD,
            "Treefolk" => creature_types |= CreatureType::TREEFOLK,
            "Sliver" => creature_types |= CreatureType::SLIVER,
            "Snake" => creature_types |= CreatureType::SNAKE,
            "Wolf" => creature_types |= CreatureType::WOLF,
            "Werewolf" => creature_types |= CreatureType::WEREWOLF,
            "Scout" => creature_types |= CreatureType::SCOUT,
            "Advisor" => creature_types |= CreatureType::ADVISOR,
            "Ally" => creature_types |= CreatureType::ALLY,
            "Mercenary" => creature_types |= CreatureType::MERCENARY,
            "Rebel" => creature_types |= CreatureType::REBEL,
            "Spider" => creature_types |= CreatureType::SPIDER,
            _ => continue,
        }
    }

    // Apply retroactive types based on name
    creature_types = CreatureType::apply_retroactive_types(name, creature_types);

    // Apply types based on text
    let text = format!("{} {}", name, text).to_lowercase();
    creature_types = CreatureType::infer_from_name_and_text(&text, creature_types);

    creature_types
}

/// Parse a mana cost string into a Mana struct
fn parse_mana_cost(mana_cost: &str) -> Mana {
    let mut result = Mana::default();
    let mut generic_mana = 0;

    // Extract all mana symbols from the string
    let re = regex::Regex::new(r"\{([^}]+)\}").unwrap();
    for cap in re.captures_iter(mana_cost) {
        let symbol = cap.get(1).unwrap().as_str();
        match symbol {
            "W" => result.white += 1,
            "U" => result.blue += 1,
            "B" => result.black += 1,
            "R" => result.red += 1,
            "G" => result.green += 1,
            "C" => result.colorless += 1,
            // Handle generic mana (numbers)
            s if s.parse::<u64>().is_ok() => {
                generic_mana += s.parse::<u64>().unwrap();
            }
            // Ignore other symbols for now (hybrid, phyrexian, etc.)
            _ => {}
        }
    }

    // Set generic mana
    result.colorless = generic_mana;

    // Set color flags
    let mut color = ManaColor::NONE;
    if result.white > 0 {
        color |= ManaColor::WHITE;
    }
    if result.blue > 0 {
        color |= ManaColor::BLUE;
    }
    if result.black > 0 {
        color |= ManaColor::BLACK;
    }
    if result.red > 0 {
        color |= ManaColor::RED;
    }
    if result.green > 0 {
        color |= ManaColor::GREEN;
    }
    if result.colorless > 0 {
        color |= ManaColor::COLORLESS;
    }
    result.color = color;
    result.reflectable_color = color.into();

    result
}

/// Default language for cards (English)
fn default_language() -> String {
    "English".to_string()
}

/// Default rarity for cards (Common)
fn default_rarity() -> String {
    "common".to_string()
}

/// Structure containing information about a set from the SetList endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetList {
    /// List of available sets
    pub data: Vec<MTGJSONSetInfo>,
    /// Metadata about the response
    pub meta: MTGJSONMetaData,
}

/// Information about a single set from the SetList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetInfo {
    /// Set code (e.g., "M21")
    pub code: String,
    /// Set name
    pub name: String,
    /// Type of set (core, expansion, etc.)
    #[serde(rename = "type")]
    pub set_type: String,
    /// Release date (YYYY-MM-DD)
    #[serde(rename = "releaseDate", default)]
    pub release_date: String,
    /// Number of unique cards
    #[serde(rename = "baseSetSize", default)]
    pub base_set_size: i32,
    /// Total number of cards including variants
    #[serde(rename = "totalSetSize", default)]
    pub total_set_size: i32,
    /// Whether only available in foil
    #[serde(rename = "isFoilOnly", default)]
    pub is_foil_only: bool,
    /// Whether only available online
    #[serde(rename = "isOnlineOnly", default)]
    pub is_online_only: bool,
    /// Code for set symbol font
    #[serde(rename = "keyruneCode", default)]
    pub keyrune_code: String,
    /// Whether this is an unreleased preview
    #[serde(rename = "isPartialPreview", default)]
    pub is_partial_preview: bool,
    /// Available languages
    #[serde(default)]
    pub languages: Vec<String>,
    /// Set name translations
    #[serde(default)]
    pub translations: HashMap<String, Option<String>>,
}

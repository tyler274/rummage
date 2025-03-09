use crate::card::{
    ArtifactCard, Card, CardDetails, CardTypes, CreatureCard, CreatureType, EnchantmentCard,
    LandCard, SpellCard, SpellType,
};
use crate::mana::{Color, Mana};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{sleep, Duration};
use std::time::Instant;
use lazy_static::lazy_static;

pub mod test_utils;

use test_utils::MockClient;

lazy_static! {
    static ref RATE_LIMITER: Arc<TokioMutex<Instant>> = Arc::new(TokioMutex::new(Instant::now()));
}

const RATE_LIMIT_DURATION: Duration = Duration::from_millis(100); // 10 requests per second max

#[allow(dead_code)]
type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMetaResponse {
    pub meta: MTGJSONMetaData,
    pub data: MTGJSONMetaData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMetaData {
    pub date: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONMeta {
    pub date: String,
    pub version: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MTGJSONSetMeta {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetResponse {
    pub data: MTGJSONSet,
    pub meta: MTGJSONSetMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSet {
    #[serde(default)]
    pub artist_ids: Option<Vec<String>>,
    #[serde(default)]
    pub availability: Vec<String>,
    pub cards: Vec<MTGJSONCard>,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub total_set_size: i32,
    #[serde(default)]
    pub release_date: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub booster: Option<serde_json::Value>,
    #[serde(default)]
    pub sealed_product: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub tokens: Option<Vec<MTGJSONCard>>,
    #[serde(default)]
    pub translations: Option<HashMap<String, Option<String>>>,
    #[serde(default)]
    pub base_set_size: Option<i32>,
    #[serde(default)]
    pub block: Option<String>,
    #[serde(default)]
    pub is_foreign_only: Option<bool>,
    #[serde(default)]
    pub is_partial_preview: Option<bool>,
    #[serde(default)]
    pub is_online_only: Option<bool>,
    #[serde(default)]
    pub keyrunecode: Option<String>,
    #[serde(default)]
    pub mcm_id: Option<i32>,
    #[serde(default)]
    pub mcm_name: Option<String>,
    #[serde(default)]
    pub mtgo_code: Option<String>,
    #[serde(default)]
    pub tcgplayer_group_id: Option<i32>,
    #[serde(default)]
    pub meta: Option<MTGJSONSetMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONCard {
    pub artist: Option<String>,
    #[serde(rename = "artistIds")]
    pub artist_ids: Option<Vec<String>>,
    pub availability: Vec<String>,
    #[serde(rename = "borderColor")]
    pub border_color: String,
    #[serde(rename = "colorIdentity")]
    pub color_identity: Vec<String>,
    pub colors: Option<Vec<String>>,
    #[serde(rename = "convertedManaCost")]
    pub converted_mana_cost: Option<f32>,
    #[serde(rename = "edhrecRank")]
    pub edhrec_rank: Option<i32>,
    pub finishes: Vec<String>,
    #[serde(rename = "foreignData")]
    pub foreign_data: Option<Vec<serde_json::Value>>,
    #[serde(rename = "frameVersion")]
    pub frame_version: String,
    #[serde(rename = "hasFoil")]
    pub has_foil: bool,
    #[serde(rename = "hasNonFoil")]
    pub has_non_foil: bool,
    pub identifiers: MTGJSONCardIdentifiers,
    #[serde(rename = "isReprint")]
    pub is_reprint: Option<bool>,
    #[serde(rename = "isStarter")]
    pub is_starter: Option<bool>,
    pub keywords: Option<Vec<String>>,
    #[serde(default = "default_language")]
    pub language: String,
    pub layout: String,
    #[serde(default)]
    pub legalities: HashMap<String, String>,
    #[serde(rename = "manaCost")]
    pub mana_cost: Option<String>,
    #[serde(rename = "manaValue")]
    pub mana_value: Option<f32>,
    pub name: String,
    pub number: String,
    pub power: Option<String>,
    #[serde(default)]
    pub printings: Vec<String>,
    #[serde(rename = "purchaseUrls")]
    pub purchase_urls: Option<HashMap<String, String>>,
    #[serde(default = "default_rarity")]
    pub rarity: String,
    pub rulings: Option<Vec<MTGJSONRuling>>,
    #[serde(rename = "securityStamp")]
    pub security_stamp: Option<String>,
    #[serde(rename = "setCode")]
    pub set_code: String,
    #[serde(rename = "sourceProducts")]
    pub source_products: Option<HashMap<String, Vec<String>>>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub types: Vec<String>,
    pub uuid: String,
    pub variations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONCardIdentifiers {
    #[serde(rename = "cardKingdomId")]
    pub card_kingdom_id: Option<String>,
    #[serde(rename = "cardKingdomFoilId")]
    pub card_kingdom_foil_id: Option<String>,
    #[serde(rename = "mtgjsonV4Id")]
    pub mtgjson_v4_id: Option<String>,
    #[serde(rename = "scryfallCardBackId")]
    pub scryfall_card_back_id: Option<String>,
    #[serde(rename = "scryfallId")]
    pub scryfall_id: Option<String>,
    #[serde(rename = "scryfallIllustrationId")]
    pub scryfall_illustration_id: Option<String>,
    #[serde(rename = "scryfallOracleId")]
    pub scryfall_oracle_id: Option<String>,
    #[serde(rename = "tcgplayerProductId")]
    pub tcgplayer_product_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONRuling {
    pub date: String,
    pub text: String,
}

#[async_trait]
pub trait MTGClient: Send + Sync {
    #[allow(dead_code)]
    async fn fetch_set(&self, set_code: &str) -> Result<MTGJSONSet, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MTGClientType {
    HTTP(reqwest::Client),
    Mock(Arc<MockClient>),
}

impl MTGClientType {
    #[allow(dead_code)]
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

#[allow(dead_code)]
pub struct MTGService {
    client: MTGClientType,
    cache: Arc<TokioMutex<HashMap<String, Vec<Card>>>>,
    meta: Arc<TokioMutex<Option<MTGJSONMeta>>>,
}

impl MTGService {
    #[allow(dead_code)]
    pub fn new(client: MTGClientType) -> Self {
        Self {
            client,
            cache: Arc::new(TokioMutex::new(HashMap::new())),
            meta: Arc::new(TokioMutex::new(None)),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_reqwest() -> Self {
        Self::new(MTGClientType::HTTP(reqwest::Client::new()))
    }

    #[allow(dead_code)]
    fn get_raw_cache_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json", set_code))
    }

    #[allow(dead_code)]
    fn get_set_archive_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2", set_code))
    }

    #[allow(dead_code)]
    fn get_set_checksum_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2.sha256", set_code))
    }

    #[allow(dead_code)]
    fn get_set_version_path(&self, set_code: &str) -> std::path::PathBuf {
        std::path::PathBuf::from("sets").join(format!("{}.json.bz2.version", set_code))
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
            .collect();

        // Update memory cache
        let mut memory_cache = self.cache.lock().await;
        memory_cache.insert(set_code.to_string(), cards.clone());

        Ok(cards)
    }

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

pub fn convert_mtgjson_to_card(mtg_card: MTGJSONCard) -> Option<Card> {
    let types = determine_card_type(
        &mtg_card.types,
        Some(&mtg_card.supertypes),
        Some(&mtg_card.subtypes),
    )?;
    let mana_cost = parse_mana_cost(mtg_card.mana_cost.as_deref().unwrap_or(""));

    let card_details = if types.contains(CardTypes::CREATURE) {
        let power = mtg_card.power.and_then(|p| p.parse().ok()).unwrap_or(0);
        let toughness = mtg_card.toughness.and_then(|t| t.parse().ok()).unwrap_or(0);
        let creature_type = determine_creature_types(
            &mtg_card.subtypes,
            &mtg_card.name,
            mtg_card.text.as_deref().unwrap_or(""),
        );
        CardDetails::Creature(CreatureCard {
            power,
            toughness,
            creature_type,
        })
    } else if types.contains(CardTypes::INSTANT) {
        CardDetails::Instant(SpellCard {
            spell_type: SpellType::Instant,
            targets: Vec::new(), // TODO: Parse targets from rules text
        })
    } else if types.contains(CardTypes::SORCERY) {
        CardDetails::Sorcery(SpellCard {
            spell_type: SpellType::Sorcery,
            targets: Vec::new(), // TODO: Parse targets from rules text
        })
    } else if types.contains(CardTypes::ENCHANTMENT) {
        CardDetails::Enchantment(EnchantmentCard {
            enchantment_type: mtg_card.subtypes.first().cloned(),
        })
    } else if types.contains(CardTypes::ARTIFACT) {
        CardDetails::Artifact(ArtifactCard {
            artifact_type: mtg_card.subtypes.first().cloned(),
        })
    } else if types.contains(CardTypes::LAND) {
        CardDetails::Land(LandCard {
            land_type: mtg_card.subtypes.first().cloned(),
            produces: Vec::new(), // TODO: Parse mana production from rules text
        })
    } else {
        CardDetails::Other
    };

    Some(Card {
        name: mtg_card.name,
        cost: mana_cost,
        types,
        card_details,
        rules_text: mtg_card.text.unwrap_or_default(),
    })
}

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

pub fn parse_mana_cost(cost: &str) -> Mana {
    let mut mana = Mana::default();
    let mut in_brace = false;
    let mut current_number = String::new();

    for c in cost.chars() {
        match c {
            '{' => in_brace = true,
            '}' => {
                in_brace = false;
                if !current_number.is_empty() {
                    if let Ok(n) = current_number.parse::<u64>() {
                        mana.colorless = n;
                    }
                    current_number.clear();
                }
            }
            'W' => {
                if in_brace {
                    mana.white += 1;
                    mana.color |= Color::WHITE;
                }
            }
            'U' => {
                if in_brace {
                    mana.blue += 1;
                    mana.color |= Color::BLUE;
                }
            }
            'B' => {
                if in_brace {
                    mana.black += 1;
                    mana.color |= Color::BLACK;
                }
            }
            'R' => {
                if in_brace {
                    mana.red += 1;
                    mana.color |= Color::RED;
                }
            }
            'G' => {
                if in_brace {
                    mana.green += 1;
                    mana.color |= Color::GREEN;
                }
            }
            n if n.is_ascii_digit() && in_brace => {
                current_number.push(n);
            }
            _ => {}
        }
    }

    mana
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetList {
    pub data: Vec<MTGJSONSetInfo>,
    pub meta: MTGJSONMetaData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTGJSONSetInfo {
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub set_type: String,
    #[serde(rename = "releaseDate", default)]
    pub release_date: String,
    #[serde(rename = "baseSetSize", default)]
    pub base_set_size: i32,
    #[serde(rename = "totalSetSize", default)]
    pub total_set_size: i32,
    #[serde(rename = "isFoilOnly", default)]
    pub is_foil_only: bool,
    #[serde(rename = "isOnlineOnly", default)]
    pub is_online_only: bool,
    #[serde(rename = "keyruneCode", default)]
    pub keyrune_code: String,
    #[serde(rename = "isPartialPreview", default)]
    pub is_partial_preview: bool,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub translations: HashMap<String, Option<String>>,
}

fn default_rarity() -> String {
    "common".to_string()
}

fn default_language() -> String {
    "English".to_string()
}

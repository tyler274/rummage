use rummage::card::{Card, CardDetails, CardTypes};
use rummage::cards::mtgjson::{MTGJSONSetResponse, MTGService};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio;

async fn validate_set_import(
    service: &MTGService,
    set_code: &str,
) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    println!("Validating set: {}", set_code);

    // Try up to 3 times with exponential backoff
    let mut retry_count = 0;
    let mut last_error = None;
    while retry_count < 3 {
        match service.fetch_set(set_code).await {
            Ok(cards) => {
                // Skip empty set validation for certain sets that might be empty
                // SUNF is a special case set that can be empty
                if set_code != "SUNF" {
                    assert!(!cards.is_empty(), "Set {} should contain cards", set_code);
                }

                // Validate cache files
                let bz2_path = Path::new("sets").join(format!("{}.json.bz2", set_code));
                let sha256_path = Path::new("sets").join(format!("{}.json.bz2.sha256", set_code));
                let version_path = Path::new("sets").join(format!("{}.json.bz2.version", set_code));

                assert!(bz2_path.exists(), "BZ2 file should exist for {}", set_code);
                assert!(
                    sha256_path.exists(),
                    "SHA256 file should exist for {}",
                    set_code
                );
                assert!(
                    version_path.exists(),
                    "Version file should exist for {}",
                    set_code
                );

                // Validate checksum
                let checksum_valid = service.verify_file_checksum(set_code, &bz2_path).await?;
                assert!(checksum_valid, "Checksum should be valid for {}", set_code);

                // Validate BZ2 file can be decompressed and contains valid JSON
                let compressed_data = fs::read(&bz2_path)?;
                let decompressed = bzip2::read::BzDecoder::new(&compressed_data[..]);
                let _set_response: MTGJSONSetResponse = serde_json::from_reader(decompressed)?;

                // Collect statistics
                let mut stats = CardSetStatistics::default();
                for card in &cards {
                    stats.update(card);
                }

                println!("Statistics for set {}:", set_code);
                println!("{:#?}", stats);

                return Ok(cards);
            }
            Err(e) => {
                println!(
                    "Attempt {} failed for set {}: {}",
                    retry_count + 1,
                    set_code,
                    e
                );
                last_error = Some(e);
                retry_count += 1;
                if retry_count < 3 {
                    // Exponential backoff: 1s, 2s, 4s
                    tokio::time::sleep(Duration::from_secs(1 << retry_count)).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Maximum retries exceeded".into()))
}

#[derive(Debug, Default)]
struct CardSetStatistics {
    total_cards: usize,
    creature_count: usize,
    instant_count: usize,
    sorcery_count: usize,
    enchantment_count: usize,
    artifact_count: usize,
    land_count: usize,
    planeswalker_count: usize,
    unique_creature_types: HashSet<String>,
    unique_card_types: HashSet<String>,
}

impl CardSetStatistics {
    fn update(&mut self, card: &Card) {
        self.total_cards += 1;

        // Count card types
        if card.types.contains(CardTypes::CREATURE) {
            self.creature_count += 1;
        }
        if card.types.contains(CardTypes::INSTANT) {
            self.instant_count += 1;
        }
        if card.types.contains(CardTypes::SORCERY) {
            self.sorcery_count += 1;
        }
        if card.types.contains(CardTypes::ENCHANTMENT) {
            self.enchantment_count += 1;
        }
        if card.types.contains(CardTypes::ARTIFACT) {
            self.artifact_count += 1;
        }
        if card.types.contains(CardTypes::LAND) {
            self.land_count += 1;
        }
        if card.types.contains(CardTypes::PLANESWALKER) {
            self.planeswalker_count += 1;
        }

        // Collect unique types
        if let CardDetails::Creature(creature) = &card.card_details {
            // Convert CreatureType bits to strings
            let type_strings = format!("{:?}", creature.creature_type);
            for type_str in type_strings.split('|') {
                self.unique_creature_types
                    .insert(type_str.trim().to_string());
            }
        }

        // Collect unique card types
        let type_strings = format!("{:?}", card.types);
        for type_str in type_strings.split('|') {
            self.unique_card_types.insert(type_str.trim().to_string());
        }
    }
}

#[cfg(feature = "all_set_importer")]
#[tokio::test]
async fn test_import_all_sets() -> Result<(), Box<dyn std::error::Error>> {
    // Skip test in CI environment
    if std::env::var("CI").is_ok() || std::env::var("SKIP_NETWORK_TESTS").is_ok() {
        return Ok(());
    }

    // Clean up and create cache directory
    let _ = fs::remove_dir_all("sets");
    fs::create_dir_all("sets")?;

    let service = MTGService::new_with_reqwest();

    // Test a few key sets first
    let key_sets = [
        "30A", "MH2", "2X2", "DMU", "BRO", "ONE", "MAT", "MOM", "LCI", "MKM", "WOE", "LTR",
    ];

    println!("Testing key sets...");
    for set_code in key_sets.iter() {
        let cards = validate_set_import(&service, set_code).await?;
        println!(
            "Successfully imported {} cards from {}",
            cards.len(),
            set_code
        );
    }

    // Fetch all sets from the API
    println!("\nFetching complete set list...");
    let all_sets = service.fetch_set_list().await?;
    println!("Found {} sets", all_sets.len());

    // Process sets in chunks to avoid overwhelming the API
    const CHUNK_SIZE: usize = 5;
    for chunk in all_sets.chunks(CHUNK_SIZE) {
        for set_code in chunk {
            match validate_set_import(&service, set_code).await {
                Ok(cards) => {
                    println!(
                        "Successfully imported {} cards from {}",
                        cards.len(),
                        set_code
                    );
                }
                Err(e) => {
                    println!("Failed to import set {}: {}", set_code, e);
                }
            }
        }
        // Add a small delay between chunks to be nice to the API
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    // Collect all unique creature types and card types
    let mut all_creature_types = HashSet::new();
    let mut all_card_types = HashSet::new();

    for set_code in all_sets {
        if let Ok(cards) = service.fetch_set(&set_code).await {
            for card in &cards {
                if let CardDetails::Creature(creature) = &card.card_details {
                    let type_strings = format!("{:?}", creature.creature_type);
                    for type_str in type_strings.split('|') {
                        all_creature_types.insert(type_str.trim().to_string());
                    }
                }

                let type_strings = format!("{:?}", card.types);
                for type_str in type_strings.split('|') {
                    all_card_types.insert(type_str.trim().to_string());
                }
            }
        }
    }

    println!("\nUnique creature types found:");
    for creature_type in &all_creature_types {
        println!("  {}", creature_type);
    }

    println!("\nUnique card types found:");
    for card_type in &all_card_types {
        println!("  {}", card_type);
    }

    // TODO: Compare found types with our model and suggest additions

    Ok(())
}

#[tokio::test]
async fn test_cache_invalidation() -> Result<(), Box<dyn std::error::Error>> {
    // Skip test in CI environment
    if std::env::var("CI").is_ok() || std::env::var("SKIP_NETWORK_TESTS").is_ok() {
        return Ok(());
    }

    let service = MTGService::new_with_reqwest();
    let test_set = "30A";

    // First fetch to populate cache
    let initial_cards = service.fetch_set(test_set).await?;

    // Corrupt the cache file
    let bz2_path = Path::new("sets").join(format!("{}.json.bz2", test_set));
    fs::write(&bz2_path, "corrupted data")?;

    // Fetch again - should detect corruption and re-download
    let revalidated_cards = service.fetch_set(test_set).await?;

    assert_eq!(
        initial_cards.len(),
        revalidated_cards.len(),
        "Card count should match after cache invalidation"
    );

    Ok(())
}

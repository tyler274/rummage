use bevy::prelude::*;
use bevy_persistent::Storage;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::mana::ManaPool;
use crate::player::Player;

mod data;
mod events;
mod plugin;
mod resources;
mod systems;

// Re-export public API
pub use data::*;
pub use events::*;
pub use plugin::SaveLoadPlugin;
pub use resources::*;

/// Plugin for save and load game functionality
pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<CheckStateBasedActionsEvent>()
            .add_systems(Startup, setup_save_system)
            .add_systems(
                Update,
                (
                    handle_save_game.run_if(resource_exists::<GameState>()),
                    handle_load_game,
                    handle_auto_save,
                ),
            );
    }
}

/// Event to trigger saving the game
#[derive(Event)]
pub struct SaveGameEvent {
    pub slot_name: String,
}

/// Event to trigger loading a saved game
#[derive(Event)]
pub struct LoadGameEvent {
    pub slot_name: String,
}

/// Event for checking state-based actions
#[derive(Event)]
pub struct CheckStateBasedActionsEvent;

/// Configuration for the save system
#[derive(Resource)]
pub struct SaveConfig {
    pub save_directory: PathBuf,
    pub auto_save_enabled: bool,
    pub auto_save_frequency: usize, // How often to auto-save (in state-based action checks)
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            save_directory: PathBuf::from("saves"),
            auto_save_enabled: true,
            auto_save_frequency: 10, // Auto-save every 10 state-based action checks
        }
    }
}

/// Tracker for auto-saving
#[derive(Resource)]
pub struct AutoSaveTracker {
    pub counter: usize,
}

impl Default for AutoSaveTracker {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

/// Complete game save data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSaveData {
    pub game_state: GameStateData,
    pub players: Vec<PlayerData>,
    pub zones: ZoneData,
    pub commanders: CommanderData,
    pub save_version: String,
}

/// Serializable game state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
    pub lands_played: Vec<(usize, u32)>,
    pub main_phase_action_taken: bool,
    pub drawn_this_turn: Vec<usize>,
    pub eliminated_players: Vec<usize>,
    pub use_commander_damage: bool,
    pub commander_damage_threshold: u32,
    pub starting_life: i32,
}

/// Serializable player data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: usize,
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub player_index: usize,
    // Add other player-specific data
}

/// Serializable zone data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneData {
    // Serialize all zone contents
    // This will need customization based on how your ZoneManager works
}

/// Serializable commander data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderData {
    // Serialize all commander-specific data
    // This will need customization based on how your CommandZoneManager works
}

/// System to set up the save system on startup
fn setup_save_system(mut commands: Commands) {
    // Create save directory if it doesn't exist
    let config = SaveConfig::default();
    std::fs::create_dir_all(&config.save_directory).unwrap_or_else(|e| {
        error!("Failed to create save directory: {}", e);
    });

    commands.insert_resource(config);
    commands.insert_resource(AutoSaveTracker::default());

    // Initialize persistent save metadata
    let save_metadata = Persistent::builder()
        .name("save_metadata")
        .format(StorageFormat::Bincode)
        .path("saves/metadata.bin")
        .default(SaveMetadata::default())
        .build()
        .expect("Failed to create persistent save metadata");

    commands.insert_resource(save_metadata);
}

/// Metadata about all saved games
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub saves: Vec<SaveInfo>,
}

impl Default for SaveMetadata {
    fn default() -> Self {
        Self { saves: Vec::new() }
    }
}

/// Information about a single save file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveInfo {
    pub slot_name: String,
    pub timestamp: u64,
    pub description: String,
    pub turn_number: u32,
    pub player_count: usize,
}

/// System to handle save game requests
fn handle_save_game(
    mut event_reader: EventReader<SaveGameEvent>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    zones: Option<Res<ZoneManager>>,
    commanders: Option<Res<CommandZoneManager>>,
    mut save_metadata: ResMut<Persistent<SaveMetadata>>,
    config: Res<SaveConfig>,
) {
    for event in event_reader.read() {
        info!("Saving game to slot: {}", event.slot_name);

        // Create save path
        let save_path = config
            .save_directory
            .join(format!("{}.bin", event.slot_name));

        let mut player_data = Vec::new();

        // Convert entity-based references to indices for serialization
        let mut entity_to_index = std::collections::HashMap::new();

        for (i, (entity, player)) in query_players.iter().enumerate() {
            entity_to_index.insert(entity, i);

            player_data.push(PlayerData {
                id: i,
                name: player.name.clone(),
                life: player.life,
                mana_pool: player.mana_pool,
                player_index: i,
                // Add other player data as needed
            });
        }

        // Transform GameState to serializable GameStateData
        let active_player_index = *entity_to_index.get(&game_state.active_player).unwrap_or(&0);
        let priority_holder_index = *entity_to_index
            .get(&game_state.priority_holder)
            .unwrap_or(&0);

        let turn_order_indices = game_state
            .turn_order
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let lands_played = game_state
            .lands_played
            .iter()
            .filter_map(|(e, count)| entity_to_index.get(e).map(|i| (*i, *count)))
            .collect();

        let drawn_this_turn = game_state
            .drawn_this_turn
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let eliminated_players = game_state
            .eliminated_players
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let game_state_data = GameStateData {
            turn_number: game_state.turn_number,
            active_player_index,
            priority_holder_index,
            turn_order_indices,
            lands_played,
            main_phase_action_taken: game_state.main_phase_action_taken,
            drawn_this_turn,
            eliminated_players,
            use_commander_damage: game_state.use_commander_damage,
            commander_damage_threshold: game_state.commander_damage_threshold,
            starting_life: game_state.starting_life,
        };

        // TODO: Extract and serialize zone data
        let zone_data = ZoneData {
            // Implement based on your ZoneManager structure
        };

        // TODO: Extract and serialize commander data
        let commander_data = CommanderData {
            // Implement based on your CommandZoneManager structure  
        };

        // Create complete save data
        let save_data = GameSaveData {
            game_state: game_state_data,
            players: player_data,
            zones: zone_data,
            commanders: commander_data,
            save_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        // Serialize and save the data
        if let Err(e) = bincode::serialize_into(
            std::fs::File::create(save_path).expect("Failed to create save file"),
            &save_data,
        ) {
            error!("Failed to save game: {}", e);
            continue;
        }

        // Update metadata
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let save_info = SaveInfo {
            slot_name: event.slot_name.clone(),
            timestamp,
            description: format!("Turn {}", game_state.turn_number),
            turn_number: game_state.turn_number,
            player_count: player_data.len(),
        };

        // Add or update save info in metadata
        if let Some(existing) = save_metadata
            .saves
            .iter_mut()
            .find(|s| s.slot_name == event.slot_name)
        {
            *existing = save_info;
        } else {
            save_metadata.saves.push(save_info);
        }

        // Save metadata
        if let Err(e) = save_metadata.persist() {
            error!("Failed to update save metadata: {}", e);
        }

        info!("Game saved successfully to slot: {}", event.slot_name);
    }
}

/// System to handle load game requests
fn handle_load_game(
    mut event_reader: EventReader<LoadGameEvent>,
    mut commands: Commands,
    config: Res<SaveConfig>,
) {
    for event in event_reader.read() {
        info!("Loading game from slot: {}", event.slot_name);

        // Create save path
        let save_path = config
            .save_directory
            .join(format!("{}.bin", event.slot_name));

        // Check if save file exists
        if !save_path.exists() {
            error!("Save file not found: {}", save_path.display());
            continue;
        }

        // Load and deserialize the save data
        let file = match std::fs::File::open(&save_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open save file: {}", e);
                continue;
            }
        };

        let save_data: GameSaveData = match bincode::deserialize_from(file) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to deserialize save data: {}", e);
                continue;
            }
        };

        // Clear existing game state
        // TODO: Implement clear_game_state to remove all relevant entities and resources
        // clear_game_state(&mut commands);

        // Reconstruct the game from the save data
        // This part depends heavily on your game structure and will need customization

        // 1. Create player entities first to establish entity mappings
        let mut index_to_entity = Vec::with_capacity(save_data.players.len());

        for player_data in &save_data.players {
            let player_entity = commands
                .spawn((
                    Player {
                        name: player_data.name.clone(),
                        life: player_data.life,
                        // Set other player fields
                    },
                    // Add other components as needed
                ))
                .id();

            // Store mapping from index to entity
            index_to_entity.push(player_entity);
        }

        // 2. Recreate the game state
        let game_state_data = &save_data.game_state;

        let active_player = index_to_entity[game_state_data.active_player_index];
        let priority_holder = index_to_entity[game_state_data.priority_holder_index];

        let turn_order = std::collections::VecDeque::from(
            game_state_data
                .turn_order_indices
                .iter()
                .map(|&i| index_to_entity[i])
                .collect::<Vec<_>>(),
        );

        let lands_played = game_state_data
            .lands_played
            .iter()
            .map(|(i, count)| (index_to_entity[*i], *count))
            .collect();

        let drawn_this_turn = game_state_data
            .drawn_this_turn
            .iter()
            .map(|&i| index_to_entity[i])
            .collect();

        let eliminated_players = game_state_data
            .eliminated_players
            .iter()
            .map(|&i| index_to_entity[i])
            .collect();

        let game_state = GameState {
            turn_number: game_state_data.turn_number,
            active_player,
            priority_holder,
            turn_order,
            lands_played,
            main_phase_action_taken: game_state_data.main_phase_action_taken,
            drawn_this_turn,
            state_based_actions_performed: false, // Reset this flag
            eliminated_players,
            use_commander_damage: game_state_data.use_commander_damage,
            commander_damage_threshold: game_state_data.commander_damage_threshold,
            starting_life: game_state_data.starting_life,
        };

        commands.insert_resource(game_state);

        // 3. Recreate zones
        // TODO: Implement zone recreation based on your ZoneManager structure

        // 4. Recreate commander state
        // TODO: Implement commander state recreation

        info!("Game loaded successfully from slot: {}", event.slot_name);
    }
}

/// Event for checking state-based actions
#[derive(Event)]
pub struct CheckStateBasedActionsEvent;

/// System to handle auto-saving
fn handle_auto_save(
    mut event_reader: EventReader<CheckStateBasedActionsEvent>,
    mut event_writer: EventWriter<SaveGameEvent>,
    mut auto_save_tracker: ResMut<AutoSaveTracker>,
    config: Res<SaveConfig>,
) {
    // Skip if auto-save is disabled
    if !config.auto_save_enabled {
        return;
    }

    for _ in event_reader.read() {
        auto_save_tracker.counter += 1;

        // Check if it's time to auto-save
        if auto_save_tracker.counter >= config.auto_save_frequency {
            auto_save_tracker.counter = 0;

            // Trigger auto-save
            event_writer.send(SaveGameEvent {
                slot_name: "auto_save".to_string(),
            });
        }
    }
}

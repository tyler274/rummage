use bevy::prelude::*;
use bevy_persistent::prelude::*;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::GameSaveData;
use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

use super::get_storage_path;

/// System to handle load game requests
pub fn handle_load_game(
    mut event_reader: EventReader<LoadGameEvent>,
    mut commands: Commands,
    config: Res<SaveConfig>,
    mut query_players: Query<(Entity, &mut Player)>,
    mut game_state: Option<ResMut<GameState>>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Loading game from slot: {}", event.slot_name);

        let save_path = get_storage_path(&config, &format!("{}.bin", event.slot_name));

        // Check if the save file exists (only on native platforms)
        #[cfg(not(target_arch = "wasm32"))]
        if !save_path.exists() {
            error!("Save file not found at: {:?}", save_path);
            continue;
        }

        // Create a persistent resource to load the save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(save_path)
            .default(GameSaveData::default())
            .build();

        match persistent_save {
            Ok(save) => {
                let save_data = save.clone();

                // Apply the loaded state using the fully qualified path
                crate::game_engine::save::systems::utils::apply_game_state(
                    &save_data,
                    &mut game_state,
                    &mut commands,
                    &mut query_players,
                    &mut zones,
                    &mut commanders,
                );

                info!("Game loaded successfully from slot {}", event.slot_name);
            }
            Err(e) => {
                error!("Failed to load save: {}", e);
            }
        }
    }
}

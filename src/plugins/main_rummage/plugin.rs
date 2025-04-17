use bevy::prelude::*;

use crate::cards::CardPlugin;
use crate::cards::drag::DragPlugin;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::zones::ZoneManager;
use crate::menu::GameMenuState;
use crate::player::{PlayerPlugin, resources::PlayerConfig};
use crate::text::DebugConfig;

use super::camera::{GameCameraSetupSet, ensure_game_camera_visible, setup_game_camera};
use super::diagnostics::check_card_status;
use super::setup::setup_game;
use super::visual_hand::spawn_player_visual_hands;
use super::zones::{connect_cards_to_zones, register_unzoned_cards};

// System to set the clear color for the game state
fn setup_clear_color(mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(Color::srgb(0.3, 0.3, 0.3)); // Set to a gray color
    info!("Set clear color for InGame state");
}

pub struct MainRummagePlugin;

impl Plugin for MainRummagePlugin {
    fn build(&self, app: &mut App) {
        // Add Player Plugin
        app.add_plugins(PlayerPlugin)
            // Add Card Plugin for card dragging and other card functionality
            .add_plugins(CardPlugin)
            // Add Drag Plugin for drag and drop functionality
            .add_plugins(DragPlugin)
            // Add Text Plugin for text rendering and debugging
            .add_plugins(crate::text::TextPlugin::default())
            // Initialize debug config resource
            .insert_resource(DebugConfig {
                show_text_positions: false,
            })
            // Add Save/Load system
            .add_plugins(SaveLoadPlugin)
            // Setup game configuration
            .insert_resource(
                PlayerConfig::new()
                    .with_player_count(4)
                    .with_spawn_all_cards(true)
                    .with_starting_life(40)
                    .with_card_size(Vec2::new(67.2, 93.6)) // Reduce card size to 1/10th original size
                    .with_card_spacing_multiplier(0.8) // Add tighter spacing between cards
                    .with_player_card_distance(300.0) // Reduce distance to bring players closer together
                    .with_player_card_offset(0, -100.0) // Bottom player - reduced offset
                    .with_player_card_offset(1, 0.0) // Right player
                    .with_player_card_offset(2, 100.0) // Top player - reduced offset
                    .with_player_card_offset(3, 0.0), // Left player
            )
            // Initialize zone manager resource
            .init_resource::<ZoneManager>()
            // Add game setup systems for InGame state
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (
                    setup_game,
                    // Put camera setup in its own set
                    setup_game_camera.in_set(GameCameraSetupSet),
                    ensure_game_camera_visible,
                    // Add the clear color setup system
                    setup_clear_color,
                ),
            )
            // System to connect cards to zones after they're spawned - moved to FixedUpdate
            .add_systems(
                Update,
                (
                    spawn_player_visual_hands,
                    connect_cards_to_zones,
                    check_card_status,
                    register_unzoned_cards.run_if(in_state(GameMenuState::InGame)),
                ),
            );
    }
}

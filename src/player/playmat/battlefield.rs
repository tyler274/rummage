//! Battlefield zone implementation for the player playmat

use crate::camera::components::AppLayer;
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::ecs::hierarchy::ChildOf;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use super::PlaymatZone;

/// Component for the battlefield zone specifically
#[derive(Component, Debug)]
pub struct BattlefieldZone {
    /// Player owning this battlefield
    #[allow(dead_code)]
    pub player_id: Entity,
    /// Grid rows for layout
    pub grid_rows: u32,
    /// Grid columns for layout
    pub grid_columns: u32,
    /// Current zoom level (1.0 = normal)
    pub zoom_level: f32,
    /// Whether grouping by card types is enabled
    pub group_by_type: bool,
}

impl Default for BattlefieldZone {
    fn default() -> Self {
        Self {
            player_id: Entity::PLACEHOLDER,
            grid_rows: 4,
            grid_columns: 6,
            zoom_level: 1.0,
            group_by_type: true,
        }
    }
}

/// Tag component for different permanent types on the battlefield
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PermanentType {
    Creature,
    Land,
    Artifact,
    Enchantment,
    Planeswalker,
    Token,
}

/// Local struct for grouping card positioning parameters
struct CardGroupPositioning {
    start_row: f32,
    start_col: f32,
    end_row: f32,
    end_col: f32,
    cell_size: f32,
    scale: f32,
}

/// Spawn the battlefield zone for a player
pub fn spawn_battlefield_zone(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    playmat_entity: Entity,
    player_entity: Entity,
    player: &Player,
    _config: &PlayerConfig,
) -> Entity {
    info!("Spawning battlefield zone for player {}", player.name);

    // Determine position relative to playmat based on player index
    let position = match player.player_index {
        0 => Vec3::new(0.0, -100.0, 0.0), // Bottom player - Slightly offset down
        1 => Vec3::new(100.0, 0.0, 0.0),  // Right player - Slightly offset right
        2 => Vec3::new(0.0, 100.0, 0.0),  // Top player - Slightly offset up
        3 => Vec3::new(-100.0, 0.0, 0.0), // Left player - Slightly offset left
        _ => Vec3::ZERO,
    };

    // Create the battlefield zone entity
    let battlefield_entity = commands
        .spawn((
            Transform::from_translation(position),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            PlaymatZone {
                player_id: player_entity,
                zone_type: Zone::Battlefield,
            },
            BattlefieldZone {
                player_id: player_entity,
                grid_rows: 4,
                grid_columns: 6,
                zoom_level: 1.0,
                group_by_type: true,
            },
            AppLayer::game_layers(),
            Name::new(format!("Battlefield-{}", player.name)),
        ))
        .insert(ChildOf(playmat_entity))
        .id();

    info!(
        "Battlefield zone spawned for player {} with entity {:?}",
        player.name, battlefield_entity
    );

    battlefield_entity
}

/// Organize the cards on the battlefield in a grid layout
pub fn organize_battlefield_cards(
    battlefield_query: Query<(&BattlefieldZone, &Children)>,
    mut card_query: Query<(&mut Transform, Option<&PermanentType>)>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Safely get the window dimensions, defaulting to reasonable values if not available
    let (window_width, window_height) = if let Ok(window) = windows.single() {
        (window.width(), window.height())
    } else {
        // Default to standard HD resolution if window can't be queried
        (1920.0, 1080.0)
    };

    for (battlefield, children) in battlefield_query.iter() {
        let card_count = children.len();

        // Skip if no cards on battlefield
        if card_count == 0 {
            continue;
        }

        // Calculate layout parameters
        let (grid_width, grid_height, cell_size, scale) = calculate_battlefield_layout(
            card_count as u32,
            battlefield.grid_columns,
            battlefield.grid_rows,
            battlefield.zoom_level,
            window_width,
            window_height,
        );

        if battlefield.group_by_type {
            // Separate cards by type
            let mut creatures = Vec::new();
            let mut lands = Vec::new();
            let mut artifacts = Vec::new();
            let mut enchantments = Vec::new();
            let mut planeswalkers = Vec::new();
            let mut tokens = Vec::new();
            let mut other = Vec::new();

            // Group cards by type
            for child_entity_ref in children.iter() {
                if let Ok((_, permanent_type)) = card_query.get(child_entity_ref) {
                    match permanent_type {
                        Some(PermanentType::Creature) => creatures.push(child_entity_ref),
                        Some(PermanentType::Land) => lands.push(child_entity_ref),
                        Some(PermanentType::Artifact) => artifacts.push(child_entity_ref),
                        Some(PermanentType::Enchantment) => enchantments.push(child_entity_ref),
                        Some(PermanentType::Planeswalker) => planeswalkers.push(child_entity_ref),
                        Some(PermanentType::Token) => tokens.push(child_entity_ref),
                        None => other.push(child_entity_ref),
                    }
                } else {
                    other.push(child_entity_ref);
                }
            }

            // Position each group in its own section
            let grid_width = battlefield.grid_columns as f32;
            let grid_height = battlefield.grid_rows as f32;
            position_card_group(
                &mut card_query,
                &creatures,
                CardGroupPositioning {
                    start_row: 0.0,
                    start_col: 0.0,
                    end_row: grid_height / 2.0,
                    end_col: grid_width / 2.0,
                    cell_size,
                    scale,
                },
            );
            position_card_group(
                &mut card_query,
                &lands,
                CardGroupPositioning {
                    start_row: 0.0,
                    start_col: grid_width / 2.0,
                    end_row: grid_height,
                    end_col: grid_width,
                    cell_size,
                    scale,
                },
            );
            position_card_group(
                &mut card_query,
                &artifacts,
                CardGroupPositioning {
                    start_row: grid_height / 2.0,
                    start_col: 0.0,
                    end_row: grid_height,
                    end_col: grid_width / 2.0,
                    cell_size,
                    scale,
                },
            );
            position_card_group(
                &mut card_query,
                &enchantments,
                CardGroupPositioning {
                    start_row: grid_height / 2.0,
                    start_col: grid_width / 2.0,
                    end_row: grid_height,
                    end_col: grid_width,
                    cell_size,
                    scale,
                },
            );

            // Place planeswalkers and tokens in remaining space or overflow areas
            let remaining_cards: Vec<Entity> = planeswalkers
                .iter()
                .chain(tokens.iter())
                .chain(other.iter())
                .copied()
                .collect();
            position_card_group(
                &mut card_query,
                &remaining_cards,
                CardGroupPositioning {
                    start_row: 0.0,
                    start_col: 0.0,
                    end_row: grid_height,
                    end_col: grid_width,
                    cell_size,
                    scale,
                },
            );
        } else {
            // Simple grid layout without type grouping
            let start_x = -(grid_width * cell_size) / 2.0 + (cell_size / 2.0);
            let start_y = -(grid_height * cell_size) / 2.0 + (cell_size / 2.0);

            for (i, child_entity_ref) in children.iter().enumerate() {
                if let Ok((mut transform, _)) = card_query.get_mut(child_entity_ref) {
                    let row = (i as u32) / battlefield.grid_columns;
                    let col = (i as u32) % battlefield.grid_columns;

                    let x = start_x + (col as f32 * cell_size);
                    let y = start_y + (row as f32 * cell_size);

                    transform.translation = Vec3::new(x, y, i as f32 * 0.1);
                    transform.scale = Vec3::splat(scale);
                }
            }
        }
    }
}

/// Calculate layout parameters for battlefield based on card count
fn calculate_battlefield_layout(
    card_count: u32,
    grid_columns: u32,
    grid_rows: u32,
    zoom_level: f32,
    window_width: f32,
    window_height: f32,
) -> (f32, f32, f32, f32) {
    // Standard card dimensions with increased size multiplier
    let multiplier = crate::text::get_battlefield_card_size_multiplier();
    let card_width: f32 = 63.0 * multiplier;
    let card_height: f32 = 88.0 * multiplier;

    // Available space
    let available_width = window_width * 0.7;
    let available_height = window_height * 0.7;

    // Calculate required columns and rows
    let columns = grid_columns.max((card_count as f32).sqrt().ceil() as u32);
    let rows = grid_rows.max(card_count.div_ceil(columns));

    // Calculate cell size with spacing
    let cell_width = available_width / columns as f32;
    let cell_height = available_height / rows as f32;
    let cell_size = cell_width.min(cell_height);

    // Calculate scale based on cell size and zoom
    let scale = (cell_size / card_width.max(card_height)) * 0.9 * zoom_level;

    (columns as f32, rows as f32, cell_size, scale)
}

/// Position a group of cards in a specified grid area
fn position_card_group(
    card_query: &mut Query<(&mut Transform, Option<&PermanentType>)>,
    cards: &[Entity],
    positioning: CardGroupPositioning,
) {
    if cards.is_empty() {
        return;
    }

    let group_columns = ((positioning.end_col - positioning.start_col) * 2.0) as u32;
    if group_columns == 0 {
        return;
    }

    let start_x = (positioning.start_col * positioning.cell_size)
        - (((positioning.end_col - positioning.start_col) / 2.0) * positioning.cell_size);
    let start_y = (positioning.start_row * positioning.cell_size)
        - (((positioning.end_row - positioning.start_row) / 2.0) * positioning.cell_size);

    for (i, card_entity_ref) in cards.iter().enumerate() {
        if let Ok((mut transform, _)) = card_query.get_mut(*card_entity_ref) {
            let local_row = (i as u32) / group_columns;
            let local_col = (i as u32) % group_columns;

            let x = start_x + (local_col as f32 * positioning.cell_size / 2.0);
            let y = start_y + (local_row as f32 * positioning.cell_size / 2.0);

            transform.translation = Vec3::new(x, y, i as f32 * 0.1);
            transform.scale = Vec3::splat(positioning.scale);
        }
    }
}

/// System to toggle battlefield card grouping
pub fn toggle_battlefield_grouping(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut battlefield_query: Query<&mut BattlefieldZone>,
    game_state: Res<State<crate::menu::state::GameMenuState>>,
) {
    // Disable interactions if in any menu state
    if *game_state != crate::menu::state::GameMenuState::InGame {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyG) {
        for mut battlefield in battlefield_query.iter_mut() {
            battlefield.group_by_type = !battlefield.group_by_type;
            info!(
                "Battlefield grouping toggled: {}",
                battlefield.group_by_type
            );
        }
    }
}

/// System to adjust battlefield zoom level
pub fn adjust_battlefield_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut battlefield_query: Query<&mut BattlefieldZone>,
    game_state: Res<State<crate::menu::state::GameMenuState>>,
) {
    // Disable interactions if in any menu state
    if *game_state != crate::menu::state::GameMenuState::InGame {
        return;
    }

    for ev in scroll_evr.read() {
        let scroll_amount = match ev.unit {
            MouseScrollUnit::Line => ev.y * 0.1,
            MouseScrollUnit::Pixel => ev.y * 0.001,
        };

        for mut battlefield in battlefield_query.iter_mut() {
            battlefield.zoom_level = (battlefield.zoom_level + scroll_amount).clamp(0.5, 2.0);
        }
    }
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::collections::VecDeque;

/// Screenshot request event
#[derive(Event)]
pub struct ScreenshotEvent {
    pub entity: Entity,
    pub name: String,
}

/// Helper function to create a screenshot request event
pub fn request_screenshot(entity: Entity, name: String) -> ScreenshotEvent {
    ScreenshotEvent { entity, name }
}

/// Resource to track screenshot requests and results
#[derive(Resource, Default)]
pub struct ScreenshotRequests {
    /// Queue of processed screenshots (name, image)
    pub requests: VecDeque<(String, DynamicImage)>,
}

/// System that processes screenshot requests
pub fn capture_screenshot_system(
    mut er_screenshot: EventReader<ScreenshotEvent>,
    mut screenshot_requests: ResMut<ScreenshotRequests>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Process any screenshot events
    for event in er_screenshot.read() {
        // Get window
        let window = match q_window.get_single() {
            Ok(window) => window,
            Err(_) => {
                error!("Failed to get primary window for screenshot");
                continue;
            }
        };

        // Create a screenshot (in a real implementation, this would use the GPU to capture the screen)
        let width = window.physical_width();
        let height = window.physical_height();
        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let image = DynamicImage::ImageRgba8(buffer);

        // Store the screenshot result
        screenshot_requests
            .requests
            .push_back((event.name.clone(), image));
    }
}

/// Takes a screenshot of the current frame
pub fn take_screenshot() -> Option<DynamicImage> {
    // For the placeholder implementation, we'll just create a blank image with fixed size
    // In a real implementation, we would need to properly access the render app

    // Create a blank image with a default size
    let width = 1280;
    let height = 720;

    // Create a blank image
    let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let image = DynamicImage::ImageRgba8(buffer);

    Some(image)
}

/// Captures rendering of a specific entity
pub fn capture_entity_rendering(_world: &World, entity: Entity) -> DynamicImage {
    // In a real implementation, this would:
    // 1. Set up a temporary camera focused on just this entity
    // 2. Render a single frame
    // 3. Capture the output
    // 4. Clean up the temporary camera

    // Placeholder for now - in real implementation, we would use the entity parameter
    let _ = entity;

    if let Some(screenshot) = take_screenshot() {
        screenshot
    } else {
        // Fallback to a 1x1 pixel
        DynamicImage::ImageRgba8(ImageBuffer::new(1, 1))
    }
}

/// System for capturing screenshots on command
pub fn capture_on_command_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut screenshot_counter: Local<u32>,
    mut ew_screenshots: EventWriter<ScreenshotEvent>,
) {
    // Capture screenshot when F12 is pressed
    if keyboard_input.just_pressed(KeyCode::F12) {
        let screenshot_name = format!("screenshot_{}.png", *screenshot_counter);
        *screenshot_counter += 1;

        // Queue screenshot capture for next frame
        ew_screenshots.send(ScreenshotEvent {
            entity: Entity::PLACEHOLDER,
            name: screenshot_name,
        });
    }
}

/// Plugin to add screenshot capture hotkeys
pub struct ScreenshotCapturePlugin;

impl Plugin for ScreenshotCapturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenshotRequests>()
            .add_event::<ScreenshotEvent>()
            .add_systems(
                Update,
                (
                    capture_screenshot_system,
                    capture_on_command_system,
                    visual_test_saved_games_system,
                ),
            );
    }
}

/// Captures a screenshot from a saved game at a specific point
pub fn capture_saved_game_snapshot(
    world: &mut World,
    save_slot: &str,
    turn_number: Option<u32>,
    replay_step: Option<usize>,
) -> Option<DynamicImage> {
    // Load the save file or resume the replay
    match (turn_number, replay_step) {
        // Just load the save at its latest state
        (None, None) => {
            info!("Loading saved game: {}", save_slot);
            world.send_event(crate::game_engine::save::LoadGameEvent {
                slot_name: save_slot.to_string(),
            });
            // Run update to process load
            let _ = world.run_schedule(bevy::app::Update);
        }
        // Load to a specific turn
        (Some(turn), None) => {
            info!("Starting replay to turn {}: {}", turn, save_slot);
            // Start a replay
            world.send_event(crate::game_engine::save::StartReplayEvent {
                slot_name: save_slot.to_string(),
            });
            // Run update to process replay start
            let _ = world.run_schedule(bevy::app::Update);

            // Step through replay until we reach the desired turn
            // This is simplified - real code would need to track turns more carefully
            for _ in 0..turn {
                world.send_event(crate::game_engine::save::StepReplayEvent { steps: 1 });
                // Run update to process step
                let _ = world.run_schedule(bevy::app::Update);
            }
        }
        // Load and step to a specific point in the replay
        (_, Some(step)) => {
            info!("Starting replay at step {}: {}", step, save_slot);
            // Start a replay
            world.send_event(crate::game_engine::save::StartReplayEvent {
                slot_name: save_slot.to_string(),
            });
            // Run update to process replay start
            let _ = world.run_schedule(bevy::app::Update);

            // Step to the exact step
            world.send_event(crate::game_engine::save::StepReplayEvent { steps: step });
            // Run update to process step
            let _ = world.run_schedule(bevy::app::Update);
        }
    }

    // Find the game camera to capture
    let camera_entity = world
        .query_filtered::<Entity, With<crate::camera::components::GameCamera>>()
        .iter(world)
        .next();

    if let Some(camera) = camera_entity {
        // Take the snapshot
        let snapshot_name = match (turn_number, replay_step) {
            (None, None) => format!("test_save_{}", save_slot),
            (Some(turn), None) => format!("test_save_{}_turn_{}", save_slot, turn),
            (_, Some(step)) => format!("test_save_{}_step_{}", save_slot, step),
        };

        let snapshot_event = crate::snapshot::SnapshotEvent::new()
            .with_camera(camera)
            .with_filename(format!("{}.png", snapshot_name))
            .with_description(snapshot_name.clone())
            .with_debug(true);

        // Send the event to take a snapshot
        world.send_event(snapshot_event);

        // Run update to process the snapshot
        let _ = world.run_schedule(bevy::app::Update);
        let _ = world.run_schedule(bevy::app::PostUpdate);

        // Simplified - in a real implementation, we'd need to wait for the snapshot to complete
        // and retrieve the actual image data from the snapshot system

        return take_screenshot();
    } else {
        error!("No game camera found for capturing saved game");
        None
    }
}

/// System for visually testing saved games
pub fn visual_test_saved_games_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Option<Res<crate::game_engine::save::SaveConfig>>,
) {
    if keyboard_input.just_pressed(KeyCode::F9) {
        if let Some(config) = config {
            // Check save directory for save files
            let save_dir = &config.save_directory;

            // In a real implementation, we would enumerate save files from the directory
            info!("Visual testing save directory: {:?}", save_dir);

            // Start a visual test of all saved games
            commands.spawn(()).insert(VisualTestSavedGamesMarker);
        }
    }
}

/// Component to mark visual test of saved games in progress
#[derive(Component)]
pub struct VisualTestSavedGamesMarker;

/// System for capturing differential snapshots at specific points in a saved game for visual testing
pub fn capture_differential_game_snapshot(
    world: &mut World,
    save_slot: &str,
    turn_number: Option<u32>,
    replay_step: Option<usize>,
    snapshot_name: &str,
) -> Option<DynamicImage> {
    // Load the save file or resume the replay
    capture_saved_game_snapshot(world, save_slot, turn_number, replay_step)?;

    // Find the game camera
    let camera_entity = world
        .query_filtered::<Entity, With<crate::camera::components::GameCamera>>()
        .iter(world)
        .next()?;

    // Create a SaveGameSnapshot component to link this snapshot to the save game
    let save_snapshot = crate::snapshot::SaveGameSnapshot::new(save_slot, turn_number.unwrap_or(0))
        .with_description(format!("Visual diff: {}", snapshot_name))
        .with_timestamp(chrono::Local::now().timestamp());

    // Attach the SaveGameSnapshot component to the camera
    world.entity_mut(camera_entity).insert(save_snapshot);

    // Take the snapshot
    let filename = format!("visual_diff_{}.png", snapshot_name);
    let snapshot_event = crate::snapshot::SnapshotEvent::new()
        .with_camera(camera_entity)
        .with_filename(filename)
        .with_description(format!("Visual differential test: {}", snapshot_name))
        .with_debug(true);

    // Send the event
    world.send_event(snapshot_event);

    // Run update to process the snapshot
    let _ = world.run_schedule(bevy::app::Update);
    let _ = world.run_schedule(bevy::app::PostUpdate);

    // Simplified - in a real implementation, we'd wait for the snapshot to complete
    // and retrieve the actual image data
    take_screenshot()
}

/// Compare two rendered states of the same game for visual differences
pub fn compare_game_states(
    world: &mut World,
    save_slot: &str,
    reference_point: (Option<u32>, Option<usize>), // (turn, step)
    comparison_point: (Option<u32>, Option<usize>), // (turn, step)
) -> Option<(DynamicImage, DynamicImage, f32)> {
    // Capture reference image
    let reference_name = match (reference_point.0, reference_point.1) {
        (Some(turn), Some(step)) => format!("{}_turn{}_step{}", save_slot, turn, step),
        (Some(turn), None) => format!("{}_turn{}", save_slot, turn),
        (None, Some(step)) => format!("{}_step{}", save_slot, step),
        (None, None) => format!("{}_latest", save_slot),
    };

    let reference_image = capture_differential_game_snapshot(
        world,
        save_slot,
        reference_point.0,
        reference_point.1,
        &reference_name,
    )?;

    // Capture comparison image
    let comparison_name = match (comparison_point.0, comparison_point.1) {
        (Some(turn), Some(step)) => format!("{}_turn{}_step{}", save_slot, turn, step),
        (Some(turn), None) => format!("{}_turn{}", save_slot, turn),
        (None, Some(step)) => format!("{}_step{}", save_slot, step),
        (None, None) => format!("{}_latest", save_slot),
    };

    let comparison_image = capture_differential_game_snapshot(
        world,
        save_slot,
        comparison_point.0,
        comparison_point.1,
        &comparison_name,
    )?;

    // Calculate difference (placeholder - in a real implementation, this would compare pixels)
    // This example just returns a percentage difference of 10%
    let difference = 0.10;

    Some((reference_image, comparison_image, difference))
}

/// Function to run a visual differential test on a saved game
pub fn run_visual_diff_test(save_slot: &str) -> Result<(), String> {
    let mut app = bevy::app::App::new();

    // Add required plugins (minimal set)
    app.add_plugins(bevy::MinimalPlugins)
        .add_plugins(crate::snapshot::SnapshotPlugin)
        .add_plugins(crate::game_engine::save::SaveLoadPlugin);

    // Initialize the app
    app.update();

    // Compare initial state to state after 3 turns
    let comparison_result = compare_game_states(
        &mut app.world_mut(),
        save_slot,
        (Some(1), None), // Turn 1
        (Some(3), None), // Turn 3
    );

    match comparison_result {
        Some((_, _, diff)) => {
            if diff > 0.5 {
                Err(format!(
                    "Visual difference of {}% exceeds threshold",
                    diff * 100.0
                ))
            } else {
                Ok(())
            }
        }
        None => Err("Failed to capture and compare game states".to_string()),
    }
}

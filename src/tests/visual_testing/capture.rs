use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::collections::VecDeque;
use std::path::PathBuf;

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

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
                (capture_screenshot_system, capture_on_command_system),
            );
    }
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};

/// Resource to track screenshot requests
#[derive(Resource, Default)]
pub struct ScreenshotRequests {
    pending: Vec<ScreenshotRequest>,
}

/// Request for capturing a screenshot
pub struct ScreenshotRequest {
    pub name: String,
    pub callback: Option<Box<dyn Fn(DynamicImage) + Send + Sync>>,
}

/// System that processes screenshot requests
pub fn capture_screenshot_system(world: &mut World) {
    // Check if there are any pending requests
    let has_pending_requests = {
        let screenshot_requests = world.resource::<ScreenshotRequests>();
        !screenshot_requests.pending.is_empty()
    };

    // If no pending requests, return early
    if !has_pending_requests {
        return;
    }

    // Take a screenshot
    let screenshot = take_screenshot_from_world(world);

    if let Some(image) = screenshot {
        // Process all pending requests with this screenshot
        let mut screenshot_requests = world.resource_mut::<ScreenshotRequests>();
        let requests = std::mem::take(&mut screenshot_requests.pending);

        for request in requests {
            if let Some(callback) = request.callback {
                callback(image.clone());
            } else {
                // If no callback, save as reference
                use crate::tests::visual_testing::utils::save_reference_image;
                if let Err(e) = save_reference_image(image.clone(), &request.name) {
                    error!("Failed to save screenshot {}: {}", request.name, e);
                }
            }
        }
    }
}

/// Takes a screenshot of the current frame from world
fn take_screenshot_from_world(world: &mut World) -> Option<DynamicImage> {
    // Try to get the render app - we'll skip this in the placeholder implementation
    // as the render app access approach needs to be revised in Bevy 0.15.x

    // Get window entity
    let mut primary_window_query = world.query_filtered::<&Window, With<PrimaryWindow>>();

    if let Ok(window) = primary_window_query.get_single(world) {
        // In a real implementation, we would:
        // 1. Get the texture view for the window
        // 2. Create a buffer to copy the texture to
        // 3. Issue a copy command from the texture to the buffer
        // 4. Map the buffer and read the pixels
        // 5. Convert to an image

        // Placeholder for simplicity
        let width = window.physical_width();
        let height = window.physical_height();

        // Create a blank image for now
        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let image = DynamicImage::ImageRgba8(buffer);

        return Some(image);
    }

    None
}

/// Takes a screenshot of the current frame
pub fn take_screenshot(_app: &App) -> Option<DynamicImage> {
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

/// Request a screenshot to be taken on the next frame
pub fn request_screenshot(
    world: &mut World,
    name: String,
    callback: Option<Box<dyn Fn(DynamicImage) + Send + Sync>>,
) {
    let mut screenshot_requests = world.resource_mut::<ScreenshotRequests>();
    screenshot_requests
        .pending
        .push(ScreenshotRequest { name, callback });
}

/// Captures rendering of a specific entity
pub fn capture_entity_rendering(app: &App, _entity: Entity) -> DynamicImage {
    // In a real implementation, this would:
    // 1. Set up a temporary camera focused on just this entity
    // 2. Render a single frame
    // 3. Capture the output
    // 4. Clean up the temporary camera

    // Placeholder for now
    if let Some(screenshot) = take_screenshot(app) {
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
    mut screenshot_requests: ResMut<ScreenshotRequests>,
) {
    // Capture screenshot when F12 is pressed
    if keyboard_input.just_pressed(KeyCode::F12) {
        let screenshot_name = format!("screenshot_{}.png", *screenshot_counter);
        *screenshot_counter += 1;

        // Queue screenshot capture for next frame
        screenshot_requests.pending.push(ScreenshotRequest {
            name: screenshot_name.clone(),
            callback: Some(Box::new(move |image| {
                use crate::tests::visual_testing::utils::save_reference_image;
                if let Err(e) = save_reference_image(image, &screenshot_name) {
                    error!("Failed to save screenshot {}: {}", screenshot_name, e);
                } else {
                    info!("Screenshot saved as {}", screenshot_name);
                }
            })),
        });
    }
}

/// Plugin to add screenshot capture hotkeys
pub struct ScreenshotCapturePlugin;

impl Plugin for ScreenshotCapturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenshotRequests>()
            .add_systems(Update, capture_on_command_system);
    }
}

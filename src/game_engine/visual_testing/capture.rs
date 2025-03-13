use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::RenderDevice;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::sync::Arc;

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
    let mut screenshot_requests = world.resource_mut::<ScreenshotRequests>();
    if screenshot_requests.pending.is_empty() {
        return;
    }

    let app = world.as_app();
    if let Some(image) = take_screenshot(app) {
        // Process all pending requests with this screenshot
        let requests = std::mem::take(&mut screenshot_requests.pending);
        for request in requests {
            if let Some(callback) = request.callback {
                callback(image.clone());
            } else {
                // If no callback, save as reference
                use crate::game_engine::visual_testing::utils::save_reference_image;
                if let Err(e) = save_reference_image(image.clone(), &request.name) {
                    error!("Failed to save screenshot {}: {}", request.name, e);
                }
            }
        }
    }
}

/// Takes a screenshot of the current frame
pub fn take_screenshot(app: &App) -> Option<DynamicImage> {
    // Get access to render resources
    if let Ok(render_app) = app.get_sub_app(RenderApp) {
        let render_device = render_app.world.resource::<RenderDevice>();

        // Get the current window
        if let Some(window) = app.world().get_resource::<bevy::window::PrimaryWindow>() {
            // In a real implementation, we would:
            // 1. Get the texture view for the window
            // 2. Create a buffer to copy the texture to
            // 3. Issue a copy command from the texture to the buffer
            // 4. Map the buffer and read the pixels
            // 5. Convert to an image

            // Placeholder for simplicity - in a real implementation, this would
            // actually extract from the GPU render target
            let width = window.width() as u32;
            let height = window.height() as u32;

            // Create a blank image for now
            let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
            let image = DynamicImage::ImageRgba8(buffer);

            return Some(image);
        }
    }

    None
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
pub fn capture_entity_rendering(app: &App, entity: Entity) -> DynamicImage {
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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut screenshot_counter: Local<u32>,
) {
    // Capture screenshot when F12 is pressed
    if keyboard_input.just_pressed(KeyCode::F12) {
        let screenshot_name = format!("screenshot_{}.png", *screenshot_counter);
        *screenshot_counter += 1;

        // Queue screenshot capture for next frame
        request_screenshot(
            commands.world_mut(),
            screenshot_name.clone(),
            Some(Box::new(move |image| {
                use crate::game_engine::visual_testing::utils::save_reference_image;
                if let Err(e) = save_reference_image(image, &screenshot_name) {
                    error!("Failed to save screenshot {}: {}", screenshot_name, e);
                } else {
                    info!("Screenshot saved as {}", screenshot_name);
                }
            })),
        );
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

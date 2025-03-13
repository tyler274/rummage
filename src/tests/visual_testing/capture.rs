use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, ImageBuffer, Rgba};

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
pub fn capture_screenshot_system(
    mut screenshot_requests: ResMut<ScreenshotRequests>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Check if there are any pending requests
    if screenshot_requests.pending.is_empty() {
        return;
    }

    // Get window
    let window = match q_window.get_single() {
        Ok(window) => window,
        Err(_) => {
            error!("Failed to get primary window for screenshot");
            return;
        }
    };

    // Create a placeholder screenshot
    let width = window.physical_width();
    let height = window.physical_height();
    let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let image = DynamicImage::ImageRgba8(buffer);

    // Process all pending requests with this screenshot
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

/// Takes a screenshot of the current frame from world
fn take_screenshot_from_window(window: &Window) -> Option<DynamicImage> {
    // Create a blank image with the window size
    let width = window.physical_width();
    let height = window.physical_height();

    // Create a blank image
    let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let image = DynamicImage::ImageRgba8(buffer);

    Some(image)
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
pub fn request_screenshot<'a, T: AsMut<ScreenshotRequests>>(
    screenshot_requests: &mut T,
    name: String,
    callback: Option<Box<dyn Fn(DynamicImage) + Send + Sync>>,
) {
    screenshot_requests
        .as_mut()
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

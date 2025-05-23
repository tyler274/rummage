use bevy::prelude::*;

use crate::menu::{camera::setup::MenuCamera, state::MenuState};

/// Manages the visibility of the menu camera based on game states
pub fn manage_camera_visibility(
    mut menu_cameras: Query<&mut Visibility, With<MenuCamera>>,
    state: Res<State<MenuState>>,
) {
    // Determine if the camera should be visible based on state
    let should_be_visible = matches!(
        *state.get(),
        MenuState::MainMenu | MenuState::PauseMenu | MenuState::Settings | MenuState::Credits
    );

    // Update camera visibility
    for mut visibility in menu_cameras.iter_mut() {
        let new_visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if *visibility != new_visibility {
            info!(
                "Setting menu camera visibility to {:?} in state {:?}",
                new_visibility,
                state.get()
            );
            *visibility = new_visibility;
        }
    }
}

/// Sets the zoom level for the menu camera
pub fn set_menu_camera_zoom(mut cameras: Query<&mut Projection, With<MenuCamera>>) {
    for mut projection_enum in cameras.iter_mut() {
        if let Projection::Orthographic(ref mut ortho_projection) = *projection_enum {
            ortho_projection.scale = 1.0;
            info!("Set menu camera zoom to 1.0");
        }
    }
}

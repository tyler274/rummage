use crate::camera::components::AppLayer;
use crate::menu::components::{MenuItem, ZLayers};
use bevy::prelude::*;

use super::components::StarOfDavid;

/// Creates the Star of David bundle for spawning in the scene
/// This is used by the logo module to create the Star of David component
pub fn create_star_of_david() -> impl Bundle {
    (
        // Mark this entity with the StarOfDavid component
        StarOfDavid,
        // Add a default size Node for rendering
        Node {
            width: Val::Px(150.0),
            height: Val::Px(150.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // Make the background transparent
        BackgroundColor(Color::NONE),
        // Add standard UI components
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        // Add menu layer and components
        AppLayer::Menu.layer(),
        MenuItem,
        ZIndex::from(ZLayers::MenuButtonText),
    )
}

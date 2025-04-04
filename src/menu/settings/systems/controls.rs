use super::common::{
    TEXT_COLOR, create_toggle_setting, spawn_settings_button, spawn_settings_container,
    spawn_settings_root, spawn_settings_title,
};
use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use bevy::prelude::*;

/// Sets up the controls settings menu
pub fn setup_controls_settings(mut commands: Commands) {
    info!("Setting up controls settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Controls Settings",
    );

    // Store root_entity for later use
    let mut root = commands.entity(root_entity);

    // Create a new scope for the first with_children call
    root.with_children(|parent| {
        spawn_settings_title(parent, "Controls Settings");

        let _container = spawn_settings_container(parent);

        // Add controls settings here
        create_toggle_setting(parent, "Invert Mouse Y", false);
        create_toggle_setting(parent, "Mouse Acceleration", true);

        // Back button
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });
}

/// Creates a keybinding display
fn create_keybinding(parent: &mut ChildBuilder, action: &str, key: &str) {
    parent
        .spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            MenuItem,
            SettingsMenuItem,
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new(format!("Keybinding {}", action)),
        ))
        .with_children(|parent| {
            // Action label
            parent.spawn((
                Text::new(action),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new(format!("Keybinding Action {}", action)),
            ));

            // Key label
            parent.spawn((
                Text::new(key),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new(format!("Keybinding Key {}", key)),
            ));
        });
}

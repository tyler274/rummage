use super::common::*;
use crate::menu::settings::components::*;
use bevy::prelude::*;

/// Sets up the controls settings screen
pub fn setup_controls_settings(mut commands: Commands) {
    info!("Setting up controls settings screen");

    // Create root node with purple tint for controls settings
    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.3, 0.2, 0.3, 0.95),
        "Controls Settings",
    );

    commands.entity(root_entity).insert(ControlsSettingsScreen);

    commands.entity(root_entity).with_children(|parent| {
        // Title
        spawn_settings_title(parent, "CONTROLS SETTINGS");

        // Settings container
        let container = spawn_settings_container(parent);

        commands.entity(container).with_children(|parent| {
            // Create keybinding displays
            create_keybinding(parent, "Pause Game:", "ESC");
            create_keybinding(parent, "Select Card:", "Left Click");
            create_keybinding(parent, "Card Info:", "Right Click");
            create_keybinding(parent, "Zoom Camera:", "Mouse Wheel");

            // Back button
            spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
        });
    });

    info!(
        "Controls settings screen setup complete - root entity: {:?}",
        root_entity
    );
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

use super::common::*;
use crate::menu::settings::components::*;
use bevy::prelude::*;

/// Sets up the gameplay settings screen
pub fn setup_gameplay_settings(
    mut commands: Commands,
    gameplay_settings: Option<Res<GameplaySettings>>,
) {
    let settings = gameplay_settings.map(|s| s.clone()).unwrap_or_default();

    info!("Setting up gameplay settings screen");

    // Create root node with green tint for gameplay settings
    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.2, 0.3, 0.1, 0.95),
        "Gameplay Settings",
    );

    commands.entity(root_entity).insert(GameplaySettingsScreen);

    commands.entity(root_entity).with_children(|parent| {
        // Title
        spawn_settings_title(parent, "GAMEPLAY SETTINGS");

        // Settings container
        let container = spawn_settings_container(parent);

        commands.entity(container).with_children(|parent| {
            // Auto-pass setting
            create_toggle_setting(parent, "Auto-Pass Priority:", settings.auto_pass);

            // Show tooltips setting
            create_toggle_setting(parent, "Show Card Tooltips:", settings.show_tooltips);

            // Animation speed setting
            create_animation_speed_setting(parent, settings.animation_speed);

            // Back button
            spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
        });
    });

    info!(
        "Gameplay settings screen setup complete - root entity: {:?}",
        root_entity
    );
}

/// Creates an animation speed setting display
fn create_animation_speed_setting(parent: &mut ChildBuilder, speed: f32) {
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
            Name::new("Animation Speed Row"),
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new("Animation Speed:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Animation Speed Label"),
            ));

            // Value
            parent.spawn((
                Text::new(format!("{:.1}x", speed)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Animation Speed Value"),
            ));
        });
}

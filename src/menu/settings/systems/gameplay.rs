use super::common::{
    TEXT_COLOR, create_toggle_setting, spawn_settings_button, spawn_settings_container,
    spawn_settings_root, spawn_settings_title,
};
use crate::menu::components::*;
use crate::menu::settings::components::OnGameplaySettingsMenu;
use crate::menu::settings::components::*;
use bevy::prelude::*;

/// Sets up the gameplay settings UI elements
pub fn setup_gameplay_settings(mut commands: Commands, settings: Res<GameplaySettings>) {
    info!("Setting up gameplay settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Gameplay Settings",
    );

    // Add the marker component to the root entity
    commands.entity(root_entity).insert(OnGameplaySettingsMenu);

    // Get the container entity before the closure
    let mut container_entity = Entity::PLACEHOLDER;
    let mut root = commands.entity(root_entity);

    root.with_children(|parent| {
        spawn_settings_title(parent, "Gameplay Settings");
        container_entity = spawn_settings_container(parent);
        // Back button outside the container's closure
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });

    // Build content for the container separately to avoid double borrow of commands
    let mut container_children = commands.entity(container_entity);
    container_children.with_children(|parent| {
        create_toggle_setting(parent, "Auto Pass", settings.auto_pass);
        create_toggle_setting(parent, "Show Tooltips", settings.show_tooltips);
        // create_slider_setting(parent, "Animation Speed", settings.animation_speed);
    });

    // Back button to return to main settings
    commands.entity(container_entity).with_children(|parent| {
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });
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

use super::common::*;
use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use crate::menu::settings::state::SettingsMenuState;
use crate::menu::styles::*;
use bevy::prelude::*;

/// Sets up the gameplay settings menu
pub fn setup_gameplay_settings(mut commands: Commands) {
    info!("Setting up gameplay settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Gameplay Settings",
    );

    // Store root_entity for later use
    let mut root = commands.entity(root_entity);

    // Create a new scope for the first with_children call
    root.with_children(|parent| {
        spawn_settings_title(parent, "Gameplay Settings");

        let _container = spawn_settings_container(parent);

        // Add gameplay settings here
        create_toggle_setting(parent, "Show Card Tooltips", true);
        create_toggle_setting(parent, "Auto-Pass Priority", false);
        create_toggle_setting(parent, "Stack Auto-Ordering", true);

        // Back button
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

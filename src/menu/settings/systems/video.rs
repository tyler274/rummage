use super::common::*;
use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use crate::menu::settings::state::SettingsMenuState;
use crate::menu::styles::*;
use bevy::prelude::*;

/// Sets up the video settings menu
pub fn setup_video_settings(mut commands: Commands) {
    info!("Setting up video settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Video Settings",
    );

    // Store root_entity for later use
    let mut root = commands.entity(root_entity);

    // Create a new scope for the first with_children call
    root.with_children(|parent| {
        spawn_settings_title(parent, "Video Settings");

        let _container = spawn_settings_container(parent);

        // Add video settings here
        create_toggle_setting(parent, "Fullscreen", true);
        create_toggle_setting(parent, "VSync", true);
        create_toggle_setting(parent, "Motion Blur", false);

        // Back button
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });
}

/// Creates a graphics quality setting display
fn create_graphics_quality_setting(parent: &mut ChildBuilder, quality: &GraphicsQuality) {
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
            Name::new("Graphics Quality Row"),
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new("Graphics Quality:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Graphics Quality Label"),
            ));

            // Value
            let quality_text = match quality {
                GraphicsQuality::Low => "Low",
                GraphicsQuality::Medium => "Medium",
                GraphicsQuality::High => "High",
            };

            parent.spawn((
                Text::new(quality_text),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Graphics Quality Value"),
            ));
        });
}

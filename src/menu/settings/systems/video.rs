use super::common::*;
use crate::menu::settings::components::*;
use bevy::prelude::*;

/// Sets up the video settings screen
pub fn setup_video_settings(
    mut commands: Commands,
    graphics_quality: Option<Res<GraphicsQuality>>,
) {
    let quality = graphics_quality.map(|q| q.clone()).unwrap_or_default();

    info!("Setting up video settings screen");

    // Create root node with blue tint for video settings
    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.1, 0.2, 0.3, 0.95),
        "Video Settings",
    );

    commands.entity(root_entity).insert(VideoSettingsScreen);

    commands.entity(root_entity).with_children(|parent| {
        // Title
        spawn_settings_title(parent, "VIDEO SETTINGS");

        // Settings container
        let container = spawn_settings_container(parent);

        commands.entity(container).with_children(|parent| {
            // Quality setting
            create_graphics_quality_setting(parent, &quality);

            // Back button
            spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
        });
    });

    info!(
        "Video settings screen setup complete - root entity: {:?}",
        root_entity
    );
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

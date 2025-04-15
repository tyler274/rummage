use super::common::{
    TEXT_COLOR, spawn_settings_button, spawn_settings_container, spawn_settings_root,
    spawn_settings_title,
};
use crate::menu::components::*;
use crate::menu::settings::components::OnVideoSettingsMenu;
use crate::menu::settings::components::{
    GraphicsQuality, QualityButton, SettingsButtonAction, SettingsMenuItem,
};
use crate::menu::settings::plugin::CurrentGraphicsQuality;
use bevy::prelude::*;

/// Sets up the video settings UI elements
pub fn setup_video_settings(mut commands: Commands, graphics_quality: Res<CurrentGraphicsQuality>) {
    info!("Setting up video settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Video Settings",
    );

    // Add the marker component to the root entity
    commands.entity(root_entity).insert(OnVideoSettingsMenu);

    // Variable to hold the container entity ID, initialized inside the closure
    let mut container_entity_id = Entity::PLACEHOLDER;

    commands.entity(root_entity).with_children(|parent| {
        spawn_settings_title(parent, "Video Settings");

        // Spawn container using the parent builder and store its ID
        container_entity_id = spawn_settings_container(parent); // This returns an Entity

        // Spawn the back button
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });

    // Now that the first `with_children` scope is closed, we can borrow `commands` again
    // Add children to the container using its stored ID
    commands
        .entity(container_entity_id)
        .with_children(|container_parent| {
            // Add video settings here
            create_quality_setting(
                container_parent,
                "Graphics Quality",
                &graphics_quality.quality,
            );
        });
}

/// Creates a quality setting display with buttons
fn create_quality_setting(
    parent: &mut ChildBuilder,
    label: &str,
    current_quality: &GraphicsQuality,
) {
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
            Name::new("Quality Setting Row"),
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Quality Label"),
            ));

            // Quality buttons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_quality_button(parent, GraphicsQuality::Low, current_quality);
                    spawn_quality_button(parent, GraphicsQuality::Medium, current_quality);
                    spawn_quality_button(parent, GraphicsQuality::High, current_quality);
                });
        });
}

/// Spawns a quality button
fn spawn_quality_button(
    parent: &mut ChildBuilder,
    quality: GraphicsQuality,
    current_quality: &GraphicsQuality,
) {
    let quality_text = match quality {
        GraphicsQuality::Low => "Low",
        GraphicsQuality::Medium => "Medium",
        GraphicsQuality::High => "High",
    };

    let background_color = if quality == *current_quality {
        BackgroundColor(Color::srgba(0.4, 0.4, 0.8, 1.0)) // Highlighted
    } else {
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)) // Normal
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::horizontal(Val::Px(5.0)),
                ..default()
            },
            background_color,
            QualityButton(quality),
            MenuItem,
            SettingsMenuItem,
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new(format!("Quality Button {}", quality_text)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(quality_text),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// System to handle interactions with graphics quality buttons
pub fn quality_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &QualityButton, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut quality_setting: ResMut<CurrentGraphicsQuality>,
    mut button_query: Query<(Entity, &mut BackgroundColor, &QualityButton), With<Button>>,
) {
    for (interaction, clicked_quality_button, clicked_entity) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let new_quality = clicked_quality_button.0;

            // Only update if the quality actually changed
            if new_quality != quality_setting.quality {
                info!("Changing graphics quality to: {:?}", new_quality);
                quality_setting.quality = new_quality;

                // Update background colors for all quality buttons
                for (_entity, mut bg_color, button_quality) in button_query.iter_mut() {
                    if button_quality.0 == new_quality {
                        *bg_color = BackgroundColor(Color::srgba(0.4, 0.4, 0.8, 1.0)); // Highlighted
                    } else {
                        *bg_color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)); // Normal
                    }
                }
            }
        }
    }
}

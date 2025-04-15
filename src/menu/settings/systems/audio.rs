use super::common::*;
use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
use crate::menu::settings::components::OnAudioSettingsMenu;
use crate::menu::settings::components::*;
use bevy::audio::Volume;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

/// Query for volume slider interactions
type VolumeSliderInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static VolumeType,
        &'static Node,
        &'static GlobalTransform,
    ),
    (Changed<Interaction>, With<VolumeSlider>),
>;

/// Volume slider type to identify which volume is being adjusted
#[derive(Component, Clone, Copy, Debug)]
pub enum VolumeType {
    Master,
    Music,
    Sfx,
}

/// Component to mark volume value text for updating
#[derive(Component)]
pub struct VolumeValueText(pub VolumeType);

/// Volume slider component
#[derive(Component)]
pub struct VolumeSlider;

/// Volume settings context without queries to avoid conflicts
#[derive(SystemParam)]
pub struct VolumeSettingsContext<'w> {
    volume_settings: ResMut<'w, VolumeSettings>,
    global_volume: ResMut<'w, bevy::prelude::GlobalVolume>,
    persistent_settings: Option<ResMut<'w, Persistent<RummageSettings>>>,
}

/// Resource to store volume update requests
#[derive(Resource, Default)]
pub struct VolumeUpdateRequests {
    requests: Vec<(VolumeType, i32, f32)>,
}

/// Sets up the audio settings menu
pub fn setup_audio_settings(mut commands: Commands, volume_settings: Res<VolumeSettings>) {
    info!("Setting up audio settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Audio Settings",
    );

    // Add the marker component to the root entity
    commands.entity(root_entity).insert(OnAudioSettingsMenu);

    // Store root_entity for later use
    let mut root = commands.entity(root_entity);

    // Create a new scope for the first with_children call
    root.with_children(|parent| {
        spawn_settings_title(parent, "Audio Settings");

        let _container = spawn_settings_container(parent);

        // Volume slider
        parent
            .spawn((
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                MenuItem,
                SettingsMenuItem,
                AppLayer::Menu.layer(),
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Volume Slider Container"),
            ))
            .with_children(|parent| {
                // Label
                parent.spawn((
                    Text::new("Master Volume"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    MenuItem,
                    SettingsMenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    Name::new("Volume Label"),
                ));

                // Slider
                let initial_volume_percent = (volume_settings.master * 100.0).round() as u32;

                parent
                    .spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        MenuItem,
                        SettingsMenuItem,
                        AppLayer::Menu.layer(),
                        Visibility::Visible,
                        InheritedVisibility::VISIBLE,
                        Name::new("Volume Slider"),
                        VolumeSlider,
                        VolumeType::Master,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Node {
                                width: Val::Percent(initial_volume_percent as f32),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            MenuItem,
                            SettingsMenuItem,
                            AppLayer::Menu.layer(),
                            Visibility::Visible,
                            InheritedVisibility::VISIBLE,
                            Name::new("Volume Slider Fill"),
                        ));
                    });

                // Value text
                parent.spawn((
                    Text::new(format!("{}%", initial_volume_percent)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    MenuItem,
                    SettingsMenuItem,
                    AppLayer::Menu.layer(),
                    Visibility::Visible,
                    InheritedVisibility::VISIBLE,
                    VolumeValueText(VolumeType::Master),
                    Name::new("Volume Value"),
                ));
            });

        // Back button
        spawn_settings_button(parent, "Back", SettingsButtonAction::NavigateToMain);
    });
}

/// System to process slider interactions and queue volume updates
pub fn volume_slider_interaction(
    interaction_query: VolumeSliderInteractionQuery,
    mut volume_requests: ResMut<VolumeUpdateRequests>,
    windows: Query<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    // Only process if left mouse button is pressed
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    // Get the primary window and cursor position
    let window = windows.get_single().expect("Expected a primary window");

    if let Some(cursor_pos) = window.cursor_position() {
        // Process interaction data without modifying any other components
        for (interaction, volume_type, node, transform) in interaction_query.iter() {
            if let Interaction::Pressed = *interaction {
                // Calculate slider value from cursor position
                let button_size = match node.width {
                    Val::Px(px) => px,
                    Val::Percent(pct) => pct * 2.0, // Approximation, adjust if needed
                    _ => 200.0,                     // Fallback width
                };

                let button_pos = transform.translation().x;
                let relative_x = ((cursor_pos.x - (button_pos - button_size / 2.0)) / button_size)
                    .clamp(0.0, 1.0);
                let clamped_value = (relative_x * 100.0).round() as i32;
                let volume_value = clamped_value as f32 / 100.0;

                // Queue the update
                volume_requests
                    .requests
                    .push((*volume_type, clamped_value, volume_value));
            }
        }
    }
}

/// System to apply volume changes and update UI
pub fn apply_volume_updates(
    mut volume_requests: ResMut<VolumeUpdateRequests>,
    mut text_query: Query<(&mut Text, &VolumeValueText)>,
    mut volume_indicators: Query<(&mut Node, &Parent), Without<Button>>,
    volume_type_query: Query<&VolumeType>,
    mut audio_players: Query<&mut bevy::audio::PlaybackSettings>,
    mut context: VolumeSettingsContext,
) {
    if volume_requests.requests.is_empty() {
        return;
    }

    // Process all updates
    for (volume_type, clamped_value, volume_value) in volume_requests.requests.drain(..) {
        // Update volume settings
        match volume_type {
            VolumeType::Master => {
                context.volume_settings.master = volume_value;
                context.global_volume.volume = Volume::new(volume_value);
            }
            VolumeType::Music => {
                context.volume_settings.music = volume_value;
                for mut settings in audio_players.iter_mut() {
                    settings.volume = Volume::new(volume_value);
                }
            }
            VolumeType::Sfx => {
                context.volume_settings.sfx = volume_value;
                // NOTE: Need a way to apply SFX volume globally or per-sound
            }
        }

        // Update text display
        for (mut text, value_text) in text_query.iter_mut() {
            if value_text.0 as u8 == volume_type as u8 {
                text.0 = format!("{}%", clamped_value);
            }
        }

        // Update slider indicators
        for (mut node, parent) in volume_indicators.iter_mut() {
            if let Ok(parent_volume_type) = volume_type_query.get(parent.get()) {
                if *parent_volume_type as u8 == volume_type as u8 {
                    node.width = Val::Percent(clamped_value as f32);
                }
            }
        }

        // Save settings
        save_volume_settings(&mut context.persistent_settings, &context.volume_settings);

        info!("Volume {:?} set to {}%", volume_type, clamped_value);
    }
}

fn save_volume_settings(
    persistent_settings: &mut Option<ResMut<Persistent<RummageSettings>>>,
    volume_settings: &VolumeSettings,
) {
    if let Some(persistent) = persistent_settings {
        let mut settings = (*persistent).clone();
        settings.volume = volume_settings.clone();
        if let Err(e) = persistent.set(settings) {
            error!("Failed to save volume settings: {:?}", e);
        }
    }
}

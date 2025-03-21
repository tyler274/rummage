use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};
use bevy_persistent::prelude::*;

use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
use crate::menu::input_blocker::InputBlocker;
use crate::menu::state::{GameMenuState, StateTransitionContext};
use crate::menu::styles::*;

use super::components::*;
use super::state::SettingsMenuState;

/// Text color for settings menu
const TEXT_COLOR: Color = Color::WHITE;

/// Sets up the main settings menu
pub fn setup_main_settings(mut commands: Commands, context: Res<StateTransitionContext>) {
    info!(
        "Setting up main settings menu - START (from origin: {:?})",
        context.settings_origin
    );

    // Log the transition context to see if it's set correctly
    info!(
        "Settings transition context: from_pause_menu={}, settings_origin={:?}",
        context.from_pause_menu, context.settings_origin
    );

    // First, create a full-screen transparent input blocker
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        AppLayer::Menu.layer(),
        InputBlocker,
        MenuItem,
        SettingsMenuItem,
        Name::new("Settings Menu Input Blocker"),
    ));

    let root_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            MenuItem,
            SettingsMenuItem,
            MainSettingsScreen,
            AppLayer::Menu.layer(),
            Name::new("Settings Root Node"),
        ))
        .with_children(|parent| {
            // Title
            let title_entity = parent
                .spawn((
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextColor(TEXT_COLOR),
                    AppLayer::Menu.layer(),
                    Name::new("Settings Title"),
                ))
                .id();

            info!("Spawned settings title entity: {:?}", title_entity);

            // Settings container
            let container_entity = parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                    Name::new("Settings Container"),
                ))
                .with_children(|parent| {
                    spawn_settings_button(parent, "Video", SettingsButtonAction::VideoSettings);
                    spawn_settings_button(parent, "Audio", SettingsButtonAction::AudioSettings);
                    spawn_settings_button(
                        parent,
                        "Gameplay",
                        SettingsButtonAction::GameplaySettings,
                    );
                    spawn_settings_button(
                        parent,
                        "Controls",
                        SettingsButtonAction::ControlsSettings,
                    );
                    spawn_settings_button(parent, "Back", SettingsButtonAction::Back);
                })
                .id();

            info!("Spawned settings container entity: {:?}", container_entity);
        })
        .id();

    info!(
        "Setting up main settings menu - COMPLETE. Root entity: {:?}",
        root_entity
    );
}

/// Sets up the video settings screen
pub fn setup_video_settings(
    mut commands: Commands,
    graphics_quality: Option<Res<GraphicsQuality>>,
) {
    let quality = graphics_quality.map(|q| q.clone()).unwrap_or_default();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            MenuItem,
            SettingsMenuItem,
            VideoSettingsScreen,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("VIDEO SETTINGS"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));

            // Settings container
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Quality setting
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
                            AppLayer::Menu.layer(),
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
                                AppLayer::Menu.layer(),
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
                                AppLayer::Menu.layer(),
                            ));
                        });

                    // Back button
                    spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
                });
        });
}

/// Sets up the audio settings screen
pub fn setup_audio_settings(mut commands: Commands, volume_settings: Option<Res<VolumeSettings>>) {
    let volume = volume_settings.map(|v| v.clone()).unwrap_or_default();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            MenuItem,
            SettingsMenuItem,
            AudioSettingsScreen,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("AUDIO SETTINGS"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));

            // Settings container
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Master volume setting
                    create_volume_slider(
                        parent,
                        "Master Volume:",
                        (volume.master * 100.0) as i32,
                        VolumeType::Master,
                    );

                    // Music volume setting
                    create_volume_slider(
                        parent,
                        "Music Volume:",
                        (volume.music * 100.0) as i32,
                        VolumeType::Music,
                    );

                    // SFX volume setting
                    create_volume_slider(
                        parent,
                        "SFX Volume:",
                        (volume.sfx * 100.0) as i32,
                        VolumeType::Sfx,
                    );

                    // Back button
                    spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
                });
        });
}

/// Volume slider type to identify which volume is being adjusted
#[derive(Component, Clone, Copy, Debug)]
pub enum VolumeType {
    /// Master volume
    Master,
    /// Music volume
    Music,
    /// Sound effects volume
    Sfx,
}

/// Creates a volume slider control
fn create_volume_slider(
    parent: &mut ChildBuilder,
    label: &str,
    value: i32,
    volume_type: VolumeType,
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
            AppLayer::Menu.layer(),
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
                AppLayer::Menu.layer(),
            ));

            // Slider container
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button { ..default() },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    volume_type,
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Current value indicator
                    parent.spawn((
                        Node {
                            width: Val::Percent(value as f32),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.4, 0.6, 0.8, 0.8)),
                        AppLayer::Menu.layer(),
                    ));
                });

            // Value text
            parent.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                Text::new(format!("{}%", value)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                VolumeValueText(volume_type),
                AppLayer::Menu.layer(),
            ));
        });
}

/// Component to mark volume value text for updating
#[derive(Component)]
pub struct VolumeValueText(pub VolumeType);

/// Sets up the gameplay settings screen
pub fn setup_gameplay_settings(
    mut commands: Commands,
    gameplay_settings: Option<Res<GameplaySettings>>,
) {
    let settings = gameplay_settings.map(|s| s.clone()).unwrap_or_default();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            MenuItem,
            SettingsMenuItem,
            GameplaySettingsScreen,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("GAMEPLAY SETTINGS"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));

            // Settings container
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Auto-pass setting
                    create_toggle_setting(parent, "Auto-Pass Priority:", settings.auto_pass);

                    // Show tooltips setting
                    create_toggle_setting(parent, "Show Card Tooltips:", settings.show_tooltips);

                    // Animation speed setting
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
                            AppLayer::Menu.layer(),
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
                                AppLayer::Menu.layer(),
                            ));

                            // Value
                            parent.spawn((
                                Text::new(format!("{:.1}x", settings.animation_speed)),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                AppLayer::Menu.layer(),
                            ));
                        });

                    // Back button
                    spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
                });
        });
}

/// Creates a toggle setting with a label and current value
fn create_toggle_setting(parent: &mut ChildBuilder, label: &str, value: bool) {
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
            AppLayer::Menu.layer(),
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
                AppLayer::Menu.layer(),
            ));

            // Value
            parent.spawn((
                Text::new(if value { "On" } else { "Off" }),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));
        });
}

/// Sets up the controls settings screen
pub fn setup_controls_settings(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            MenuItem,
            SettingsMenuItem,
            ControlsSettingsScreen,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("CONTROLS SETTINGS"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));

            // Settings container
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    AppLayer::Menu.layer(),
                ))
                .with_children(|parent| {
                    // Create some simple key bindings display
                    create_keybinding(parent, "Pause Game:", "ESC");
                    create_keybinding(parent, "Select Card:", "Left Click");
                    create_keybinding(parent, "Card Info:", "Right Click");
                    create_keybinding(parent, "Zoom Camera:", "Mouse Wheel");

                    // Back button
                    spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
                });
        });
}

/// Creates a keybinding display
fn create_keybinding(parent: &mut ChildBuilder, action: &str, key: &str) {
    parent
        .spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            // Action
            parent.spawn((
                Text::new(action),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));

            // Key
            parent.spawn((
                Text::new(key),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));
        });
}

/// Creates a settings button with text
fn spawn_settings_button(parent: &mut ChildBuilder, text: &str, action: SettingsButtonAction) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            action,
            AppLayer::Menu.layer(),
            SettingsMenuItem,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::from_section(text, text_style()),
                TextLayout::new_with_justify(JustifyText::Center),
                AppLayer::Menu.layer(),
                SettingsMenuItem,
            ));
        });
}

/// Handles button interactions in the settings menu
pub fn settings_button_action(
    mut interaction_query: Query<
        (&Interaction, &SettingsButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<SettingsMenuState>>,
    mut game_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON);
                match action {
                    SettingsButtonAction::VideoSettings => {
                        next_state.set(SettingsMenuState::Video);
                    }
                    SettingsButtonAction::AudioSettings => {
                        next_state.set(SettingsMenuState::Audio);
                    }
                    SettingsButtonAction::GameplaySettings => {
                        next_state.set(SettingsMenuState::Gameplay);
                    }
                    SettingsButtonAction::ControlsSettings => {
                        next_state.set(SettingsMenuState::Controls);
                    }
                    SettingsButtonAction::Back => {
                        // Return to the previous menu (main menu or pause menu)
                        if let Some(origin) = context.settings_origin {
                            info!("Returning to origin state: {:?}", origin);

                            // Always reset from_pause_menu flag if returning to main menu
                            if origin == GameMenuState::MainMenu {
                                info!(
                                    "Resetting from_pause_menu flag because we're returning to main menu"
                                );
                                context.from_pause_menu = false;
                            } else if origin == GameMenuState::PausedGame {
                                // Ensure the flag is set if returning to the pause menu
                                info!(
                                    "Setting from_pause_menu flag because we're returning to pause menu"
                                );
                                context.from_pause_menu = true;
                            }

                            // First set the settings menu state to disabled to trigger cleanup
                            next_state.set(SettingsMenuState::Disabled);

                            // Log transition for debugging
                            info!("Transitioning from Settings to {:?}", origin);

                            // Then set the game menu state to the origin state
                            game_state.set(origin);
                        } else {
                            // Default to main menu if origin is not set
                            info!("No origin state found, defaulting to MainMenu");
                            context.from_pause_menu = false;
                            next_state.set(SettingsMenuState::Disabled);

                            // Log transition for debugging
                            info!("Transitioning from Settings to MainMenu (default)");

                            game_state.set(GameMenuState::MainMenu);
                        }
                    }
                    SettingsButtonAction::BackToMainSettings => {
                        next_state.set(SettingsMenuState::Main);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}

/// Cleans up settings menu entities
pub fn cleanup_settings_menu(
    mut commands: Commands,
    menu_query: Query<(Entity, Option<&Name>), With<SettingsMenuItem>>,
    input_blockers: Query<Entity, With<crate::menu::input_blocker::InputBlocker>>,
) {
    let count = menu_query.iter().count();
    info!("Cleaning up {} settings menu items", count);

    for (entity, name) in menu_query.iter() {
        if let Some(name) = name {
            info!(
                "Despawning settings menu entity: {:?} with name: {}",
                entity,
                name.as_str()
            );
        } else {
            info!("Despawning settings menu entity without name: {:?}", entity);
        }
        commands.entity(entity).despawn_recursive();
    }

    // Explicitly clean up any input blockers
    let blocker_count = input_blockers.iter().count();
    if blocker_count > 0 {
        info!(
            "Cleaning up {} input blockers from settings menu",
            blocker_count
        );
        for entity in input_blockers.iter() {
            info!("Despawning input blocker: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    if count == 0 {
        warn!("No settings menu entities found to clean up");
    }
}

/// System to handle interactions with volume sliders
pub fn volume_slider_interaction(
    mut interaction_query: Query<
        (&Interaction, &VolumeType, &Node, &GlobalTransform),
        (Changed<Interaction>, With<Button>),
    >,
    volume_type_query: Query<&VolumeType>,
    mut volume_settings: ResMut<VolumeSettings>,
    mut text_query: Query<(&mut Text, &VolumeValueText)>,
    mut volume_indicators: Query<(&mut Node, &Parent), Without<Button>>,
    mut global_volume: ResMut<bevy::prelude::GlobalVolume>,
    mut audio_players: Query<&mut bevy::audio::PlaybackSettings>,
    mut persistent_settings: Option<ResMut<Persistent<RummageSettings>>>,
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
        for (interaction, volume_type, node, transform) in interaction_query.iter_mut() {
            if let Interaction::Pressed = *interaction {
                // Get the width as a concrete value, providing minimum and default values
                let button_size = match node.width {
                    Val::Px(px) => px,
                    Val::Percent(pct) => pct * 2.0, // Approximate conversion
                    _ => 200.0,                     // Default value if we can't determine
                };

                let button_pos = transform.translation().x;

                // Calculate relative position (0.0 - 1.0) manually
                let relative_x = ((cursor_pos.x - (button_pos - button_size / 2.0)) / button_size)
                    .clamp(0.0, 1.0);
                let clamped_value = (relative_x * 100.0).round() as i32;
                let volume_value = clamped_value as f32 / 100.0;

                // Update the appropriate volume setting
                match *volume_type {
                    VolumeType::Master => {
                        volume_settings.master = volume_value;
                        // Update global volume
                        global_volume.volume = bevy::audio::Volume::new(volume_value);
                    }
                    VolumeType::Music => {
                        volume_settings.music = volume_value;
                        // Update music players (you'd need a way to identify music players)
                        // This is a simplified approach
                        for mut settings in audio_players.iter_mut() {
                            settings.volume = bevy::audio::Volume::new(volume_value);
                        }
                    }
                    VolumeType::Sfx => {
                        volume_settings.sfx = volume_value;
                        // SFX would be handled when playing new sounds
                    }
                }

                // Update the text display
                for (mut text, value_text) in text_query.iter_mut() {
                    if value_text.0 as u8 == *volume_type as u8 {
                        text.0 = format!("{}%", clamped_value);
                    }
                }

                // Update the visual slider
                for (mut indicator_node, parent) in volume_indicators.iter_mut() {
                    // Use the separate query to get the parent's volume type
                    if let Ok(parent_volume_type) = volume_type_query.get(parent.get()) {
                        if *parent_volume_type as u8 == *volume_type as u8 {
                            indicator_node.width = Val::Percent(clamped_value as f32);
                        }
                    }
                }

                // Save settings immediately
                if let Some(persistent) = persistent_settings.as_mut() {
                    let mut settings = (*persistent).clone();
                    settings.volume = volume_settings.clone();
                    if let Err(e) = persistent.set(settings) {
                        error!("Failed to save volume settings: {:?}", e);
                    }
                }

                info!("Volume {:?} set to {}%", volume_type, clamped_value);
            }
        }
    }
}

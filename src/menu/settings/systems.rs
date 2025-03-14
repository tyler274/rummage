use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
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
                    create_volume_slider(parent, "Master Volume:", (volume.master * 100.0) as i32);

                    // Music volume setting
                    create_volume_slider(parent, "Music Volume:", (volume.music * 100.0) as i32);

                    // SFX volume setting
                    create_volume_slider(parent, "SFX Volume:", (volume.sfx * 100.0) as i32);

                    // Back button
                    spawn_settings_button(parent, "Back", SettingsButtonAction::BackToMainSettings);
                });
        });
}

/// Creates a volume slider control
fn create_volume_slider(parent: &mut ChildBuilder, label: &str, value: i32) {
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
                Text::new(format!("{}%", value)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
            ));
        });
}

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
            button_style(),
            BackgroundColor(NORMAL_BUTTON),
            Button,
            action,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                text_style(),
                TextLayout::new_with_justify(JustifyText::Center),
                AppLayer::Menu.layer(),
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
    context: Res<StateTransitionContext>,
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
                            game_state.set(origin);
                        } else {
                            // Default to main menu if origin is not set
                            info!("No origin state found, defaulting to MainMenu");
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

    if count == 0 {
        warn!("No settings menu entities found to clean up");
    }
}

use super::common::*;
use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use crate::menu::settings::state::SettingsMenuState;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Sets up the main settings menu
pub fn setup_main_settings(mut commands: Commands) {
    info!("Setting up main settings menu");

    let root_entity = spawn_settings_root(
        &mut commands,
        Color::srgba(0.0, 0.0, 0.0, 0.7),
        "Main Settings",
    );

    // Store container entity outside the closure
    let mut container_entity = Entity::PLACEHOLDER;

    // Store root_entity for later use
    let mut root = commands.entity(root_entity);

    // Create a new scope for the first with_children call
    root.with_children(|parent| {
        spawn_settings_title(parent, "Settings");

        // Create a container and store its entity
        container_entity = spawn_settings_container(parent);
    });

    // Add buttons inside the container as a separate step
    commands.entity(container_entity).with_children(|parent| {
        spawn_settings_button(parent, "Video", SettingsButtonAction::NavigateToVideo);
        spawn_settings_button(parent, "Audio", SettingsButtonAction::NavigateToAudio);
        spawn_settings_button(parent, "Gameplay", SettingsButtonAction::NavigateToGameplay);
        spawn_settings_button(parent, "Controls", SettingsButtonAction::NavigateToControls);
        spawn_settings_button(parent, "Back", SettingsButtonAction::ExitSettings);
    });
}

/// Handles button actions in the settings menu
pub fn settings_button_action(
    mut interaction_query: Query<
        (&Interaction, &SettingsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<SettingsMenuState>>,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            info!("Settings button pressed: {:?}", action);
            match action {
                SettingsButtonAction::NavigateToVideo => next_state.set(SettingsMenuState::Video),
                SettingsButtonAction::NavigateToAudio => next_state.set(SettingsMenuState::Audio),
                SettingsButtonAction::NavigateToGameplay => {
                    next_state.set(SettingsMenuState::Gameplay)
                }
                SettingsButtonAction::NavigateToControls => {
                    next_state.set(SettingsMenuState::Controls)
                }
                SettingsButtonAction::NavigateToMain => next_state.set(SettingsMenuState::Main),
                SettingsButtonAction::ExitSettings => {
                    next_state.set(SettingsMenuState::Disabled);
                    game_menu_state.set(GameMenuState::MainMenu);
                }
            }
        }
    }
}

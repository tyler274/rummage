use crate::menu::decorations::MenuDecorativeElement;
use crate::menu::logo::text::{create_english_text, create_hebrew_text};
use crate::menu::star_of_david::create_star_of_david;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::MainMenu), setup_combined_logo)
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_logo)
            .add_systems(OnExit(GameMenuState::MainMenu), cleanup_logo)
            .add_systems(OnExit(GameMenuState::PausedGame), cleanup_logo);

        debug!("Logo plugin registered - combines Star of David with text");
    }
}

/// Sets up the combined logo with Star of David and text for the main menu
fn setup_combined_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up combined logo with Star of David and text for main menu");

    commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuDecorativeElement,
            Name::new("Main Menu Logo Container"),
        ))
        .with_children(|parent| {
            // Spawn the Star of David as part of the logo
            parent.spawn((create_star_of_david(), Name::new("Main Menu Star of David")));

            // Add Hebrew text
            parent.spawn((
                create_hebrew_text(&asset_server),
                Name::new("Main Menu Hebrew Text"),
            ));

            // Add English text
            parent.spawn((
                create_english_text(&asset_server),
                Name::new("Main Menu English Text"),
            ));
        });
}

/// Sets up the combined logo with Star of David and text for the pause menu
fn setup_pause_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up combined logo with Star of David and text for pause menu");

    commands
        .spawn((
            Node {
                width: Val::Px(250.0), // Slightly smaller for pause menu
                height: Val::Px(350.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuDecorativeElement,
            Name::new("Pause Menu Logo Container"),
        ))
        .with_children(|parent| {
            // Spawn the Star of David as part of the logo
            parent.spawn((
                create_star_of_david(),
                Name::new("Pause Menu Star of David"),
            ));

            // Add Hebrew text
            parent.spawn((
                create_hebrew_text(&asset_server),
                Name::new("Pause Menu Hebrew Text"),
            ));

            // Add English text
            parent.spawn((
                create_english_text(&asset_server),
                Name::new("Pause Menu English Text"),
            ));
        });
}

/// Cleans up the logo container and all its children
fn cleanup_logo(mut commands: Commands, logo_containers: Query<(Entity, &Name)>) {
    for (entity, _name) in logo_containers
        .iter()
        .filter(|(_, name)| name.as_str().contains("Logo Container"))
    {
        info!("Cleaning up logo container: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

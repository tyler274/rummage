use crate::camera::components::AppLayer;
use crate::menu::{components::*, styles::button_styles::*};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, Val};

/// Creates a button for the pause menu
pub fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON.into()),
            MenuItem,
            action,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(Color::WHITE),
                MenuItem,
                AppLayer::Menu.layer(),
            ));
        });
}

/// Creates buttons for the pause menu
pub fn create_pause_menu_buttons(commands: &mut Commands, asset_server: &AssetServer) {
    // Create Resume button
    commands
        .spawn((
            Name::new("Resume Game Button"),
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON.into()),
            MenuButtonAction::Resume,
            MenuItem,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Resume"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

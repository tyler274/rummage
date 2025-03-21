use crate::camera::components::AppLayer;
use crate::menu::{components::*, styles::*};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, Val};

/// Creates a button for the pause menu
pub fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    _asset_server: &AssetServer,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            MenuItem,
            action,
            AppLayer::Menu.layer(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                MenuItem,
                AppLayer::Menu.layer(),
            ));
        });
} 
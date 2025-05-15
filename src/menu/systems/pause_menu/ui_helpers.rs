use bevy::{prelude::*, text::JustifyText};

use crate::{
    camera::components::AppLayer,
    menu::{
        components::{MenuButtonAction, MenuItem, ZLayers},
        styles::NORMAL_BUTTON,
    },
};

/// Spawns a standard menu button with text within the pause menu context.
pub(super) fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    button_text: &str,
    action: MenuButtonAction,
    button_name: &str,
) {
    parent
        .spawn((
            Name::new(button_name.to_string()),
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            action,
            MenuItem, // Mark the button itself as a MenuItem for cleanup
            ZIndex::from(ZLayers::MenuButtons),
            AppLayer::Menu.layer(), // Ensure button is on the correct layer
        ))
        .with_children(|text_parent| {
            text_parent.spawn((
                Text::new(button_text),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                MenuItem, // Mark text as MenuItem for cleanup (could potentially be removed if parent cleanup is recursive)
                ZIndex::from(ZLayers::MenuButtonText),
                AppLayer::Menu.layer(), // Ensure text is on the correct layer
            ));
        });
}

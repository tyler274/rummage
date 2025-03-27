use crate::menu::components::{MenuItem, ZLayers};
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{UiRect, Val};

/// Creates the Hebrew text "Rummage" (רומאג')
pub fn create_hebrew_text(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect {
                top: Val::Px(20.0), // Reduced space between Star and text
                ..default()
            },
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            ..default()
        },
        Text::new("רומאג'"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgb(0.85, 0.65, 0.13)),
        BackgroundColor(Color::NONE),
        MenuItem,
        Visibility::Visible,
        ZIndex::from(ZLayers::MenuButtonText),
    )
}

/// Creates the English text "Rummage"
pub fn create_english_text(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(Val::Px(5.0)), // Reduced space between texts
            width: Val::Px(200.0),
            height: Val::Px(40.0),
            ..default()
        },
        Text::new("Rummage"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgb(0.85, 0.65, 0.13)),
        BackgroundColor(Color::NONE),
        MenuItem,
        Visibility::Visible,
        ZIndex::from(ZLayers::MenuButtonText),
    )
}

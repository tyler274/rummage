use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, PositionType, UiRect, Val};

/// Creates a vectorized Star of David with Hebrew text
pub fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

/// Creates the Hebrew text "Rummage" (רומאג')
pub fn create_hebrew_text(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
        Text::new("רומאג'"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans.ttf"),
            font_size: 64.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgb(0.85, 0.65, 0.13)),
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

/// Creates the English text "Rummage"
pub fn create_english_text(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
        Text::new("Rummage"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgb(0.85, 0.65, 0.13)),
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

/// Creates the decorative elements around the logo
pub fn create_decorative_elements() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

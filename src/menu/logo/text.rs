use crate::camera::components::AppLayer;
use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, PositionType, UiRect, Val};

/// Creates a container for the logo group (Star of David + text)
pub fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(300.0),
            height: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        AppLayer::Menu.layer(), // Top-level element needs explicit RenderLayers
        Interaction::None,
    )
}

/// Creates the Hebrew text "Rummage" (רומאג')
pub fn create_hebrew_text(asset_server: &AssetServer) -> impl Bundle {
    // No RenderLayers component - will inherit from parent
    (
        Node {
            margin: UiRect {
                top: Val::Px(150.0), // Position below the Star of David
                ..default()
            },
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
    // No RenderLayers component - will inherit from parent
    (
        Node {
            margin: UiRect {
                top: Val::Px(210.0), // Position below the Hebrew text
                ..default()
            },
            ..default()
        },
        Text::new("Rummage"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans.ttf"),
            font_size: 28.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgb(0.85, 0.65, 0.13)),
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

/// Creates decorative elements around the logo
pub fn create_decorative_elements() -> impl Bundle {
    // No RenderLayers component - will inherit from parent
    (
        Node {
            margin: UiRect {
                top: Val::Px(270.0), // Position below the English text
                ..default()
            },
            position_type: PositionType::Relative,
            ..default()
        },
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

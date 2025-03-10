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

/// Creates the Star of David shape
fn create_star_of_david() -> impl Bundle {
    (
        Node {
            width: Val::Px(120.0),
            height: Val::Px(120.0),
            position_type: PositionType::Relative,
            ..default()
        },
        BackgroundColor(Color::NONE),
        Interaction::None,
    )
}

/// Creates the Hebrew text "Rummage" (רומאג')
fn create_hebrew_text() -> impl Bundle {
    (
        Node {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
        Text::new("רומאג'"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        BackgroundColor(Color::rgb(1.0, 0.84, 0.0)), // Gold color
        Interaction::None,
    )
}

/// Creates the English text "Rummage"
fn create_english_text() -> impl Bundle {
    (
        Node {
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
        Text::new("Rummage"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        BackgroundColor(Color::rgb(1.0, 0.84, 0.0)), // Gold color
        Interaction::None,
    )
}

/// Creates the decorative elements around the logo
fn create_decorative_elements() -> impl Bundle {
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

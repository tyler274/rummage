use bevy::prelude::*;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

/// Creates a vectorized Star of David with Hebrew text
pub fn create_logo() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
        ..default()
    }
}

/// Creates the Star of David shape
fn create_star_of_david() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(120.0),
            height: Val::Px(120.0),
            position_type: PositionType::Relative,
            ..default()
        },
        ..default()
    }
}

/// Creates the Hebrew text "Rummage" (רומאג')
fn create_hebrew_text() -> TextBundle {
    TextBundle::from_sections([TextSection::new(
        "רומאג'",
        TextStyle {
            font_size: 48.0,
            color: Color::GOLD,
            ..default()
        },
    )])
    .with_style(Style {
        margin: UiRect::top(Val::Px(20.0)),
        ..default()
    })
}

/// Creates the English text "Rummage"
fn create_english_text() -> TextBundle {
    TextBundle::from_sections([TextSection::new(
        "Rummage",
        TextStyle {
            font_size: 24.0,
            color: Color::GOLD,
            ..default()
        },
    )])
    .with_style(Style {
        margin: UiRect::top(Val::Px(10.0)),
        ..default()
    })
}

/// Creates the decorative elements around the logo
fn create_decorative_elements() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    }
}

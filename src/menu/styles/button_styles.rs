use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

/// Creates a standard menu button with consistent styling
pub fn create_main_menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }
}

/// Creates text for a menu button with consistent styling
pub fn create_main_menu_button_text(asset_server: &AssetServer, text: &str) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::Center),
        ..default()
    }
}

/// Creates a smaller settings button
pub fn create_settings_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }
}

/// Creates smaller text for settings buttons
pub fn create_settings_button_text(asset_server: &AssetServer, text: &str) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::Center),
        ..default()
    }
}

/// Creates a checkbox for settings
pub fn create_settings_checkbox(is_checked: bool) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: if is_checked {
            Color::rgb(0.4, 0.4, 0.8).into()
        } else {
            Color::rgb(0.15, 0.15, 0.15).into()
        },
        ..default()
    }
}

/// Creates a slider container for settings
pub fn create_settings_slider_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(300.0),
            height: Val::Px(20.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }
}

/// Creates a slider fill for settings
pub fn create_settings_slider_fill(value: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(value * 100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::rgb(0.4, 0.4, 0.8).into(),
        ..default()
    }
}

/// Creates a container for settings options
pub fn create_settings_option_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(500.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

/// Creates label text for settings
pub fn create_settings_label(asset_server: &AssetServer, text: &str) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        style: Style {
            margin: UiRect::right(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }
}

use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

/// Normal button color
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

/// Hovered button color
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

/// Pressed button color
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

/// Creates a standard button style
pub fn button_style() -> Node {
    Node {
        width: Val::Px(180.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    }
}

/// Creates a standard font style component
pub fn create_text_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        ..default()
    }
}

/// Creates components for a main menu button with consistent styling
pub fn create_main_menu_button() -> (Button, Node, BackgroundColor) {
    (
        Button,
        Node {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )
}

/// Creates text for a menu button with consistent styling
pub fn create_main_menu_button_text(
    asset_server: &AssetServer,
    text: &str,
) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(text.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    )
}

/// Creates components for a settings button with consistent styling
pub fn create_settings_button() -> (Button, Node, BackgroundColor) {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )
}

/// Creates smaller text for settings buttons
pub fn create_settings_button_text(
    asset_server: &AssetServer,
    text: &str,
) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(text.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    )
}

/// Creates components for a settings checkbox
pub fn create_settings_checkbox(is_checked: bool) -> (Button, Node, BackgroundColor) {
    (
        Button,
        Node {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(if is_checked {
            Color::srgb(0.4, 0.4, 0.8)
        } else {
            Color::srgb(0.15, 0.15, 0.15)
        }),
    )
}

/// Creates components for a slider container
pub fn create_settings_slider_container() -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(300.0),
            height: Val::Px(20.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

/// Creates components for a slider fill element
pub fn create_settings_slider_fill(value: f32) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Percent(value * 100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.4, 0.4, 0.8)),
    )
}

/// Creates components for a settings option container
pub fn create_settings_option_container() -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(500.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )
}

/// Creates label text for settings
pub fn create_settings_label(
    asset_server: &AssetServer,
    text: &str,
) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(text.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    )
}

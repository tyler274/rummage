use bevy::prelude::*;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

/// Normal state color for buttons
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
/// Hover state color for buttons
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
/// Pressed state color for buttons
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// Menu button style
pub fn button_style() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// Menu text style
pub fn text_style() -> TextFont {
    TextFont {
        font_size: 20.0,
        ..default()
    }
}

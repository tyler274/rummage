use bevy::prelude::*;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

/// Normal state color for buttons
pub const NORMAL_BUTTON: Color = Color::srgb(0.3, 0.3, 0.3);
/// Hover state color for buttons
pub const HOVERED_BUTTON: Color = Color::srgb(0.4, 0.4, 0.4);
/// Pressed state color for buttons
pub const PRESSED_BUTTON: Color = Color::srgb(0.45, 0.85, 0.45);

/// Menu button style
pub fn button_style() -> Node {
    Node {
        width: Val::Px(220.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(12.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// Menu text style
pub fn text_style() -> TextFont {
    TextFont {
        font_size: 24.0,
        ..default()
    }
}

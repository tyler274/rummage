use bevy::prelude::*;

/// Creates a standard text style for menu items
pub fn text_style() -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: Color::WHITE,
        ..default()
    }
}

/// Creates a standard text style with a specific font and size
pub fn text_style_with_font(asset_server: &AssetServer, size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        color: Color::WHITE,
    }
}

/// Creates a standard text style with a specific font, size and color
pub fn text_style_with_font_and_color(
    asset_server: &AssetServer,
    size: f32,
    color: Color,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        color,
    }
}

/// Creates a Text2dBundle with the given text and standard styling
pub fn create_text_2d_bundle(text: &str, asset_server: &AssetServer) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(text, text_style_with_font(asset_server, 24.0)),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }
}

/// Creates a bundle for menu text with standard styling
pub fn create_menu_text_bundle(text: &str, asset_server: &AssetServer) -> impl Bundle {
    (
        Text::from_section(text, text_style_with_font(asset_server, 24.0)),
        TextLayoutBundle::default(),
    )
}

/// Creates a bundle for button text with standard styling
pub fn create_button_text_bundle(text: &str, asset_server: &AssetServer) -> impl Bundle {
    (
        Text::from_section(text, text_style_with_font(asset_server, 24.0)),
        TextLayoutBundle {
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            ..default()
        },
    )
}

use bevy::prelude::*;

/// Creates a standard text font for menu items
pub fn text_font() -> TextFont {
    TextFont {
        font_size: 24.0,
        ..default()
    }
}

/// Creates a standard text font with a specific font and size
pub fn text_font_with_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        ..default()
    }
}

/// Creates a standard text font with a specific font, size and color
pub fn text_font_with_font_and_color(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        ..default()
    }
}

/// Creates a Text2d with the given text and standard styling
#[derive(Bundle)]
pub struct Text2dBundle {
    /// 2D text marker component
    pub text_2d_marker: Text2d,
    /// Text content
    pub text: Text,
    /// Font configuration
    pub font: TextFont,
    /// Text color
    pub color: TextColor,
    /// Transform component
    pub transform: Transform,
    /// Global transform component
    pub global_transform: GlobalTransform,
}

impl Text2dBundle {
    /// Create a new Text2d bundle with the given text
    pub fn new(text: &str, asset_server: &AssetServer) -> Self {
        Self {
            text_2d_marker: Text2d::default(),
            text: Text::new(text),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            global_transform: GlobalTransform::default(),
        }
    }
}

/// Creates a bundle for menu text with standard styling
#[derive(Bundle)]
pub struct MenuTextBundle {
    /// Text content
    pub text: Text,
    /// Font configuration
    pub font: TextFont,
    /// Text color
    pub color: TextColor,
    /// Text layout
    pub layout: TextLayout,
}

impl MenuTextBundle {
    /// Create a new menu text bundle with the given text
    pub fn new(text: &str, asset_server: &AssetServer) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::default(),
        }
    }
}

/// Creates a bundle for button text with standard styling
#[derive(Bundle)]
pub struct ButtonTextBundle {
    /// Text content
    pub text: Text,
    /// Font configuration
    pub font: TextFont,
    /// Text color
    pub color: TextColor,
    /// Text layout
    pub layout: TextLayout,
}

impl ButtonTextBundle {
    /// Create a new button text bundle with the given text
    pub fn new(text: &str, asset_server: &AssetServer) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(JustifyText::Center),
        }
    }
}

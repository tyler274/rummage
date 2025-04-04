use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

use crate::menu::components::{MenuButtonAction, MenuItem, MenuRoot};
use crate::menu::styles::button_styles::create_main_menu_button;

/// Creates text components for a menu button
pub fn create_main_menu_button_text(
    asset_server: &AssetServer,
    text_str: &str,
) -> (Text, TextFont, TextColor, TextLayout) {
    (
        Text::new(text_str.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    )
}

/// Bundle for a menu button
#[derive(Bundle)]
pub struct MenuButtonBundle {
    /// Button component
    pub button: Button,
    /// Node component
    pub node: Node,
    /// Background color
    pub background: BackgroundColor,
    /// Name of the button
    pub name: Name,
    /// MenuItem marker component
    pub menu_item: MenuItem,
    /// Visibility component
    pub visibility: Visibility,
    /// Inherited visibility component
    pub inherited_visibility: InheritedVisibility,
    /// View visibility component
    pub view_visibility: ViewVisibility,
    /// Z-index for layering
    pub z_index: ZIndex,
}

impl MenuButtonBundle {
    /// Create a new menu button bundle with the given name and z-index
    #[allow(unused_variables)]
    pub fn new(button_name: &str, z_index: i32) -> Self {
        let (button, node, background) = create_main_menu_button();

        Self {
            button,
            node,
            background,
            name: Name::new(format!("{} Button", button_name)),
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::default(),
        }
    }
}

/// Bundle for a menu container
#[derive(Bundle)]
pub struct MenuContainerBundle {
    /// Node component
    pub node: Node,
    /// Name of the container
    pub name: Name,
    /// MenuItem marker component
    pub menu_item: MenuItem,
    /// Visibility component
    pub visibility: Visibility,
    /// Inherited visibility component
    pub inherited_visibility: InheritedVisibility,
    /// View visibility component
    pub view_visibility: ViewVisibility,
    /// Z-index for layering
    pub z_index: ZIndex,
}

impl MenuContainerBundle {
    /// Create a new main menu button container
    pub fn button_container() -> Self {
        Self {
            node: Node {
                width: Val::Px(200.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(20.0)),
                ..default()
            },
            name: Name::new("Main Menu Button Container"),
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::default(),
        }
    }
}

/// Bundle for the main menu root
#[derive(Bundle)]
pub struct MenuRootBundle {
    /// Node component
    pub node: Node,
    /// Background color
    pub background: BackgroundColor,
    /// Name of the root
    pub name: Name,
    /// MenuRoot marker component
    pub menu_root: MenuRoot,
    /// MenuItem marker component
    pub menu_item: MenuItem,
    /// Visibility component
    pub visibility: Visibility,
    /// Inherited visibility component
    pub inherited_visibility: InheritedVisibility,
    /// View visibility component
    pub view_visibility: ViewVisibility,
    /// Z-index for layering
    pub z_index: ZIndex,
}

impl MenuRootBundle {
    /// Create a new main menu root
    pub fn new() -> Self {
        Self {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background: BackgroundColor(Color::NONE),
            name: Name::new("Main Menu Root"),
            menu_root: MenuRoot,
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::default(),
        }
    }
}

impl Default for MenuRootBundle {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates the main menu buttons
pub fn create_main_menu_buttons(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    save_exists: bool,
) {
    // Create the buttons container
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(300.0),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
            MenuItem,
            Name::new("Menu Buttons Container"),
        ))
        .with_children(|container| {
            // New Game button
            spawn_menu_button(
                container,
                "New Game",
                MenuButtonAction::NewGame,
                asset_server,
            );

            // Load Game button - only show if save exists
            if save_exists {
                spawn_menu_button(
                    container,
                    "Load Game",
                    MenuButtonAction::LoadGame,
                    asset_server,
                );
            }

            // Multiplayer button
            spawn_menu_button(
                container,
                "Multiplayer",
                MenuButtonAction::Multiplayer,
                asset_server,
            );

            // Settings button
            spawn_menu_button(
                container,
                "Settings",
                MenuButtonAction::Settings,
                asset_server,
            );

            // Credits button
            spawn_menu_button(
                container,
                "Credits",
                MenuButtonAction::Credits,
                asset_server,
            );

            // Exit button
            spawn_menu_button(container, "Exit", MenuButtonAction::Quit, asset_server);
        });
}

/// Spawns a menu button with the given text and action
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(250.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            Button,
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.8)),
            action,
            MenuItem,
            Name::new(format!("{} Button", text)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout::new_with_justify(JustifyText::Center),
                MenuItem,
                Name::new(format!("{} Text", text)),
            ));
        });
}

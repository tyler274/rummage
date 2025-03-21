use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

use super::super::components::{MainMenuButton, MainMenuContainer};
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
    /// Main menu button marker
    pub main_menu_button: MainMenuButton,
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
    /// Create a new main menu button
    pub fn new(button_name: &str, z_index: i32) -> Self {
        let (button, node, background) = create_main_menu_button();
        Self {
            button,
            node,
            background,
            name: Name::new(button_name.to_string()),
            menu_item: MenuItem,
            main_menu_button: MainMenuButton,
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex(z_index),
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
    /// Main menu container marker
    pub main_menu_container: MainMenuContainer,
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
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            name: Name::new("Main Menu Buttons Container"),
            menu_item: MenuItem,
            main_menu_container: MainMenuContainer,
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex(10),
        }
    }
}

/// Bundle for a menu root
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
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background: BackgroundColor(Color::NONE),
            name: Name::new("Main Menu Root"),
            menu_root: MenuRoot,
            menu_item: MenuItem,
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex(1),
        }
    }
}

/// Creates the main menu buttons
pub fn create_main_menu_buttons(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    save_exists: bool,
) {
    // Create the container for buttons
    parent
        .spawn(MenuContainerBundle::button_container())
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Rummage"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                MenuItem,
                ZIndex(15),
                Name::new("Main Menu Title"),
            ));

            // Subtitle - Divider
            parent.spawn((
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(2.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.2)),
                MenuItem,
                ZIndex(15),
                Name::new("Title Divider"),
            ));

            // New Game button
            spawn_menu_button(parent, "New Game", MenuButtonAction::NewGame, asset_server);

            // Continue button (only if save exists)
            if save_exists {
                spawn_menu_button(parent, "Continue", MenuButtonAction::Continue, asset_server);
            }

            // Settings button
            spawn_menu_button(parent, "Settings", MenuButtonAction::Settings, asset_server);

            // Credits button
            spawn_menu_button(parent, "Credits", MenuButtonAction::Credits, asset_server);

            // Quit button
            spawn_menu_button(parent, "Quit", MenuButtonAction::Quit, asset_server);
        });
}

/// Helper function to spawn a menu button with consistent styling
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuButtonAction,
    asset_server: &AssetServer,
) {
    // Create the button entity
    parent
        .spawn((
            MenuButtonBundle::new(&format!("{} Button", text), 15),
            action, // Store the action with the button
        ))
        .with_children(|parent| {
            // Add the text as a child of the button
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                MenuItem,
                Name::new(format!("{} Text", text)),
            ));
        });
}

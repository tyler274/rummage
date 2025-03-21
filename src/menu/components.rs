use bevy::prelude::*;

/// Marker component for menu items
#[derive(Component, Debug)]
pub struct MenuItem;

/// Marker component for menu root elements (for standalone UI)
#[derive(Component, Debug)]
pub struct MenuRoot;

/// Resource for tracking visible menu items
#[derive(Resource, Debug, Default)]
pub struct MenuVisibilityState {
    /// Number of visible menu items
    pub visible_items: usize,
}

/// Resource to indicate that the main menu needs to be set up
#[derive(Resource, Debug, Default)]
pub struct NeedsMainMenuSetup(pub bool);

/// Resource to track if we've checked for UI hierarchy issues
#[derive(Resource, Debug, Default, PartialEq)]
pub struct UiHierarchyChecked(pub bool);

/// Marker component for the game camera
#[allow(dead_code)]
#[derive(Component)]
pub struct GameCamera;

/// Button actions for different menu states
#[derive(Component, Clone, Debug)]
pub enum MenuButtonAction {
    /// Start a new game session
    NewGame,
    /// Continue a previously saved game
    Continue,
    /// Load a previously saved game
    LoadGame,
    /// Enter multiplayer mode
    Multiplayer,
    /// Open settings menu
    Settings,
    /// Exit the game
    Quit,
    /// Resume the current game
    Resume,
    /// Restart the current game with a new hand
    Restart,
    /// Return to the main menu
    MainMenu,
    /// Save the current game
    SaveGame,
    /// Show credits screen
    Credits,
}

/// Bundle for menu items that combines commonly used components
#[derive(Bundle)]
pub struct MenuItemBundle {
    /// The name of the menu item for debugging
    pub name: Name,
    /// The menu item component
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

impl Default for MenuItemBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Menu Item"),
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::default(),
        }
    }
}

/// Bundle for menu text that combines commonly used components
#[derive(Bundle)]
pub struct MenuTextBundle {
    /// The text component
    pub text: Text,
    /// The text layout component
    pub text_layout: TextLayout,
    /// Menu item marker
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

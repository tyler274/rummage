use bevy::prelude::*;

/// Component marking an entity as a Star of David graphical element
#[derive(Component, Debug)]
pub struct StarOfDavid;

/// Bundle for creating the Star of David logo
#[derive(Bundle)]
pub struct StarOfDavidBundle {
    /// The Star of David component
    pub star: StarOfDavid,
    /// The Star of David name
    pub name: Name,
    /// Visibility component
    pub visibility: Visibility,
    /// Inherited visibility component
    pub inherited_visibility: InheritedVisibility,
    /// View visibility component
    pub view_visibility: ViewVisibility,
}

impl StarOfDavidBundle {
    /// Create a new Star of David bundle with the given name
    pub fn new(name_str: &str) -> Self {
        Self {
            star: StarOfDavid,
            name: Name::new(String::from(name_str)),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

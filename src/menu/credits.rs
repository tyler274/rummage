use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Plugin for handling the credits screen functionality
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::Credits), setup_credits)
            .add_systems(Update, handle_credits_esc_key);

        info!("CreditsPlugin initialized");
    }
}

/// Set up the credits screen UI
pub fn setup_credits(_commands: Commands, _asset_server: Res<AssetServer>) {
    info!("Setting up credits screen");
    // TODO: Implement credits screen content
}

/// Handle ESC key press in credits screen
pub fn handle_credits_esc_key(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    // Only handle ESC key in credits state
    if *state.get() == GameMenuState::Credits && keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed in credits screen - returning to main menu");
        next_state.set(GameMenuState::MainMenu);
    }
}

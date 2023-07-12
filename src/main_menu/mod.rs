use self::systems::{setup, teardown, transition_to_game};
use crate::GameState;
use bevy::prelude::*;
mod components;
mod systems;
/**
 * sett state for lukking
 * hello world fader ut
 */
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                transition_to_game.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), teardown);
    }
}

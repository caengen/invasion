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
        app.add_system(setup.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(transition_to_game.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(teardown.in_schedule(OnExit(GameState::MainMenu)));
    }
}

use self::systems::{setup, teardown, transition_to_game};
use crate::AppState;
use bevy::prelude::*;
mod data;
mod systems;
/**
 * sett state for lukking
 * hello world fader ut
 */
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(transition_to_game.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(teardown.in_schedule(OnExit(AppState::MainMenu)));
    }
}

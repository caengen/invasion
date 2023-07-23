use bevy::prelude::*;

mod systems;
use systems::*;

use crate::{game::prelude::stage_colors, GameState};

// Loads the level from json, setups level resources and show the
// level introduction
pub struct EnterStagePlugin;
impl Plugin for EnterStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnterGame), setup_stage)
            .add_systems(
                Update,
                (
                    show_stage_intro,
                    stage_intro_keys,
                    stage_intro_timer,
                    stage_colors,
                )
                    .run_if(in_state(GameState::EnterGame)),
            )
            .add_systems(
                OnExit(GameState::EnterGame),
                (setup_resources, teardown_stage).chain(),
            );
    }
}

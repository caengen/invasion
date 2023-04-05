use self::{
    data::{Paused, PhysicsSet},
    effects::flick_system,
    systems::{
        animate_sprite, example_setup, example_update, game_keys, is_paused, pause_controls,
        setup_player, teardown,
    },
};
use crate::AppState;
use bevy::prelude::*;

mod collision;
mod data;
mod effects;
mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((example_setup, setup_player).in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    game_keys.run_if(not(is_paused)),
                    animate_sprite.run_if(not(is_paused)),
                    example_update.run_if(not(is_paused)),
                    flick_system.run_if(not(is_paused)),
                    pause_controls,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
            .add_system(teardown.in_schedule(OnExit(AppState::InGame)))
            .insert_resource(Paused(false));
    }
}

use self::{
    data::{Paused, PhysicsSet},
    systems::{
        animate_sprite, example_setup, example_update, game_keys, is_not_paused, pause_controls,
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
                    game_keys,
                    animate_sprite,
                    example_update.run_if(is_not_paused),
                    pause_controls,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
            .add_system(teardown.in_schedule(OnExit(AppState::InGame)))
            .insert_resource(Paused(false));
    }
}

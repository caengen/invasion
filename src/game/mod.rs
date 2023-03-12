use self::{
    data::{Paused, PhysicsSet},
    systems::{example_update, game_keys, is_not_paused, paused, setup, teardown},
};
use crate::{random::Random, AppState};
use bevy::{math::vec2, prelude::*};
use rand::Rng;

mod collision;
mod data;
mod effects;
mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (game_keys, example_update.run_if(is_not_paused), paused)
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
            .add_system(teardown.in_schedule(OnExit(AppState::InGame)))
            .insert_resource(Paused(false));
    }
}

use self::{
    components::{Paused, PhysicsSet},
    effects::flick_system,
    systems::{
        animate_sprite, example_setup, example_update, game_keys, move_cursor, pause_controls,
        setup_player, teardown,
    },
};
use crate::GameState;
use bevy::prelude::*;

mod collision;
mod components;
mod effects;
mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            example_setup.in_schedule(OnEnter(GameState::InGame)),
            setup_player.in_schedule(OnEnter(GameState::InGame)),
        ))
        .add_systems(
            (
                game_keys,
                animate_sprite,
                example_update,
                move_cursor,
                flick_system,
                pause_controls,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
        .add_system(teardown.in_schedule(OnExit(GameState::InGame)))
        .insert_resource(Paused(false));
    }
}

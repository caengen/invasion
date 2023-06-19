use self::{
    components::PhysicsSet,
    effects::{flick_system, timed_removal_system},
    systems::{animate_sprite, change_colors, game_keys, move_cursor, setup_cursor, teardown},
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
        app.add_systems((setup_cursor.in_schedule(OnEnter(GameState::InGame)),))
            .add_systems(
                (
                    game_keys,
                    animate_sprite,
                    move_cursor,
                    flick_system,
                    change_colors,
                    timed_removal_system,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
            .add_system(teardown.in_schedule(OnExit(GameState::InGame)));
    }
}

use std::time::Duration;

use self::{
    components::{EnemySpawn, IdCounter, PhysicsSet},
    effects::{flick_system, timed_removal_system},
    systems::{
        animate_sprite_indices, animate_sprite_steps, change_colors, game_keys, move_cursor,
        move_missile, setup_cursor, spawn_enemy, teardown,
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
        app.add_systems((setup_cursor.in_schedule(OnEnter(GameState::InGame)),))
            .add_systems(
                (
                    game_keys,
                    animate_sprite_indices,
                    animate_sprite_steps,
                    move_cursor,
                    flick_system,
                    change_colors,
                    timed_removal_system,
                    move_missile,
                    spawn_enemy,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .configure_set(PhysicsSet::Movement.before(PhysicsSet::CollisionDetection))
            .add_system(teardown.in_schedule(OnExit(GameState::InGame)))
            .insert_resource(IdCounter(0))
            .insert_resource(EnemySpawn(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )));
    }
}

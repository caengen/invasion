use std::time::Duration;

use self::{
    components::{EnemySpawn, IdCounter, PhysicsSet},
    effects::{flick_system, timed_removal_system},
    systems::{
        animate_sprite_indices, animate_sprite_steps, change_colors, flame_engulf_system,
        game_keys, move_cursor, move_missile, setup_cursor, spawn_enemy, teardown,
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
        app.add_systems(OnEnter(GameState::InGame), setup_cursor)
            .add_systems(
                Update,
                (
                    game_keys,
                    animate_sprite_indices,
                    animate_sprite_steps,
                    move_cursor,
                    flick_system,
                    change_colors,
                    timed_removal_system,
                    move_missile,
                    flame_engulf_system.after(move_missile),
                    spawn_enemy,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .configure_set(
                Update,
                PhysicsSet::Movement.before(PhysicsSet::CollisionDetection),
            )
            .add_systems(OnExit(GameState::InGame), teardown)
            .insert_resource(IdCounter(0))
            .insert_resource(EnemySpawn(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )));
    }
}

use std::time::Duration;

use self::{
    components::{EnemySpawn, IdCounter, MissileArrivalEvent, PhysicsSet},
    effects::{flick_system, timed_removal_system},
    systems::{
        animate_sprite_indices, animate_sprite_steps, change_colors, flame_engulf_system,
        game_keys, game_over_ui, health_ui, missile_arrival_event_listner, move_cursor,
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
        app.add_event::<MissileArrivalEvent>()
            .add_systems(OnEnter(GameState::InGame), setup_cursor)
            .add_systems(
                Update,
                (
                    // always run these systems
                    (
                        move_cursor,
                        animate_sprite_indices,
                        animate_sprite_steps,
                        health_ui,
                    ),
                    // run these systems if we are in the InGame state
                    (
                        game_keys,
                        flick_system,
                        change_colors,
                        timed_removal_system,
                        move_missile,
                        missile_arrival_event_listner,
                        flame_engulf_system.after(move_missile),
                        spawn_enemy,
                    )
                        .run_if(in_state(GameState::InGame)),
                    // run these systems if we are in the GameOver state
                    (game_over_ui).run_if(in_state(GameState::GameOver)),
                ),
            )
            .add_systems(OnExit(GameState::InGame), teardown)
            .configure_set(
                Update,
                PhysicsSet::Movement.before(PhysicsSet::CollisionDetection),
            )
            .insert_resource(IdCounter(0))
            .insert_resource(EnemySpawn(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )));
    }
}

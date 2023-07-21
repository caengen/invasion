use std::time::Duration;

use self::{
    components::{EnemySpawn, ExplosionEvent, IdCounter, MissileArrivalEvent, PhysicsSet, Score},
    effects::{flick_system, timed_removal_system},
    prelude::stage_colors,
    systems::{
        animate_sprite_indices, animate_sprite_steps, change_colors,
        explosion_event_listener_system, explosion_system, flame_engulf_system, game_keys,
        game_over_ui, gizmo_missile_trails, health_ui, missile_arrival_event_listner, move_cursor,
        move_missile, move_ufo, reset_game_listener, rotate_player, score_ui, setup_player,
        spawn_enemies, teardown,
    },
};
use crate::GameState;
use bevy::prelude::*;

mod collision;
mod components;
mod effects;
pub mod prelude;
mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MissileArrivalEvent>()
            .add_event::<ExplosionEvent>()
            .add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(
                Update,
                (
                    // always run these systems
                    (
                        move_cursor,
                        animate_sprite_steps,
                        animate_sprite_indices,
                        score_ui,
                    )
                        .run_if(in_state(GameState::InGame).or_else(in_state(GameState::GameOver))),
                    // run these systems if we are in the InGame state
                    (
                        game_keys,
                        flick_system,
                        change_colors,
                        (
                            spawn_enemies,
                            move_missile,
                            gizmo_missile_trails,
                            move_ufo,
                            timed_removal_system,
                            missile_arrival_event_listner,
                            explosion_event_listener_system,
                            explosion_system,
                            flame_engulf_system,
                        )
                            .chain(),
                        health_ui,
                        rotate_player,
                        stage_colors.after(spawn_enemies),
                    )
                        .run_if(in_state(GameState::InGame)),
                    // run these systems if we are in the GameOver state
                    (game_over_ui, reset_game_listener).run_if(in_state(GameState::GameOver)),
                ),
            )
            .add_systems(OnExit(GameState::InGame), teardown)
            .configure_set(
                Update,
                PhysicsSet::Movement.before(PhysicsSet::CollisionDetection),
            )
            .insert_resource(IdCounter(0))
            .insert_resource(Score(0))
            .insert_resource(EnemySpawn(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )));
    }
}

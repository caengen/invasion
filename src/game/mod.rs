use std::time::Duration;

use self::{
    components::{
        ExplosionEvent, IdCounter, MissileArrivalEvent, PhysicsSet, Score, TankDestroyedEvent,
    },
    effects::{flick_system, timed_removal_system},
    prelude::stage_colors,
    systems::{
        ammo_ui, animate_sprite_indices, animate_sprite_steps, change_colors, defeat, despawns,
        drop_bombs, explode_city, explosion_event_listener_system, explosion_system,
        flame_engulf_system, game_keys, game_over_ui, gizmo_missile_trails, is_wave_finished,
        missile_arrival_event_listner, move_cursor, move_missile, move_ufo, player_destruction,
        reset_game_listener, rotate_player, score_ui, setup_player, spawn_enemies, split_missiles,
        teardown_game_over, teardown_in_game, tick_wave_completion, wave_complete,
        wave_complete_message_ui, wave_ui,
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
            .add_event::<TankDestroyedEvent>()
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
                        ammo_ui,
                        wave_ui,
                        tick_wave_completion,
                        wave_complete_message_ui,
                    )
                        .run_if(in_state(GameState::InGame).or_else(in_state(GameState::GameOver))),
                    // run these systems if we are in the InGame state
                    (
                        game_keys,
                        flick_system,
                        change_colors,
                        (
                            spawn_enemies.run_if(
                                in_state(GameState::InGame).and_then(not(is_wave_finished)),
                            ),
                            split_missiles,
                            move_missile,
                            gizmo_missile_trails,
                            move_ufo,
                            drop_bombs,
                            timed_removal_system,
                            missile_arrival_event_listner,
                            explosion_event_listener_system,
                            explosion_system,
                            flame_engulf_system,
                            player_destruction,
                            explode_city,
                            despawns,
                        )
                            .chain(),
                        rotate_player,
                        defeat,
                        stage_colors.after(spawn_enemies),
                        (wave_complete)
                            .run_if(in_state(GameState::InGame).and_then(is_wave_finished)),
                    )
                        .run_if(in_state(GameState::InGame)),
                    // run these systems if we are in the GameOver state
                    (
                        game_over_ui,
                        reset_game_listener,
                        move_missile,
                        explosion_event_listener_system,
                        explosion_system,
                    )
                        .run_if(in_state(GameState::GameOver)),
                ),
            )
            .add_systems(OnExit(GameState::InGame), teardown_in_game)
            .add_systems(OnExit(GameState::GameOver), teardown_game_over)
            .configure_set(
                Update,
                PhysicsSet::Movement.before(PhysicsSet::CollisionDetection),
            )
            .insert_resource(IdCounter(0))
            .insert_resource(Score(0));
    }
}

use bevy::{ecs::query::Has, math::vec2, prelude::*};
use bevy_egui::{
    egui::{self, Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, RichText},
    EguiContexts,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{GameState, ImageAssets, MainCamera, SCREEN};

use super::{
    components::{
        AnimationIndices, AnimationStep, Cursor, Enemy, EnemySpawn, Engulfable, Explosion,
        FlameRadius, Health, IdCounter, Missile, MissileArrivalEvent, MissileExplosionEvent,
        Player, Score, Scoring, Stepper, TargetLock, Ufo,
    },
    effects::{Flick, TimedRemoval},
};

pub fn game_keys(
    buttons: Res<Input<MouseButton>>,
    cursor_pos: Query<&Transform, With<Cursor>>,
    mut id_counter: ResMut<IdCounter>,
    mut commands: Commands,
    images: Res<ImageAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let transform = cursor_pos.single();
        let id = id_counter.next();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: *transform,
                ..default()
            },
            TargetLock(id),
            Flick {
                duration: Timer::from_seconds(3.0, TimerMode::Repeating),
                switch_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            TimedRemoval(Timer::from_seconds(3.0, TimerMode::Once)),
        ));

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(3),
                transform: Transform::from_translation(Vec3::new(0.0, -SCREEN.y / 2.0, 1.0)),
                ..default()
            },
            Missile {
                dest: transform.translation.truncate(),
                lock_id: id,
                vel: 250.0,
            },
        ));
    }
}

pub fn spawn_enemy_missile(
    mut id_counter: ResMut<IdCounter>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut global_rng: ResMut<GlobalRng>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    time: Res<Time>,
) {
    enemy_spawn.0.tick(time.delta());

    if !enemy_spawn.0.just_finished() {
        return;
    }

    enemy_spawn.0.reset();
    let mut rng = RngComponent::from(&mut global_rng);

    // spawn ufo
    if rng.chance(0.20) {
        let origin_y = rng.usize((-SCREEN.y as usize / 3)..(SCREEN.y as usize - 20)) as f32;
        let sign = if rng.bool() { 1.0 } else { -1.0 };
        let origin_x = sign * SCREEN.x;

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(9),
                transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 1.0)),
                ..default()
            },
            AnimationIndices {
                first: 9,
                last: 12,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            Ufo(vec2(-origin_x, origin_y)),
            Engulfable,
            Enemy,
        ));

        return;
    }
    // lag random position fra topp med random dest
    // fn ticker hvert sekund. Opprett en strek
    // kan ikke tegne strek fordi eg har ikke polyogon rammeverket........
    let origin_x = rng.usize(-SCREEN.x as usize..SCREEN.x as usize) as f32;
    let sign = if rng.bool() { 1.0 } else { -1.0 };
    let mut dest_x = sign * rng.usize(0..(SCREEN.x / 6.0) as usize) as f32;
    if dest_x < -SCREEN.x || dest_x > SCREEN.x {
        dest_x *= -1.0;
    }
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cursor.clone(),
            sprite: TextureAtlasSprite::new(3),
            transform: Transform::from_translation(Vec3::new(origin_x, SCREEN.y / 2.0, 1.0)),
            ..default()
        },
        Missile {
            dest: Vec2::new(dest_x, -SCREEN.y / 2.0),
            lock_id: id_counter.next(),
            vel: 35.0,
        },
        Engulfable,
        Enemy,
    ));
}

pub fn change_colors(mut query: Query<&mut Sprite, With<TargetLock>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(1.0, 0.0, 0.0);
    }
}

pub fn explosion_system(
    mut commands: Commands,
    mut explosion_event: EventReader<MissileExplosionEvent>,
    images: Res<ImageAssets>,
    missiles: Query<(Entity, &Transform), With<Missile>>,
) {
    for MissileExplosionEvent { entity } in explosion_event.iter() {
        for missile in missiles.iter() {
            if missile.0 == *entity {
                commands.entity(missile.0).despawn();

                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: images.explosion.clone(),
                        sprite: TextureAtlasSprite::new(0),
                        transform: Transform {
                            translation: missile.1.translation,
                            // scale: Vec3::splat(3.0),
                            ..default()
                        },
                        ..default()
                    },
                    Stepper {
                        marker: AnimationStep,
                        current: 0,
                        steps: Vec::from([0, 1, 2, 3, 3, 3, 2, 2, 1, 0]),
                        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    },
                    Stepper {
                        marker: FlameRadius {},
                        current: 0,
                        steps: Vec::from([2, 8, 12, 16, 16, 16, 12, 12, 8, 2]),
                        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    },
                    Explosion::new(),
                ));
            }
        }
    }
}

pub fn missile_arrival_event_listner(
    mut commands: Commands,
    mut missile_expl_evnt: EventReader<MissileArrivalEvent>,
    images: Res<ImageAssets>,
    mut player_health: Query<&mut Health, With<Player>>,
    target_locks: Query<(Entity, &TargetLock, &Transform), Without<Missile>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut explosion_event: EventWriter<MissileExplosionEvent>,
) {
    for MissileArrivalEvent {
        entity: id,
        missile,
        is_enemy,
    } in missile_expl_evnt.iter()
    {
        // Despawn target lock
        let lock = target_locks
            .iter()
            .find(|(_, lock, _)| lock.0 == missile.lock_id);
        if let Some((entity, _, _)) = lock {
            commands.entity(entity).despawn();
        }

        // Spawn explosion
        explosion_event.send(MissileExplosionEvent { entity: *id });

        // Damage player
        if *is_enemy && missile.dest.y <= -(SCREEN.y / 2.0).ceil() {
            let mut health = player_health.single_mut();
            if health.current > 0 {
                health.current -= 1;
            }

            // should be moved to a separate system
            if health.current == 0 {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn move_ufo(mut commands: Commands, mut ufos: Query<(Entity, &Ufo, &mut Transform)>) {
    for (entity, ufo, mut transform) in ufos.iter_mut() {
        let direction = ufo.0 - transform.translation.truncate();
        let distance = direction.length();
        let velocity = direction.normalize() * 10.0;
        let translation = velocity * 0.01;
        if distance > translation.length() {
            // move the ufo
            transform.translation += translation.extend(0.0);
        } else {
            // ufo has arrived at target
            // despawn ufo
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_missile(
    mut missiles: Query<(Entity, &Missile, &mut Transform, Has<Enemy>), Without<TargetLock>>,
    mut missile_arrival_evnt: EventWriter<MissileArrivalEvent>,
    time: Res<Time>,
) {
    for (entity, missile, mut transform, is_enemy) in missiles.iter_mut() {
        let direction = missile.dest - transform.translation.truncate();
        let distance = direction.length();
        let velocity = direction.normalize() * missile.vel;
        let translation = velocity * time.delta_seconds();
        if distance > translation.length() {
            // move the missile
            transform.translation += translation.extend(0.0);
        } else {
            // missile has arrived at target
            missile_arrival_evnt.send(MissileArrivalEvent {
                entity,
                missile: missile.clone(),
                is_enemy,
            });
        }
    }
}

pub fn move_cursor(
    mut cursor: Query<(&mut Transform, &Cursor)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        for (mut transform, _) in cursor.iter_mut() {
            transform.translation = world_position.extend(1.0);
        }
    }
}

pub fn rotate_player(
    mut player: Query<&mut Transform, (With<Player>, Without<Cursor>)>,
    cursor: Query<&Transform, (With<Cursor>, Without<Player>)>,
) {
    for mut transform in player.iter_mut() {
        for cursor in cursor.iter() {
            let direction = cursor.translation.truncate() - transform.translation.truncate();
            let angle = direction.y.atan2(direction.x) - 90.0_f32.to_radians();
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn reset_game_listener(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        next_state.set(GameState::InGame);
    }
}

pub fn animate_sprite_indices(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut TextureAtlasSprite)>,
) {
    for (mut indices, mut sprite) in &mut query {
        indices.timer.tick(time.delta());
        if indices.timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn animate_sprite_steps(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Stepper<AnimationStep, i32>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut stepper, mut sprite) in &mut query {
        stepper.timer.tick(time.delta());
        if stepper.timer.just_finished() {
            // todo: really really out of place
            if stepper.is_finished() {
                commands.entity(entity).despawn();
            } else {
                let index = stepper.next();
                if let Some(index) = index {
                    sprite.index = *index as usize;
                }
            }
        }
    }
}

pub fn flame_engulf_system(
    mut commands: Commands,
    time: Res<Time>,
    mut flames: Query<(
        Entity,
        &Transform,
        &mut Stepper<FlameRadius, i32>,
        &mut Explosion,
        Without<Engulfable>,
    )>,
    mut engulfables: Query<(Entity, &Transform, With<Engulfable>, Has<Missile>)>,
    mut score: ResMut<Score>,
    mut explosion_event: EventWriter<MissileExplosionEvent>,
) {
    for (flame_entity, flame_transform, mut stepper, mut expl, _) in flames.iter_mut() {
        stepper.timer.tick(time.delta());
        if stepper.timer.just_finished() {
            if stepper.is_finished() {
                score.0 += expl.calculated_score();
                commands
                    .entity(flame_entity)
                    .remove::<Stepper<FlameRadius, i32>>();
            } else {
                if let Some(radius) = stepper.next() {
                    for (entity, transform, _, is_missile) in engulfables.iter_mut() {
                        let distance = flame_transform.translation.distance(transform.translation);
                        if distance <= *radius as f32 {
                            if is_missile {
                                explosion_event.send(MissileExplosionEvent { entity });
                                expl.add_score(Scoring::Missile);
                            } else {
                                // is ufo, more points
                                //todo: fix this later
                                commands.entity(entity).despawn();
                                expl.add_score(Scoring::Ufo);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn teardown(
    mut commands: Commands,
    missiles: Query<Entity, (With<Missile>, Without<Player>)>,
    player: Query<Entity, With<Player>>,
) {
    for missile in missiles.iter() {
        commands.entity(missile).despawn_recursive();
    }
    for player in player.iter() {
        commands.entity(player).despawn_recursive();
    }
}

/* Setup
 * Systems that are called once at the start of the game
 */
pub fn setup_fonts(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "visitor".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/visitor.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "visitor".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("visitor".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

pub fn setup_player(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cursor.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Cursor {},
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cannon.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, -SCREEN.y / 2.0, 1.0)),
            ..default()
        },
        Player {},
        Health { max: 3, current: 3 },
    ));
}

/* UI
 * Systems that are called every frame to update the egui UI
 */
pub fn score_ui(mut contexts: EguiContexts, score: Res<Score>) {
    egui::Area::new("Score")
        .anchor(Align2::LEFT_TOP, egui::emath::vec2(10., 5.))
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label(
                    RichText::new(format!("{:0>7}", score.0))
                        .font(FontId::proportional(20.))
                        .color(Color32::WHITE),
                );
            });
        });
}

pub fn health_ui(
    images: Res<ImageAssets>,
    player: Query<(&Health, With<Player>)>,
    mut contexts: EguiContexts,
) {
    let heart_image_id = contexts.add_image(images.heart_full.clone_weak());
    let heart_empty_image_id = contexts.add_image(images.heart_empty.clone_weak());

    egui::Area::new("Health")
        .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-10., 5.))
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let (health, _) = player.single();
                for _ in 0..health.current {
                    ui.image(heart_image_id, egui::emath::vec2(8., 8.));
                }
                for _ in 0..(health.max - health.current) {
                    ui.image(heart_empty_image_id, egui::emath::vec2(8., 8.));
                }
            });
        });
}

pub fn game_over_ui(mut contexts: EguiContexts) {
    egui::Area::new("gameover")
        .anchor(Align2::CENTER_CENTER, egui::emath::vec2(0., 0.))
        .show(contexts.ctx_mut(), |ui| ui.label("Game Over!"));
}

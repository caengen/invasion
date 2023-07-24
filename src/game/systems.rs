use std::ops::Add;

use bevy::{core_pipeline::clear_color::ClearColorConfig, ecs::query::Has, math::vec2, prelude::*};
use bevy_egui::{
    egui::{self, Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, RichText},
    EguiContexts,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{GameState, ImageAssets, MainCamera, SCREEN};

use super::{
    components::{
        AnimationIndices, AnimationStep, AnimeRemoveOnFinish, Cannon, CannonBase, ChainedMeta,
        City, Cursor, Destroyed, DropBombTimer, Enemy, Engulfable, Explodable, Explosion,
        ExplosionEvent, ExplosionMode, FlameRadius, Foreground, Health, IdCounter, Missile,
        MissileArrivalEvent, MissileReserve, Player, Score, Scoring, SpawnPoint, Stepper,
        TargetLock, Ufo, MAX_AMMO, PLAYER_MISSILE_SPEED,
    },
    effects::{Flick, TimedRemoval},
    prelude::{color_from_vec, EnemySpawn, SplitTimer, Stage, StageHandle, Wave, WaveSpawnCount},
};

pub fn game_keys(
    buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    cursor_pos: Query<&Transform, (With<Cursor>, Without<CannonBase>)>,
    mut id_counter: ResMut<IdCounter>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut player: Query<((Entity, &mut MissileReserve), With<Player>)>,
    mut cannon_base: Query<
        (Entity, &mut Transform, Has<AnimationIndices>),
        (With<CannonBase>, Without<Cursor>),
    >,
    time: Res<Time>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (player_entity, mut missile_reserve) = player.single_mut().0;
        commands.entity(player_entity).insert((
            AnimationIndices {
                first: 0,
                last: 3,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            AnimeRemoveOnFinish,
        ));

        if missile_reserve.0 == 0 {
            return;
        } else {
            missile_reserve.0 -= 1;
        }

        let target_transform = cursor_pos.single();
        let id = id_counter.next();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: *target_transform,
                ..default()
            },
            TargetLock(id),
            Flick {
                duration: Timer::from_seconds(3.0, TimerMode::Repeating),
                switch_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            TimedRemoval(Timer::from_seconds(3.0, TimerMode::Once)),
        ));

        let (_, base_transform, _) = cannon_base.single();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(3),
                transform: *base_transform,
                ..default()
            },
            Missile {
                dest: target_transform.translation.truncate(),
                lock_id: id,
                vel: PLAYER_MISSILE_SPEED,
            },
            Explodable,
        ));
    }

    let (entity, mut transform, has_anim) = cannon_base.single_mut();
    if keyboard.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::Right, KeyCode::D]) {
        if !has_anim {
            commands.entity(entity).insert((AnimationIndices {
                first: 0,
                last: 1,
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            },));
        }
        if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
            transform.translation.x -= 50.0 * time.delta_seconds();
        }
        if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {
            transform.translation.x += 50.0 * time.delta_seconds();
        }
    } else if has_anim {
        commands.entity(entity).remove::<AnimationIndices>();
    }
}

// temp gizmo
pub fn gizmo_missile_trails(
    mut gizmos: Gizmos,
    missiles: Query<(&Transform, &SpawnPoint), (With<Missile>, With<Enemy>)>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
) {
    let stage = stages.get(&stage.0).unwrap();

    for (transform, spawn_point) in missiles.iter() {
        gizmos.line_2d(
            spawn_point.0,
            transform.translation.truncate(),
            color_from_vec(&stage.trail_cor),
        );
    }
}

pub fn drop_bombs(
    mut commands: Commands,
    mut ufos: Query<(Entity, &Transform, &mut DropBombTimer), (With<Ufo>, With<Enemy>)>,
    mut id_counter: ResMut<IdCounter>,
    images: Res<ImageAssets>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
    wave: Res<Wave>,
) {
    let mut rng = RngComponent::from(&mut global_rng);
    let stage = stages.get(&stage.0).unwrap();
    for (entity, transform, mut timer) in ufos.iter_mut() {
        timer.0.tick(time.delta());

        if !timer.0.just_finished() || !rng.chance(stage.drop_bomb_chance(wave.0)) {
            continue;
        }

        commands.entity(entity).remove::<DropBombTimer>();
        spawner::missile(
            &mut commands,
            &mut rng,
            &mut id_counter,
            images.cursor.clone(),
            &stage,
            Some(transform.translation.truncate()),
        );
    }
}

pub fn split_missiles(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), (With<Missile>, With<Enemy>)>,
    mut id_counter: ResMut<IdCounter>,
    images: Res<ImageAssets>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
    mut split_timer: ResMut<SplitTimer>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
    wave: Res<Wave>,
) {
    split_timer.0.tick(time.delta());

    if !split_timer.0.just_finished() && query.iter().len() > 0 {
        return;
    }
    let mut rng = RngComponent::from(&mut global_rng);
    let stage = stages.get(&stage.0).unwrap();

    if !rng.chance(stage.split_chance(wave.0)) {
        return;
    }
    let index = rng.usize(0..query.iter().len());
    let res = query.iter_mut().nth(index);

    if let Some((entity, transform)) = res {
        commands.entity(entity).despawn();
        for _ in 0..stage.max_split(wave.0) {
            spawner::missile(
                &mut commands,
                &mut rng,
                &mut id_counter,
                images.cursor.clone(),
                &stage,
                Some(transform.translation.truncate()),
            );
        }
    }
}

pub fn is_wave_finished(
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
    wave: Res<Wave>,
    spawn_count: Res<WaveSpawnCount>,
) -> bool {
    let stage = stages.get(&stage.0).unwrap();
    stage.enemies_count(wave.0) <= spawn_count.0
}

pub fn wave_complete(
    mut wave: ResMut<Wave>,
    mut spawn_count: ResMut<WaveSpawnCount>,
    mut missile_ammo: Query<&mut MissileReserve, With<Player>>,
) {
    wave.0 += 1;
    spawn_count.0 = 0;
    for mut ammo in missile_ammo.iter_mut() {
        ammo.0 = MAX_AMMO;
    }
}

pub fn spawn_enemies(
    mut id_counter: ResMut<IdCounter>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut global_rng: ResMut<GlobalRng>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    time: Res<Time>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
    wave: Res<Wave>,
    mut spawn_count: ResMut<WaveSpawnCount>,
) {
    enemy_spawn.0.tick(time.delta());

    if !enemy_spawn.0.just_finished() {
        return;
    }

    enemy_spawn.0.reset();

    let stage = stages.get(&stage.0).unwrap();
    let mut rng = RngComponent::from(&mut global_rng);

    // spawn ufo
    if rng.chance(stage.ufo_chance(wave.0)) {
        spawner::ufo(&mut commands, &mut rng, images.cursor.clone());
        spawn_count.0 += 1;
    }

    for _ in 0..=rng.usize(stage.missile_spawn_min(wave.0)..stage.missile_spawn_max(wave.0)) {
        spawner::missile(
            &mut commands,
            &mut rng,
            &mut id_counter,
            images.cursor.clone(),
            &stage,
            None,
        );
        spawn_count.0 += 1;
    }
}

pub fn change_colors(mut query: Query<&mut Sprite, With<TargetLock>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(1.0, 0.0, 0.0);
    }
}

pub fn move_ufo(
    mut commands: Commands,
    mut ufos: Query<(Entity, &Ufo, &mut Transform)>,
    time: Res<Time>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
    wave: Res<Wave>,
) {
    let stage = stages.get(&stage.0).unwrap();
    for (entity, ufo, mut transform) in ufos.iter_mut() {
        let dir = ufo.0 - transform.translation.truncate();
        let dist = dir.length();
        let translation = dir.normalize() * stage.ufo_speed(wave.0) * time.delta_seconds();
        if dist > translation.length() {
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
    cannon_base: Query<&Transform, (With<CannonBase>, Without<Cursor>, Without<Cannon>)>,
    mut cannon: Query<&mut Transform, (With<Cannon>, Without<Cursor>, Without<CannonBase>)>,
    cursor: Query<&Transform, (With<Cursor>, Without<CannonBase>, Without<Cannon>)>,
) {
    for transform in cannon_base.iter() {
        for cursor in cursor.iter() {
            // let direction = cursor.translation.truncate() - transform.translation.truncate();
            let a = transform.translation.truncate();
            let b = cursor.translation.truncate();
            let direction = b - a;
            let angle = direction.y.atan2(direction.x) - 90.0_f32.to_radians();
            for mut transform in cannon.iter_mut() {
                transform.rotation = Quat::from_rotation_z(angle);
            }
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
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimationIndices,
        &mut TextureAtlasSprite,
        Has<AnimeRemoveOnFinish>,
    )>,
) {
    for (entity, mut indices, mut sprite, has_boe) in &mut query {
        indices.timer.tick(time.delta());
        if indices.timer.just_finished() {
            if sprite.index == indices.last {
                sprite.index = indices.first;
                if has_boe {
                    commands.entity(entity).remove::<AnimationIndices>();
                }
            } else {
                sprite.index = sprite.index + 1;
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

pub fn explosion_system(
    mut explosions: Query<(&Transform, &mut Explosion)>,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    let mut rng = RngComponent::from(&mut global_rng);

    for (transform, mut explosion) in explosions.iter_mut() {
        match explosion.mode {
            ExplosionMode::Chained(ref mut meta) => {
                if meta.remaining == 0 {
                    return;
                }

                meta.timer.tick(time.delta());

                if meta.timer.just_finished() {
                    let next_pos = transform.translation
                        + Vec3::new(rng.i32(-15..15) as f32, rng.i32(-15..15) as f32, 0.0);
                    explosion_event.send(ExplosionEvent {
                        pos: next_pos,
                        mode: ExplosionMode::Chained(ChainedMeta {
                            remaining: meta.remaining - 1,
                            timer: meta.timer.clone(),
                        }),
                    });

                    // need to prevent the same explosion to be handled multiple times
                    explosion.mode = ExplosionMode::Single;
                }
            }
            _ => {}
        }
    }
}

pub fn explosion_event_listener_system(
    mut commands: Commands,
    mut explosion_event: EventReader<ExplosionEvent>,
    images: Res<ImageAssets>,
) {
    for ExplosionEvent { pos, mode } in explosion_event.iter() {
        let explosion_mode = match mode {
            ExplosionMode::Single => mode.clone(),
            ExplosionMode::Chained(meta) => ExplosionMode::Chained(ChainedMeta {
                timer: meta.timer.clone(),
                remaining: meta.remaining - 1,
            }),
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.explosion.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: *pos,
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
            Explosion::new(explosion_mode),
            Foreground,
        ));
    }
}

pub fn explode_city(
    mut commands: Commands,
    mut cities: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<City>>,
    flames: Query<(&Transform, &Stepper<FlameRadius, i32>), With<Explosion>>,
) {
    for (city_entity, city_transform, mut city_sprite) in cities.iter_mut() {
        for (flame_transform, flame_stepper) in flames.iter() {
            let distance = city_transform
                .translation
                .truncate()
                .distance(flame_transform.translation.truncate());
            if distance < flame_stepper.current as f32 + 16.0 {
                city_sprite.index = 1;
                commands.entity(city_entity).insert(Destroyed);
            }
        }
    }
}

pub fn defeat(
    mut next_state: ResMut<NextState<GameState>>,
    cities: Query<(&Transform, &mut TextureAtlasSprite), (With<City>, Without<Destroyed>)>,
) {
    if cities.iter().count() == 0 {
        next_state.set(GameState::GameOver);
    }
}

pub fn missile_arrival_event_listner(
    mut commands: Commands,
    mut missile_expl_evnt: EventReader<MissileArrivalEvent>,
    target_locks: Query<(Entity, &TargetLock, &Transform), Without<Missile>>,
    mut explosion_event: EventWriter<ExplosionEvent>,
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
        explosion_event.send(ExplosionEvent {
            pos: missile.dest.extend(1.0),
            mode: ExplosionMode::Single,
        });
        commands.entity(*id).despawn();

        // // Damage player
        // if *is_enemy {
        //     let mut health = player_health.single_mut();
        //     if health.current > 0 {
        //         health.current -= 1;
        //     }

        //     // should be moved to a separate system
        //     if health.current == 0 {
        //         next_state.set(GameState::GameOver);
        //     }
        // }
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
    mut explosion_event: EventWriter<ExplosionEvent>,
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
                        if distance > *radius as f32 {
                            continue;
                        }

                        if is_missile {
                            explosion_event.send(ExplosionEvent {
                                pos: transform.translation,
                                mode: ExplosionMode::Single,
                            });
                            commands.entity(entity).despawn();
                            expl.add_score(Scoring::Missile);
                        } else {
                            // is ufo, more points
                            explosion_event.send(ExplosionEvent {
                                pos: transform.translation,
                                mode: ExplosionMode::Chained(ChainedMeta {
                                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                    remaining: 5,
                                }),
                            });
                            commands.entity(entity).despawn();
                            expl.add_score(Scoring::Ufo);
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
    cursor: Query<Entity, With<Cursor>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    // maybe just add some teardown components...
    // TeardownLevel, TeardownLoss... 🤔
    for missile in missiles.iter() {
        commands.entity(missile).despawn_recursive();
    }
    for player in player.iter() {
        commands.entity(player).despawn_recursive();
    }
    for cursor in cursor.iter() {
        commands.entity(cursor).despawn_recursive();
    }
    for enemy in enemies.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}

/* Setup
 * Systems that are called once at the start of the game
 */

pub fn setup_player(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.ground.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, -SCREEN.y / 2.0 + 16.0, 1.0)),
            ..default()
        },
        Foreground,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cursor.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Cursor,
        Foreground,
    ));

    let cannon = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: images.cannon.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_translation(Vec3::new(0.0, -2.0, 1.0)),
                ..default()
            },
            Player,
            Cannon,
            Health { max: 3, current: 3 },
            MissileReserve(MAX_AMMO),
            Foreground,
        ))
        .id();

    let mut tank = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.tank.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, -SCREEN.y / 2.0 + 24.0, 2.0)),
            ..default()
        },
        CannonBase,
        Foreground,
    ));

    tank.add_child(cannon);

    for i in 0..6 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.city.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_translation(Vec3::new(
                    -SCREEN.x / 2.0 + 41.5 + (82.5 * i as f32),
                    -SCREEN.y / 2.0 + 32.0,
                    1.0,
                )),
                ..default()
            },
            City,
            Foreground,
        ));
    }
}

/* UI
 * Systems that are called every frame to update the egui UI
 */
pub fn ammo_ui(
    mut contexts: EguiContexts,
    images: Res<ImageAssets>,
    missile_ammo: Query<&MissileReserve, With<Player>>,
) {
    if let Ok(ammo) = missile_ammo.get_single() {
        let ammo_id = contexts.add_image(images.missile.clone_weak());

        egui::Area::new("Ammo")
            .anchor(Align2::LEFT_TOP, egui::emath::vec2(10., 5.))
            .show(contexts.ctx_mut(), |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.image(ammo_id, egui::emath::vec2(16., 16.));
                    ui.label(
                        RichText::new(format!("{:0>2}", ammo.0))
                            .font(FontId::proportional(24.))
                            .color(Color32::WHITE),
                    );
                });
            });
    }
}

pub fn wave_ui(mut contexts: EguiContexts, wave: Res<Wave>) {
    egui::Area::new("Wave")
        .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-50., 5.))
        .show(contexts.ctx_mut(), |ui: &mut egui::Ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(
                    RichText::new(format!("{:0>2}", wave.0 + 1))
                        .font(FontId::proportional(24.))
                        .color(Color32::WHITE),
                );
            });
        });
}

pub fn score_ui(mut contexts: EguiContexts, score: Res<Score>) {
    egui::Area::new("Score")
        .anchor(Align2::CENTER_TOP, egui::emath::vec2(10., 5.))
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label(
                    RichText::new(format!("{:0>7}", score.0))
                        .font(FontId::proportional(24.))
                        .color(Color32::WHITE),
                );
            });
        });
}

pub fn game_over_ui(mut contexts: EguiContexts) {
    egui::Area::new("gameover")
        .anchor(Align2::CENTER_CENTER, egui::emath::vec2(0., 0.))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(
                RichText::new("GAME OVER")
                    .font(FontId::proportional(24.))
                    .color(Color32::WHITE),
            )
        });
}

mod spawner {
    use bevy::{
        math::{vec2, vec3},
        prelude::*,
        sprite::{SpriteSheetBundle, TextureAtlasSprite},
    };
    use bevy_prototype_lyon::{
        prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke},
        shapes,
    };
    use bevy_turborand::{DelegatedRng, RngComponent};

    use crate::{
        game::{
            components::{
                AnimationIndices, DropBombTimer, Enemy, Engulfable, Explodable, Foreground,
                IdCounter, Missile, SpawnPoint, Ufo,
            },
            prelude::Stage,
        },
        SCREEN,
    };

    use super::color_from_vec;

    pub fn ufo(commands: &mut Commands, rng: &mut RngComponent, images: Handle<TextureAtlas>) {
        let origin_y = rng.i32((-(SCREEN.y / 3.0) as i32)..((SCREEN.y / 2.0) as i32 - 30)) as f32;
        let sign = if rng.bool() { 1.0 } else { -1.0 };
        let origin_x = sign * (SCREEN.x / 2.0);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images,
                sprite: TextureAtlasSprite::new(8),
                transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 1.0)),
                ..default()
            },
            AnimationIndices {
                first: 8,
                last: 11,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            Ufo(vec2(-origin_x, origin_y)),
            Explodable,
            Engulfable,
            Enemy,
            Foreground,
            DropBombTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        ));
    }

    pub fn missile(
        commands: &mut Commands,
        rng: &mut RngComponent,
        id_counter: &mut ResMut<IdCounter>,
        images: Handle<TextureAtlas>,
        stage: &Stage,
        origin: Option<Vec2>,
    ) {
        let origin = origin.unwrap_or_else(|| {
            let x = rng.i32(-(SCREEN.x / 2.0) as i32..(SCREEN.x / 2.0) as i32) as f32;
            let y = SCREEN.y / 2.0;
            vec2(x, y)
        });

        let sign = if rng.bool() { 1.0 } else { -1.0 };
        let mut dest_x = sign * rng.usize(0..(SCREEN.x / 4.0) as usize) as f32;
        if dest_x < -SCREEN.x || dest_x > SCREEN.x {
            dest_x *= -1.0;
        }
        let parent = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: images,
                    sprite: TextureAtlasSprite::new(3),
                    transform: Transform::from_translation(Vec3::new(origin.x, origin.y, 1.0)),
                    ..default()
                },
                Missile {
                    dest: Vec2::new(dest_x, -SCREEN.y / 2.0 + 16.0),
                    lock_id: id_counter.next(),
                    vel: stage.missile_speed,
                },
                Explodable,
                Engulfable,
                SpawnPoint(origin),
                Enemy,
                Foreground,
            ))
            .id();

        let shape = shapes::Line(vec2(origin.x, origin.y), vec2(dest_x, -SCREEN.y / 2.0));
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform {
                    translation: vec3(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..default()
            },
            Fill::color(color_from_vec(&stage.trail_cor)),
        ));
    }
}

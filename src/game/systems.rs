use bevy::{prelude::*, transform};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use std::time::Duration;

use crate::{game::components::AnimationSteps, ImageAssets, MainCamera, SCREEN};

use super::{
    components::{
        AnimationIndices, AnimationTimer, Cursor, EnemySpawn, Explosion, IdCounter, Missile,
        TargetLock,
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
                vel: 500.0,
            },
        ));
    }
}

pub fn spawn_enemy(
    mut id_counter: ResMut<IdCounter>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut global_rng: ResMut<GlobalRng>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    time: Res<Time>,
) {
    enemy_spawn.0.tick(time.delta());

    if (enemy_spawn.0.just_finished()) {
        enemy_spawn.0.reset();
        let mut rng = RngComponent::from(&mut global_rng);
        // lag random position fra topp med random dest
        // fn ticker hvert sekund. Opprett en strek
        // faen kan ikke tegne strek fordi eg har ikke polyogon rammeverket........
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
                vel: 10.0,
            },
        ));
    }
}

pub fn change_colors(mut query: Query<(&mut Sprite), (With<TargetLock>)>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(1.0, 0.0, 0.0);
    }
}

pub fn setup_cursor(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cursor.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Cursor {},
    ));
}

pub fn move_missile(
    mut commands: Commands,
    mut missiles: Query<(Entity, &Missile, &mut Transform), Without<TargetLock>>,
    target_locks: Query<(Entity, &TargetLock, &Transform), Without<Missile>>,
    time: Res<Time>,
    images: Res<ImageAssets>,
) {
    for (entity, missile, mut transform) in missiles.iter_mut() {
        let direction = missile.dest - transform.translation.truncate();
        let distance = direction.length();
        let velocity = direction.normalize() * missile.vel;
        let translation = velocity * time.delta_seconds();
        if distance > translation.length() {
            transform.translation += translation.extend(0.0);
        } else {
            commands.entity(entity).despawn();
            let lock = target_locks
                .iter()
                .find(|(_, lock, _)| lock.0 == missile.lock_id);
            if let Some((entity, _, _)) = lock {
                commands.entity(entity).despawn();
            }

            // move this spawn to an event
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: images.cursor.clone(),
                    sprite: TextureAtlasSprite::new(3),
                    transform: Transform {
                        translation: missile.dest.extend(1.0),
                        scale: Vec3::splat(3.0),
                        ..default()
                    },
                    ..default()
                },
                AnimationSteps {
                    current: 0,
                    steps: Vec::from([3, 4, 5, 6, 7, 6, 5, 4, 3]),
                },
                AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
                Explosion,
            ));
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

pub fn teardown(mut commands: Commands) {
    // for (entity, _) in texts.iter() {
    //     commands.entity(entity).despawn();
    // }
}

pub fn animate_sprite_indices(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
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
        &mut AnimationSteps,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut steps, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            // todo: really really out of place
            if steps.is_finished() {
                commands.entity(entity).despawn();
            } else {
                let index = steps.next();
                if let Some(index) = index {
                    sprite.index = index;
                }
            }
        }
    }
}

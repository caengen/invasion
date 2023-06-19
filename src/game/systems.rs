use bevy::prelude::*;
use std::time::Duration;

use crate::{ImageAssets, MainCamera};

use super::components::{AnimationIndices, AnimationTimer, Cursor};

pub fn game_keys(
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<(
        &Cursor,
        &mut Transform,
        &mut AnimationIndices,
        &mut TextureAtlasSprite,
        &mut AnimationTimer,
    )>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }
    if keyboard.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.0;
    }

    let move_speed = 1.0;
    let move_delta = (direction * move_speed).extend(0.0);

    for (_, mut transform, mut indices, mut sprite, mut timer) in player.iter_mut() {
        if direction == Vec2::ZERO {
            // update animation
            indices.first = 0;
            indices.last = 1;
            sprite.index = usize::clamp(sprite.index, indices.first, indices.last);
            timer.0.set_duration(Duration::from_millis(500));
            continue;
        }

        transform.translation += move_delta;

        // update animation
        indices.first = 2;
        indices.last = 3;
        sprite.index = usize::clamp(sprite.index, indices.first, indices.last);
        if move_delta.x < 0.0 {
            sprite.flip_x = true;
        } else if move_delta.x > 0.0 {
            sprite.flip_x = false;
        }
        timer.0.set_duration(Duration::from_millis(200));
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

pub fn animate_sprite(
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

use bevy::{prelude::*, transform};
use std::time::Duration;

use crate::{ImageAssets, MainCamera};

use super::{
    components::{AnimationIndices, AnimationTimer, Cursor, TargetLock},
    effects::{Flick, TimedRemoval},
};

pub fn game_keys(
    keyboard: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    cursor_pos: Query<&Transform, With<Cursor>>,
    mut commands: Commands,
    images: Res<ImageAssets>, // mut player: Query<(
                              //     &Cursor,
                              //     &mut Transform,
                              //     &mut AnimationIndices,
                              //     &mut TextureAtlasSprite,
                              //     &mut AnimationTimer,
                              // )>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let transform = cursor_pos.single();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: images.cursor.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: *transform,
                ..default()
            },
            TargetLock {},
            Flick {
                duration: Timer::from_seconds(3.0, TimerMode::Repeating),
                switch_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            TimedRemoval(Timer::from_seconds(3.0, TimerMode::Once)),
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

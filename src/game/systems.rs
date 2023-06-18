use bevy::{asset, math::vec2, prelude::*};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use std::time::Duration;

use crate::{ImageAssets, MainCamera};

use super::{
    components::{
        AnimationIndices, AnimationTimer, Direction, ExampleGameText, Paused, PausedText, Player,
        Pos, Vel,
    },
    effects::Flick,
};

pub fn pause_controls(
    keyboard: Res<Input<KeyCode>>,
    mut paused: ResMut<Paused>,
    mut pause_texts: Query<(&mut Visibility, With<PausedText>)>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        paused.0 = !paused.0;
    }

    if paused.is_changed() {
        for (mut vis, _) in pause_texts.iter_mut() {
            match paused.0 {
                false => *vis = Visibility::Hidden,
                true => *vis = Visibility::Inherited,
            }
        }
    }
}

pub fn game_keys(
    mut paused: ResMut<Paused>,
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<(
        &Player,
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

pub fn example_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let mut rng = RngComponent::from(&mut global_rng);
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "~In Game~",
            TextStyle {
                font: asset_server.load("fonts/visitor.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        Vel(vec2(1.0 + 0.5 * rng.f32(), 1.0 + 0.5 * rng.f32())),
        Pos(vec2(5.0, 15.0)),
        ExampleGameText,
        Flick {
            duration: Timer::from_seconds(60.0, TimerMode::Once),
            switch_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "Paused",
            TextStyle {
                font: asset_server.load("fonts/visitor.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            ..default()
        }),
        Vel(vec2(1.0, 1.0)),
        Pos(vec2(5.0, 15.0)),
        ExampleGameText,
        PausedText,
    ));
}

pub fn setup_player(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: images.cursor.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Player {},
    ));
}

pub fn move_cursor(
    mut cursor: Query<(&mut Transform, &Player)>,
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

pub fn teardown(mut commands: Commands, texts: Query<(Entity, With<ExampleGameText>)>) {
    for (entity, _) in texts.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn example_update(
    window: Query<&Window>,
    mut texts: Query<(
        &mut Style,
        &CalculatedSize,
        &mut Pos,
        &mut Vel,
        With<ExampleGameText>,
    )>,
) {
    let window = window.get_single().unwrap();
    for (mut style, calculated_size, mut pos, mut vel, _) in texts.iter_mut() {
        pos.0.y += vel.0.y;
        pos.0.x += vel.0.x;

        if pos.0.y + calculated_size.size.y > window.height() {
            pos.0.y = window.height() - calculated_size.size.y;
            vel.0.y *= -1.0;
        } else if pos.0.y < 0.0 {
            pos.0.y = 0.0;
            vel.0.y *= -1.0;
        }
        if pos.0.x + calculated_size.size.x > window.width() {
            pos.0.x = window.width() - calculated_size.size.x;
            vel.0.x *= -1.0;
        } else if pos.0.x < 0.0 {
            pos.0.x = 0.0;
            vel.0.x *= -1.0;
        }

        style.position = UiRect {
            top: Val::Px(pos.0.y),
            left: Val::Px(pos.0.x),
            ..default()
        };
    }
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

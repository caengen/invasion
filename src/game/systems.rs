use bevy::{math::vec2, prelude::*};
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;
use std::{f32::consts::E, time::Duration};

use crate::random::Random;

use super::{
    data::{
        AnimationIndices, AnimationTimer, Direction, ExampleGameText, Paused, PausedText, Player,
        Pos, Vel,
    },
    effects::Flick,
};

pub fn is_paused(paused: Res<Paused>) -> bool {
    paused.0
}

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
    mut rng: Local<Random>,
) {
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
        Vel(vec2(rng.gen_range(1.0..1.5), rng.gen_range(1.0..1.5))),
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

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_handle = asset_server.load("textures/chars/char_atlas.png");
    let idle_atlas =
        TextureAtlas::from_grid(idle_handle, Vec2 { x: 16.0, y: 16.0 }, 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(idle_atlas);
    let anim_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(anim_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        anim_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Player {},
    ));
}

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Size of the tile map in tiles.
    let map_size = TilemapSize { x: 32, y: 32 };

    // To create a map we use the TileStorage component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a Tilemap2dStorage
    // component per layer.
    let mut tile_storage = TileStorage::empty(map_size);

    // For the purposes of this example, we consider a tilemap with rectangular tiles.
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn_empty().id();

    // Spawn a 32 by 32 tilemap.
    // Alternatively, you can use helpers::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
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

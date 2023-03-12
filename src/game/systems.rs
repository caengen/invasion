use bevy::{math::vec2, prelude::*};
use rand::Rng;

use crate::random::Random;

use super::data::{ExampleGameText, Paused, PausedText, Pos, Vel};

pub fn is_not_paused(paused: Res<Paused>) -> bool {
    !paused.0
}

pub fn paused(paused: Res<Paused>, mut pause_texts: Query<(&mut Visibility, With<PausedText>)>) {
    if paused.is_changed() {
        for (mut vis, _) in pause_texts.iter_mut() {
            match paused.0 {
                false => *vis = Visibility::Hidden,
                true => *vis = Visibility::Inherited,
            }
        }
    }
}

pub fn game_keys(mut paused: ResMut<Paused>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::P) {
        paused.0 = !paused.0;
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut rng: Local<Random>) {
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
    for (mut style, calculatedSize, mut pos, mut vel, _) in texts.iter_mut() {
        pos.0.y += vel.0.y;
        pos.0.x += vel.0.x;

        if pos.0.y + calculatedSize.size.y > window.height() {
            pos.0.y = window.height() - calculatedSize.size.y;
            vel.0.y *= -1.0;
        } else if pos.0.y < 0.0 {
            pos.0.y = 0.0;
            vel.0.y *= -1.0;
        }
        if pos.0.x + calculatedSize.size.x > window.width() {
            pos.0.x = window.width() - calculatedSize.size.x;
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

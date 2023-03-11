use crate::AppState;
use bevy::{math::vec2, prelude::*};
use derive_more::From;

#[derive(Component)]
pub struct ExampleUpdateText;

#[derive(Debug, Component, From)]
pub struct Vel(Vec2);
#[derive(Debug, Component, From)]
pub struct Pos(Vec2);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update.in_set(OnUpdate(AppState::InGame)))
            .add_system(teardown.in_schedule(OnExit(AppState::InGame)));
    }
}

fn update(
    window: Query<&Window>,
    mut texts: Query<(
        &mut Style,
        &CalculatedSize,
        &mut Pos,
        &mut Vel,
        With<ExampleUpdateText>,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Vel(vec2(1.0, 1.0)),
        Pos(vec2(5.0, 15.0)),
        ExampleUpdateText,
    ));
}

fn teardown(mut commands: Commands, texts: Query<(Entity, With<ExampleUpdateText>)>) {
    for (entity, _) in texts.iter() {
        commands.entity(entity).despawn();
    }
}

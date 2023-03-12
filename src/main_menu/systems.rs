use bevy::prelude::*;

use crate::AppState;

use super::data::MainMenuText;

pub fn transition_to_game(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "GAME TITLE!",
            TextStyle {
                font: asset_server.load("fonts/visitor.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            margin: UiRect {
                top: Val::Px(32.0),
                right: Val::Auto,
                left: Val::Auto,
                ..default()
            },

            ..default()
        }),
        MainMenuText,
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "Press space to continue...",
            TextStyle {
                font: asset_server.load("fonts/visitor.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Percent(30.0),
                ..default()
            },
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..default()
            },
            ..default()
        }),
        MainMenuText,
    ));
}

pub fn teardown(mut commands: Commands, texts: Query<(Entity, With<MainMenuText>)>) {
    for (entity, _) in texts.iter() {
        commands.entity(entity).despawn();
    }
}

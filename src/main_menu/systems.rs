use bevy::prelude::*;

use crate::GameState;

use super::components::MainMenuText;

pub fn transition_to_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if keyboard.just_released(KeyCode::Space) {
        keyboard.clear_just_released(KeyCode::Space);
        next_state.set(GameState::EnterGame);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "INVASION",
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
            "Press [Space] to begin",
            TextStyle {
                font: asset_server.load("fonts/visitor.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(30.0),
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

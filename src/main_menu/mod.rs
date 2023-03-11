use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use crate::AppState;

#[derive(Component)]
pub struct MainMenuText;

/**
 * sett state for lukking
 * hello world fader ut
 */
pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(transition_to_game.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(teardown.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn transition_to_game(mut next_state: ResMut<NextState<AppState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn teardown(mut commands: Commands, texts: Query<(Entity, With<MainMenuText>)>) {
    for (entity, _) in texts.iter() {
        commands.entity(entity).despawn();
    }
}

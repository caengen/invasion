use bevy::prelude::*;
pub struct SplashPlugin;

/**
 * last font
 * hello world fader inn fra bunn
 * trykk space for å fortsette
 * key listener system for det
 * sett state for lukking
 * hello world fader ut
 * bytt fra state Splash til InGame
 */
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn(
        (
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "splash: hello game!",
                    TextStyle {
                        font: asset_server.load("fonts/visitor.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/visitor.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
        ),
    );
}

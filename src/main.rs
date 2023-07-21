use bevy::{
    asset::ChangeWatcher,
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::FrameTimeDiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
    window::PresentMode,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::{
    egui::{FontData, FontDefinitions, FontFamily},
    EguiContexts, EguiPlugin,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::RngPlugin;
use config::Debug;
use enter_stage::EnterStagePlugin;
use game::{prelude::*, GamePlugin};
use main_menu::*;
use std::{env, process, time::Duration};

mod config;
mod enter_stage;
mod game;
mod main_menu;

pub const SCREEN: Vec2 = Vec2::from_array([495.0, 270.0]);
pub const DARK: Color = Color::rgb(0.191, 0.184, 0.156);
pub const LIGHT: Color = Color::rgb(0.852, 0.844, 0.816);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 8, rows = 2))]
    #[asset(path = "textures/cursor.png")]
    pub cursor: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1))]
    #[asset(path = "textures/explosion.png")]
    pub explosion: Handle<TextureAtlas>,
    #[asset(path = "textures/heart-full.png")]
    pub heart_full: Handle<Image>,
    #[asset(path = "textures/heart-empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 4, rows = 1))]
    #[asset(path = "textures/cannon.png")]
    pub cannon: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 32.0, columns = 2, rows = 1))]
    #[asset(path = "textures/tank.png")]
    pub tank: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 495.0, tile_size_y = 32.0, columns = 1, rows = 1))]
    #[asset(path = "textures/ground.png")]
    pub ground: Handle<TextureAtlas>,
}

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    EnterStage,
    InGame,
    GameOver,
    LeaveStage,
}

/**
 * The configuration for the game loop. For cleanliness
 */
fn main() {
    // Possibility for program args
    let args: Vec<String> = env::args().skip(1).collect();
    let cfg = config::ProgramConfig::build(&args).unwrap_or_else(|err| {
        println!("A problem occured when parsing args: {err}");
        process::exit(1);
    });

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Invasion".into(),
                    resolution: (SCREEN.x, SCREEN.y).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: Some(ChangeWatcher {
                    delay: Duration::from_millis(200),
                }),
                ..Default::default()
            }),
    )
    .add_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::EnterStage),
    )
    .insert_resource(Debug(cfg.debug))
    .add_collection_to_loading_state::<_, ImageAssets>(GameState::AssetLoading)
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // .add_plugins(
    //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    // )
    .add_plugins((
        RngPlugin::new(), /* .with_rng_seed(220718) */
        JsonAssetPlugin::<Stage>::new(&["stage.json"]),
        EguiPlugin,
        MainMenuPlugin,
        EnterStagePlugin,
        GamePlugin,
    ))
    .add_systems(Startup, (setup_fonts, spawn_camera));

    app.run();
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(DARK),
            },
            ..default()
        },
        MainCamera,
    ));

    for mut window in windows.iter_mut() {
        window.cursor.visible = false;
    }
}

fn setup_fonts(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "visitor".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/visitor.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "visitor".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("visitor".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

use bevy::{
    prelude::{default, App, Color, Vec2},
    window::{PresentMode, WindowDescriptor},
};
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;
use random::{Random, RandomPlugin};
use std::{env, process};

mod config;
mod effects;
mod random;

pub const SCREEN: Vec2 = Vec2::from_array([1024.0, 512.0]);
pub const DARK: Color = Color::rgb(0.191, 0.184, 0.156);
pub const LIGHT: Color = Color::rgb(0.852, 0.844, 0.816);

fn main() {
    // Possibility for program args
    let args: Vec<String> = env::args().skip(1).collect();
    let cfg = config::ProgramConfig::build(&args).unwrap_or_else(|err| {
        println!("A problem occured when parsing args: {err}");
        process::exit(1);
    });

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "TITLE OF YOUR GAME".to_string(),
        present_mode: PresentMode::Fifo,
        width: SCREEN.x,
        height: SCREEN.y,
        ..default()
    });

    if cfg.debug {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

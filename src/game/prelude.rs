use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

use super::components::Foreground;

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Stage {
    pub name: String,
    pub bread: String,
    pub spawn_interval_secs: f32,
    pub split_interval_secs: f32,
    pub enemies_count: usize,
    pub missile_spawn_min: usize,
    pub missile_spawn_max: usize,
    pub missile_speed: f32,
    pub ufo_speed: f32,
    pub drop_bomb_chance: f64,
    pub ufo_chance: f64,
    pub split_chance: f64,
    pub max_split: u8,
    pub difficulty_rate: f32,
    pub text_cor: Vec<u8>,
    pub bg_cor: Vec<u8>,
    pub fg_cor: Vec<u8>,
    pub trail_cor: Vec<u8>,
}

#[derive(Resource)]
pub struct Wave(pub usize);

#[derive(Resource)]
pub struct StageHandle(pub Handle<Stage>);

pub fn stage_colors(
    mut foregrounds: Query<(&mut Sprite), (With<Foreground>)>,
    mut cameras: Query<(&mut Camera2d)>,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
) {
    let stage = stages.get(&stage.0);
    if let Some(stage) = stage {
        for mut sprite in foregrounds.iter_mut() {
            // why is this NOT WORKING?!
            sprite.color = Color::from(color_from_vec(&stage.fg_cor));
        }
        for mut camera in cameras.iter_mut() {
            camera.clear_color =
                ClearColorConfig::Custom(Color::from(color_from_vec(&stage.bg_cor)));
        }
    }
}

pub fn color_from_vec(color: &[u8]) -> Color {
    match color {
        [r, g, b] => Color::rgb(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0),
        _ => Color::rgb(1.0, 1.0, 1.0),
    }
}

#[derive(Resource)]
pub struct EnemySpawn(pub Timer);

#[derive(Resource)]
pub struct SplitTimer(pub Timer);

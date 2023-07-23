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
    spawn_interval_secs: f32,
    split_interval_secs: f32,
    enemies_count: usize,
    missile_spawn_min: usize,
    missile_spawn_max: usize,
    pub missile_speed: f32,
    ufo_speed: f32,
    drop_bomb_chance: f64,
    ufo_chance: f64,
    split_chance: f64,
    max_split: u8,
    difficulty_base: f32,
    difficulty_rate: f32,
    pub text_cor: Vec<u8>,
    pub bg_cor: Vec<u8>,
    pub fg_cor: Vec<u8>,
    pub trail_cor: Vec<u8>,
}

impl Stage {
    pub fn spawn_interval_secs(&self, wave: usize) -> f32 {
        self.spawn_interval_secs / (self.difficulty_base + wave as f32 * self.difficulty_rate)
    }
    pub fn split_interval_secs(&self, wave: usize) -> f32 {
        self.spawn_interval_secs / (self.difficulty_base + wave as f32 * self.difficulty_rate)
    }
    pub fn enemies_count(&self, wave: usize) -> usize {
        (self.enemies_count as f32 * (self.difficulty_base + wave as f32 * self.difficulty_rate))
            as usize
    }
    pub fn missile_spawn_min(&self, wave: usize) -> usize {
        (self.missile_spawn_min as f32 * (wave as f32 * self.difficulty_rate)) as usize
    }

    pub fn missile_spawn_max(&self, wave: usize) -> usize {
        self.missile_spawn_max + wave
    }

    pub fn ufo_speed(&self, wave: usize) -> f32 {
        self.ufo_speed + wave as f32 * self.difficulty_rate * 3.3
    }

    pub fn drop_bomb_chance(&self, wave: usize) -> f64 {
        self.drop_bomb_chance + ((wave as f32 / 10.0) * self.difficulty_rate) as f64
    }

    pub fn ufo_chance(&self, wave: usize) -> f64 {
        self.ufo_chance + ((wave as f32 / 20.0) + self.difficulty_rate) as f64
    }

    // increases by 1% per wave
    pub fn split_chance(&self, wave: usize) -> f64 {
        self.split_chance + 0.01 * wave as f64
    }

    // increases by 1 per 10 waves
    pub fn max_split(&self, wave: usize) -> u8 {
        self.max_split + (wave / 10) as u8
    }

    // 1. Fortsett med å konvertere til fns og start å bruke dem i game
    // 2. lage system viser current wave. kan kanskje være en timer som viser tekst og blokkerer spawning
    // Kan eg lage en fn -> bool som bare sjekker timer og ikke kj;rer visse systemer hvis true?
    // 3. lag overgang fra en wave til neste. Vise poeng kalkulering?
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

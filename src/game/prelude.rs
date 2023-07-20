use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Stage {
    pub name: String,
    pub bread: String,
    // spawn clock in seconds
    pub spawn_rate: f32,
    // Amount of enemies to spawn. 0 means infinite
    pub enemies_count: usize,
    pub missile_spawn_min: usize,
    pub missile_spawn_max: usize,
    pub missile_speed: f32,
    pub ufo_speed: f32,
    pub ufo_chance: f64,
    pub text_cor: Vec<u8>,
    pub bg_cor: Vec<u8>,
    pub fg_cor: Vec<u8>,
    pub trail_cor: Vec<u8>,
}

#[derive(Resource)]
pub struct StageHandle(pub Handle<Stage>);

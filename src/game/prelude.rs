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
    pub ufo_speed: f32,
    pub bg_cor: Vec<usize>,
    pub fg_cor: Vec<usize>,
    pub trail_cor: Vec<usize>,
}

#[derive(Resource)]
pub struct StageHandle(pub Handle<Stage>);

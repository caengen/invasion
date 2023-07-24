use std::time::Duration;

use crate::{
    game::prelude::{EnemySpawn, SplitTimer, Stage, StageHandle, Wave, WaveSpawnCount},
    GameState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, FontId, RichText},
    EguiContexts,
};

pub fn setup_stage(mut commands: Commands, asset_server: Res<AssetServer>) {
    let loaded_stage = asset_server.load("stages/1.stage.json");
    let stage = StageHandle(loaded_stage.clone());
    commands.insert_resource(stage);
}

pub fn setup_resources(
    mut commands: Commands,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
) {
    let stage = stages.get(&stage.0).unwrap();
    commands.insert_resource(Wave(0));
    commands.insert_resource(EnemySpawn(Timer::new(
        Duration::from_millis((stage.spawn_interval_secs(0) * 1000.0) as u64),
        TimerMode::Repeating,
    )));
    commands.insert_resource(SplitTimer(Timer::new(
        Duration::from_millis((stage.split_interval_secs(0) * 1000.0) as u64),
        TimerMode::Repeating,
    )));
    commands.insert_resource(WaveSpawnCount(0));
}

pub fn show_stage_intro(
    mut contexts: EguiContexts,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
) {
    if let Some(stage) = stages.get(&stage.0) {
        egui::Area::new("title")
            .anchor(Align2::CENTER_CENTER, egui::emath::vec2(0., -50.))
            .show(contexts.ctx_mut(), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(stage.name.clone())
                            .font(FontId::proportional(24.))
                            .color(Color32::WHITE),
                    );
                    ui.label(
                        RichText::new(stage.bread.clone())
                            .font(FontId::proportional(18.))
                            .color(Color32::WHITE),
                    );
                })
            });
    }
}

pub fn stage_intro_keys(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_released(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}
pub fn stage_intro_timer() {}
pub fn teardown_stage() {}

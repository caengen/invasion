use crate::{
    game::prelude::{Stage, StageHandle},
    GameState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, RichText},
    EguiContexts,
};

pub fn setup_stage(mut commands: Commands, asset_server: Res<AssetServer>) {
    let stage = StageHandle(asset_server.load("stages/a.stage.json"));
    commands.insert_resource(stage);
}

pub fn show_stage_intro(
    mut contexts: EguiContexts,
    stage: Res<StageHandle>,
    stages: Res<Assets<Stage>>,
) {
    if let Some(stage) = stages.get(&stage.0) {
        egui::Area::new("title")
            .anchor(Align2::CENTER_TOP, egui::emath::vec2(30., 0.))
            .show(contexts.ctx_mut(), |ui| ui.label(stage.name.clone()));
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

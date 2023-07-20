use crate::{
    game::prelude::{Stage, StageHandle},
    GameState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, FontId, RichText},
    EguiContexts,
};

pub fn setup_stage(mut commands: Commands, asset_server: Res<AssetServer>) {
    let stage = StageHandle(asset_server.load("stages/1.stage.json"));
    commands.insert_resource(stage);
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
